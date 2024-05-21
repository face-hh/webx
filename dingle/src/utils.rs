use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Website {
    pub tld: String,
    pub name: String,
    pub ip: String,
}

#[derive(Deserialize)]
struct DomainInfo {
    ip: String,
}

pub async fn get_websites() -> Vec<Website> {
    if let Ok(response) = reqwest::get("https://api.buss.lol/domains").await {
        if let Ok(json) = response.json::<Vec<Website>>().await {
            json
        } else {
            panic!("Shit")
        }
    } else {
        panic!("Fuck")
    }
}

pub async fn fetch_dns(domain: String, tld: String) -> String {
    let client: reqwest::ClientBuilder = reqwest::Client::builder();

    let clienturl = format!(
        "https://api.buss.lol/domain/{}/{}",
        domain,
        tld,
    );

    println!("{}", clienturl);

    let client = match client.build() {
        Ok(client) => client,
        Err(e) => {
            eprintln!("ERROR: Couldn't build reqwest client: {}", e);
            panic!()
        }
    };

    if let Ok(response) = client.get(clienturl).send().await {
        let status = response.status();

        if let Ok(json) = response.json::<DomainInfo>().await {
            json.ip
        } else {
            println!("Failed to parse response body from DNS API. Error code: {}. Returning original URL.", status.as_u16());
            String::new()
        }
    } else {
        println!("Failed to send HTTP request to DNS API. Returning original URL.");
        String::new()
    }
}

pub async fn fetch_file(url: String) -> String {
    if url.starts_with("https://github.com") {
        fetch_from_github(url).await
    } else if let Ok(response) = reqwest::get(url.clone()).await {
        let status = response.status();

        if let Ok(text) = response.text().await {
            text
        } else {
            println!(
                "Failed to parse response body from URL (\"{}\"), status code: {}",
                url, status
            );
            String::new()
        }
    } else {
        println!(
            "Failed to fetch URL (\"{}\"). Perhaps no internet connection?",
            url
        );
        String::new()
    }
}

pub async fn fetch_from_github(url: String) -> String {
    let client: reqwest::ClientBuilder = reqwest::Client::builder();

    let url = format!(
        "https://raw.githubusercontent.com/{}/{}/main/{}",
        url.split('/').nth(3).unwrap_or(""),
        url.split('/').nth(4).unwrap_or(""),
        url.split('/').last().unwrap_or(""),
    );

    let client = match client.build() {
        Ok(client) => client,
        Err(e) => {
            println!(
                "Couldn't build reqwest client, returning empty string: {}",
                e
            );
            return String::new();
        }
    };

    if let Ok(response) = client.get(&url).send().await {
        let status = response.status();

        if let Ok(json) = response.text().await {
            json
        } else {
            println!(
                "Failed to parse response body from URL (\"{}\"), status code: {}",
                url, status
            );
            String::new()
        }
    } else {
        println!(
            "Failed to fetch URL (\"{}\"). Perhaps no internet connection?",
            url
        );

        String::new()
    }
}
