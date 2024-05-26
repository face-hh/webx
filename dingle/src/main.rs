#[macro_use]
extern crate rocket;

mod utils;

use elipdotter::proximity::ProximateMap;
use elipdotter::*;
use html_parser::{Dom, Element, Node};
use index::{DocumentMap, Simple, SimpleOccurences};
use query::Query;
use regex::Regex;
use rocket::serde::{json::Json, Deserialize, Serialize};
use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
};
use utils::Website;

#[derive(Serialize)]
struct SearchResult {
    domain: String,
    rating: f32,
    title: String,
    description: String,
}

#[derive(Deserialize)]
struct SearchQuery {
    query: String,
}

#[derive(Debug, Clone)]
struct Ids {
    site: String,
    content: String,
    title: String,
    description: String,
}

async fn fetch_content(website: &Website) -> Result<(String, String, String), html_parser::Error> {
    let mut content = String::new();
    let mut title = String::new();
    let mut description = String::new();

    let url = utils::fetch_dns(website.clone().name, website.clone().tld).await;

    let html_content = utils::fetch_file(url + &"/index.html").await;

    let dom = Dom::parse(&html_content).unwrap();

    let head = find_element_by_name(&dom.children, "head").ok_or_else(|| {
        html_parser::Error::Parsing("Couldn't find head. Invalid HTML?".to_owned())
    })?;

    for element in head.element().unwrap().children.iter() {
        if let Some(element) = element.element() {
            let (meta_content, title_, description_) = render_head(element).await;

            title.push_str(&title_);
            description.push_str(&description_);
            content.push_str(&format!("{} ", meta_content));
        }
    }

    let body = find_element_by_name(&dom.children, "body").ok_or_else(|| {
        html_parser::Error::Parsing("Couldn't find head. Invalid HTML?".to_owned())
    })?;

    for element in body.element().unwrap().children.iter() {
        if let Some(element) = element.element() {
            let contents = element.children.first();
            let meta_content = render_body(element, contents);
            content.push_str(&format!("{} ", meta_content));
        }
    }

    let re = Regex::new(r"[^A-Za-z0-9\s]|\n|\r").unwrap();
    let res = re.replace_all(&content, "").into_owned();

    if res.is_empty() {
        return Err(html_parser::Error::Parsing("Empty content".to_owned()));
    }

    Ok((res.to_lowercase(), title, description))
}

async fn render_head(element: &Element) -> (String, String, String) {
    let mut content = String::new();
    let mut title = String::new();
    let mut description = String::new();

    match element.name.as_str() {
        "meta" => {
            if let Some(contents) = element.attributes.get("content") {
                if let Some(contents) = contents {
                    content.push_str(&format!("{} ", contents));

                    if let Some(desc) = element.attributes.get("name") {
                        if let Some(desc) = desc {
                            if desc == "description" {
                                description.push_str(&contents);
                            }
                        }
                    }
                }
            }
        }
        "title" => {
            if let Some(contents) = element.children.first() {
                title.push_str(contents.text().unwrap_or_default())
            }
        }
        _ => {
            println!("Unknown head element: {}", element.name);
        }
    }
    (content, title, description)
}

fn render_body(element: &Element, contents: Option<&Node>) -> String {
    let mut content = String::new();

    match element.name.as_str() {
        "p" | "h1" | "h2" | "h3" | "h4" | "h5" | "h6" => {
            if let Some(contents) = contents {
                content.push_str(&format!("{} ", contents.text().unwrap_or_default()));
            }
        }
        "ul" | "ol" => {
            for child in element.children.iter() {
                match child {
                    Node::Element(el) => {
                        if el.name.as_str() == "li" {
                            content.push_str(el.children[0].text().unwrap_or(""));
                        }
                    }
                    Node::Text(text) => {
                        content.push_str(&format!("\n{} ", text));
                    }
                    _ => {}
                }
            }
        }
        _ => {
            let children = &element.children;
            for child in children {
                if let Some(child_element) = child.element() {
                    if let Some(child) = child.element() {
                        let child_content = render_body(child_element, child.children.first());
                        content.push_str(&format!("{} ", child_content));
                    }
                }
            }
        }
    }
    content
}

fn find_element_by_name(elements: &Vec<Node>, name: &str) -> Option<Node> {
    for element in elements {
        if element.element()?.name == name {
            return Some(element.to_owned());
        }
        if let Some(child) = find_element_by_name(&element.element()?.children, name) {
            return Some(child);
        }
    }
    None
}

fn pq(s: &str) -> Query {
    match s.parse() {
        Ok(p) => p,
        Err(err) => {
            panic!("Failed to parse '{}', {:?}", s, err);
        }
    }
}

fn augment_simple<'a>(
    index: &'a Simple,
    map: &DocumentMap,
    proximate_map: &'a ProximateMap,
    ids: Vec<Ids>,
) -> SimpleOccurences<'a> {
    let mut occurences = SimpleOccurences::new(index, proximate_map);
    for id in ids {
        occurences.add_document(map.get_id(&id.site).unwrap(), Arc::new(id.content));
    }

    occurences
}

fn take<T>(vec: Vec<T>, index: usize) -> Option<T> {
    vec.into_iter().nth(index)
}

fn parse_int(input: &str) -> Option<u32> {
    input
        .chars()
        .find(|a| a.is_digit(10))
        .and_then(|a| a.to_digit(10))
}

fn query_and(
    query: String,
    map: &DocumentMap,
    index: &Simple,
    ids: &Vec<Ids>,
) -> Vec<SearchResult> {
    let q = pq(&query);
    let mut docs = q.documents(index);
    let proximate_map = docs.take_proximate_map();
    let occ_provider = augment_simple(index, map, &proximate_map, ids.clone().to_vec());
    let occurrences = q.occurrences(&occ_provider, 100).unwrap();

    let mut unique_domains: HashSet<String> = HashSet::new();

    let mut results = occurrences
        .map(|occ| {
            let id = format!("{:?}", occ.id());
            let id = parse_int(&id).unwrap_or_default();
            let id = take(ids.clone(), id as usize).unwrap();
            SearchResult {
                domain: id.site,
                rating: occ.rating(),
                title: id.title,
                description: id.description,
            }
        })
        .collect::<Vec<_>>();
    
    results.sort_by(|a, b| a.domain.cmp(&b.domain));
    results.dedup_by(|a, b| a.domain == b.domain);

    results.extend((0..10).filter_map(|i| match take(ids.clone(), i) {
        Some(id) => {
            if unique_domains.contains(&id.site) {
                None
            } else {
                unique_domains.insert(id.site.clone());
                Some(SearchResult {
                    domain: id.site,
                    rating: -999.0,
                    title: id.title,
                    description: id.description
                })
            }
        }
        None => None,
    }));

    results
}
async fn abc() -> (DocumentMap, Simple, Vec<Ids>) {
    let mut map = DocumentMap::new();
    let mut index = Simple::default();
    let mut ids: Vec<Ids> = Vec::new();
    let websites = utils::get_websites().await;

    for website in websites {
        let (content, title, description) = match fetch_content(&website).await {
            Ok((content, title, description)) => (content, title, description),
            Err(_) => continue,
        };

        let domain = website.name + "." + &website.tld;

        map.insert(&domain, &content, &mut index);
        ids.push(Ids {
            site: domain,
            content,
            title,
            description,
        });
    }

    println!("{:?}", ids);
    (map, index, ids)
}

#[post("/search", format = "json", data = "<query>")]
async fn search(
    query: Json<SearchQuery>,
    global_data: &rocket::State<Arc<Mutex<(DocumentMap, Simple, Vec<Ids>)>>>,
) -> Json<Vec<SearchResult>> {
    let data = global_data.lock().unwrap_or_else(|poisoned_data| {
        let guard = poisoned_data.into_inner();
        guard
    });

    let results = query_and(query.query.clone(), &data.0, &data.1, &data.2);
    Json(results)
}

#[tokio::main]
async fn main() {
    let global_data = Arc::new(Mutex::new((
        DocumentMap::new(),
        Simple::default(),
        Vec::new(),
    )));

    let a = global_data.clone();
    let aa = a.clone();

    let (map, index, ids) = abc().await;
    {
        let mut data = aa.lock().unwrap();
        *data = (map, index, ids);
    }

    rocket::build()
        .manage(global_data)
        .mount("/", routes![search])
        .launch()
        .await
        .unwrap();

    tokio::spawn(async move {
        tokio::time::sleep(std::time::Duration::from_secs(1800)).await;

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(std::time::Duration::from_secs(1800));
            loop {
                interval.tick().await;
                let (map, index, ids) = abc().await;
                let mut data = a.lock().unwrap();
                *data = (map, index, ids);
            }
        });
    });
}
