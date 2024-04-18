extern crate html_parser;

use gtk::prelude::*;
use html_parser::{Dom, Element, Node, Result};

fn parse_html_from_file() -> Result<(Node, Node)> {
    let html = std::fs::read_to_string("test/index.html")?;
    let dom = Dom::parse(&html)?;

    let head = find_element_by_name(&dom.children, "head").expect("Couldn't find head.");
    let body = find_element_by_name(&dom.children, "body").expect("Couldn't find body.");

    return Ok((head, body));
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

pub fn build_ui() -> Result<gtk::Box> {
    let html_view = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .halign(gtk::Align::Start)
        .css_name("htmlview")
        .build();

    let (_head, body) = parse_html_from_file()?;

    for element in body.element().unwrap().children.iter() {
        let element = element.element().unwrap();
        let contents = element.children.get(0);

        render_html(element, contents, html_view.clone(), false);
    }

    Ok(html_view)
}

fn render_html(element: &Element, contents: Option<&Node>, og_html_view: gtk::Box, recursive: bool) {
    let mut html_view = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .halign(gtk::Align::Start)
            .build();

    if !recursive {
        html_view = og_html_view;
    } else {
        println!("appended new div");
        og_html_view.append(&html_view);
    }
    
    println!("{:?}", element);

    match element.name.as_str() {
        "h1" | "h2" | "h3" | "h4" | "h5" | "h6" => {
            if let Some(text) = contents {
                match text {
                    Node::Text(t) => {
                        let label = gtk::Label::builder()
                            .label(t)
                            .css_name(element.name.as_str())
                            .css_classes(element.classes.clone())
                            .halign(gtk::Align::Start)
                            .build();
                        println!("appended 2");
                        html_view.append(&label);
                    },
                    Node::Element(el) => {
                        println!("sent it off");
                        render_html(el, el.children.get(0), html_view, true);
                    },
                    _ => {}
                }
            }
        }
        "p" => {
            let label_box = gtk::Box::builder()
                .orientation(gtk::Orientation::Horizontal)
                .build();

            for child in element.children.iter() {
                match child {
                    Node::Text(_) => {
                        let label = gtk::Label::builder()
                            .label(child.text().unwrap())
                            .css_name(element.name.as_str())
                            .css_classes(element.classes.clone())
                            .halign(gtk::Align::Start)
                            .build();
                        println!("appended 3: {}", child.text().unwrap());
                        label_box.append(&label);
                    }
                    Node::Element(el) => {
                        if el.name.as_str() == "a" {
                            let uri = el.attributes.get("href").unwrap().clone().unwrap();

                            let link_button = gtk::LinkButton::builder()
                                .label(el.children[0].text().unwrap())
                                .uri(uri)
                                .css_name("a")
                                .css_classes(el.classes.clone())
                                .build();
                            println!("appended 4");
                            label_box.append(&link_button);
                        } else {
                            render_html(el, el.children.get(0), html_view.clone(), true);
                        }
                    }
                    Node::Comment(_) => {}
                }
            }

            html_view.append(&label_box);
        }
        _ => {
            println!("Unsupported element: {:?}", element.name.as_str());
        }
    }
}
