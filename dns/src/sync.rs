// fetch GET /domains from the config.dns.sync_from_list
// and add them to the database if they don't exist

use crate::config::Config;
use crate::http::Domain;
use macros_rs::fmt::{crashln, string};
use mongodb::{bson::doc, options::FindOptions};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Domains {
    domains: Vec<Domain>,
    page: u32,
    limit: u32,
}

pub async fn sync() -> std::io::Result<()> {
    let config_path = ("config.toml".to_string());

    let config = Config::new().set_path(&config_path).read();
    let mut sync_list = config.dns.sync_from_list.clone();
    sync_list.sort();

    let db = match config.connect_to_mongo().await {
        Ok(client) => client,
        Err(err) => crashln!("Failed to connect to MongoDB.\n{}", string!(err)),
    };

    // fetch GET /domains for each sync_list item
    // and add them to the database if they don't exist
    println!("syncing domains from {:?}", sync_list);
    // {"domains":[{"tld":"btw","ip":"https://github.com/Smartlinuxcoder/iusearch.btw","name":"iusearch"},{"tld":"rizz","ip":"https://github.com/illy-dev/website-for-bussin-web","name":"skibidi"}],"page":1,"limit":2}
    for item in sync_list {
        let mut page = 1;
        let mut domains: Vec<Domain> = vec![];

        loop {
            let url = format!("https://{}/domains?limit=100&page={}", item, page);
            println!("fetching {}", url);
            let res = reqwest::get(&url).await.unwrap();
            println!("fetched");
            let body = res.text().await.unwrap();
            println!("{}", body);
            //parts domains, page, limit
            let domains = serde_json::from_str::<Domains>(&body).unwrap();
            let len = &domains.domains.len();
            for domain in domains.domains {
                if !db
                    .find_one(doc! { "name": &domain.name, "tld": &domain.tld }, None)
                    .await
                    .unwrap()
                    .is_some()
                {
                    db.insert_one(&domain, None).await.unwrap();
                }
            }
            page += 1;
            if page > 100 {
                break;
            } else if *len == 0 {
                break;
            }
        }
    }
    Ok(())
}
