extern crate html_parser;

use crate::Tab;

use super::{
    css::{self, Styleable},
    lua,
};

use std::{cell::RefCell, fs, rc::Rc, thread};

use gtk::{gdk_pixbuf, gio, glib::Bytes, prelude::*};
use html_parser::{Dom, Element, Node, Result};

use lua::Luable;

pub(crate) struct Tag {
    pub classes: Vec<String>,
    pub widget: Box<dyn Luable>,
    pub tied_variables: Vec<String>,
}

async fn parse_html(url: String) -> Result<(Node, Node)> {
    let html: String = fetch_file(url + &"/index.html").await;

    let dom = match !html.is_empty() {
        true => Dom::parse(&html),
        false => Dom::parse(&include_str!("../resources/not_found.html")),
    }?;

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

#[tokio::main]
pub async fn build_ui(tab: Tab) -> Result<gtk::Box> {
    css::reset_css();

    let tags = Rc::new(RefCell::new(Vec::new()));

    let html_view = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .halign(gtk::Align::Fill)
        .hexpand(true)
        .valign(gtk::Align::Start)
        .spacing(6)
        .css_name("body")
        .build();

    let (head, body) = match parse_html(tab.url.clone()).await {
        Ok(ok) => ok,
        Err(e) => {
            eprintln!("Couldn't parse HTML: {}", e);
            return Err(html_parser::Error::Parsing(e.to_string()));
        }
    };

    let head_elements = match head.element() {
        Some(ok) => ok,
        None => {
            eprintln!("FATAL: Couldn't get head element, aborting!");
            return Err(html_parser::Error::Parsing("Failed to get head element!".to_string()));
        }
    };

    let body_elements = match body.element() {
        Some(ok) => ok,
        None => {
            eprintln!("FATAL: Couldn't get body element, aborting!");
            return Err(html_parser::Error::Parsing("Failed to get body element!".to_string()));
        }
    };

    for element in head_elements.children.iter() {
        if let Some(element) = element.element() {
            let contents = element.children.get(0);
            let aa = &Rc::new(RefCell::new(&tab));

            let tabb = Rc::clone(aa);
            render_head(element, contents, tabb).await;
        }
    }

    html_view.style();

    for element in body_elements.children.iter() {
        if let Some(element) = element.element() {
            let contents = element.children.get(0);

            render_html(element, contents, html_view.clone(), false, tags.clone());
        }
    }

    let mut src = String::new();
    for element in head_elements.children.iter() {
        if let Some(element) = element.element() {
            if element.name == "script" {
                if let Some(src_attr) = element.attributes.get("src") {
                    if let Some(src_attr) = src_attr {
                        src = src_attr.to_string();
                        break;
                    }
                }
            }
        }
    }

    let tagss = Rc::clone(&tags);

    if !src.is_empty() {
        let luacode = fetch_file(tab.url.clone() + "/" + &src).await;

        if let Err(e) = super::lua::run(luacode, tags).await {
            println!("ERROR: Failed to run lua: {}", e);
        }
    }

    for tag in tagss.borrow_mut().iter_mut() {
        let mut tied_variables = Vec::new();

        let text = tag.widget.get_contents();

        let mut inside = false;
        let mut var = String::new();

        for c in text.chars() {
            if c == '{' {
                inside = true;
            } else if c == '}' {
                inside = false;

                tied_variables.push(var.trim().to_string());
                var.clear();
            } else if inside {
                var.push(c);
            }
        }

        tag.tied_variables = tied_variables;
    }

    Ok(html_view)
}

async fn render_head(element: &Element, contents: Option<&Node>, tab: Rc<RefCell<&Tab>>) {
    match element.name.as_str() {
        "title" => {
            if let Some(contents) = contents {
                tab.borrow()
                    .label_widget
                    .set_label(contents.text().unwrap_or(""))
            }
        }
        "link" => {
            if let Some(href) = element.attributes.get("href") {
                if let Some(href) = href.as_ref() {
                    if href.ends_with(".png") || href.ends_with(".jpg") {
                        let result = fetch_image_to_pixbuf(href.clone());

                        if let Ok(stream) = result {
                            tab.borrow()
                                .icon_widget
                                .set_paintable(Some(&gtk::gdk::Texture::for_pixbuf(&stream)));
                        } else {
                            println!("WARNING: Failed to fetch image: {}", result.unwrap_err());
                        }
                    } else {
                        let css = fetch_file(tab.borrow().url.clone() + "/" + href).await;

                        css::load_css(css);
                    }
                }
            }
        }
        "script" => {
            // this is handled later on so that Lua runs after the DOM is rendered
        }
        _ => {
            println!("Unknown head element: {}", element.name);
        }
    }
}

fn render_html(
    element: &Element,
    contents: Option<&Node>,
    og_html_view: gtk::Box,
    recursive: bool,
    tags: Rc<RefCell<Vec<Tag>>>,
) {
    let mut html_view = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(6)
        .build();

    if !recursive {
        html_view = og_html_view.clone();
    } else {
        og_html_view.append(&html_view);
    }

    match element.name.as_str() {
        "div" => {
            let div_box = gtk::Box::builder()
                .orientation(gtk::Orientation::Horizontal)
                .css_name("div")
                .css_classes(element.classes.clone())
                .build();

            div_box.style();

            tags.borrow_mut().push(Tag {
                classes: element.classes.clone(),
                widget: Box::new(div_box.clone()),
                tied_variables: Vec::new(),
            });
            html_view.append(&div_box);

            for child in element.children.iter() {
                match child {
                    Node::Element(el) => {
                        render_html(el, el.children.get(0), div_box.clone(), true, tags.clone());
                    }
                    _ => {}
                }
            }
        }
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

                        css::perform_styling(element, &label);
                        tags.borrow_mut().push(Tag {
                            classes: element.classes.clone(),
                            widget: Box::new(label.clone()),
                            tied_variables: Vec::new(),
                        });

                        html_view.append(&label);
                    }
                    Node::Element(el) => {
                        render_html(el, el.children.get(0), html_view, true, tags.clone());
                    }
                    _ => {}
                }
            }
        }
        "p" => {
            let label_box = gtk::Box::builder()
                .orientation(gtk::Orientation::Horizontal)
                .build();

            html_view.append(&label_box);

            for child in element.children.iter() {
                match child {
                    Node::Text(_) => {
                        let label = gtk::Label::builder()
                            .label(child.text().unwrap_or(""))
                            .css_name(element.name.as_str())
                            .css_classes(element.classes.clone())
                            .halign(gtk::Align::Start)
                            .wrap(true)
                            .build();
                        tags.borrow_mut().push(Tag {
                            classes: element.classes.clone(),
                            widget: Box::new(label.clone()),
                            tied_variables: Vec::new(),
                        });

                        css::perform_styling(element, &label);

                        label_box.append(&label);
                    }
                    Node::Element(el) => {
                        if el.name.as_str() == "a" {
                            render_a(el, element, label_box.clone(), tags.clone());
                        } else {
                            render_html(
                                el,
                                el.children.get(0),
                                html_view.clone(),
                                true,
                                tags.clone(),
                            );
                        }
                    }
                    Node::Comment(_) => {}
                }
            }
        }
        "a" => {
            render_a(element, element, html_view.clone(), tags.clone());
        }
        "ul" | "ol" => {
            let list_box = gtk::Box::builder()
                .orientation(gtk::Orientation::Vertical)
                .css_name(element.name.as_str())
                .build();
            tags.borrow_mut().push(Tag {
                classes: element.classes.clone(),
                widget: Box::new(list_box.clone()),
                tied_variables: Vec::new(),
            });

            css::perform_styling(element, &list_box);

            html_view.append(&list_box);

            render_list(element, list_box, tags);
        }
        "hr" => {
            let line = gtk::Separator::builder()
                .orientation(gtk::Orientation::Horizontal)
                .build();

            tags.borrow_mut().push(Tag {
                classes: element.classes.clone(),
                widget: Box::new(line.clone()),
                tied_variables: Vec::new(),
            });

            css::perform_styling(element, &line);

            html_view.append(&line);
        }
        "img" => {
            let url = match element.attributes.get("src") {
                Some(Some(url)) => url.clone(),
                _ => {
                    println!("INFO: <img> tag must have a src attribute");
                    return;
                }
            };

            let stream = match fetch_image_to_pixbuf(url) {
                Ok(s) => s,
                Err(e) => {
                    println!("ERROR: Failed to load image: {}", e);
                    return;
                }
            };

            let wrapper = gtk::Box::builder().build();

            let image = gtk::Picture::builder()
                .css_name("img")
                .alternative_text(
                    element
                        .attributes
                        .get("alt")
                        .unwrap_or(&Some(String::new()))
                        .clone()
                        .unwrap_or_else(|| "".to_string())
                )
                .css_classes(element.classes.clone())
                .halign(gtk::Align::Start)
                .valign(gtk::Align::Start)
                .can_shrink(false)
                .build();
            tags.borrow_mut().push(Tag {
                classes: element.classes.clone(),
                widget: Box::new(image.clone()),
                tied_variables: Vec::new(),
            });

            css::perform_styling(element, &image);

            image.set_paintable(Some(&gtk::gdk::Texture::for_pixbuf(&stream)));
            // weird workaround - https://discourse.gnome.org/t/can-shrink-on-picture-creates-empty-occupied-space/20547/2
            wrapper.append(&image);
            html_view.append(&wrapper);
        }
        "input" => {
            let input_type = match element.attributes.get("type") {
                Some(Some(t)) => t.to_string(),
                _ => "text".to_string(),
            };

            if input_type == "text" {
                let entry = gtk::Entry::builder()
                    .placeholder_text(
                        element
                            .attributes
                            .get("placeholder")
                            .unwrap_or(&Some(String::new()))
                            .clone()
                            .unwrap_or("".to_string()),
                    )
                    .css_name("input")
                    .css_classes(element.classes.clone())
                    .halign(gtk::Align::Start)
                    .build();

                tags.borrow_mut().push(Tag {
                    classes: element.classes.clone(),
                    widget: Box::new(entry.clone()),
                    tied_variables: Vec::new(),
                });

                css::perform_styling(element, &entry);

                html_view.append(&entry);
            }
        }
        "select" => {
            let mut strings = Vec::new();

            for child in element.children.iter() {
                match child {
                    Node::Element(el) => {
                        if el.name.as_str() == "option" {
                            // TODO: keep track of value
                            strings.push(el.children[0].text().unwrap_or(""))
                        }
                    }
                    _ => {}
                }
            }

            let dropdown = gtk::DropDown::builder()
                .model(&gtk::StringList::new(&strings[..]))
                .css_name("select")
                .css_classes(element.classes.clone())
                .halign(gtk::Align::Start)
                .build();
            tags.borrow_mut().push(Tag {
                classes: element.classes.clone(),
                widget: Box::new(dropdown.clone()),
                tied_variables: Vec::new(),
            });

            css::perform_styling(element, &dropdown);

            html_view.append(&dropdown);
        }
        "textarea" => {
            let textview = gtk::TextView::builder()
                .editable(true)
                .css_name("textarea")
                .css_classes(element.classes.clone())
                .halign(gtk::Align::Start)
                .valign(gtk::Align::Start)
                .build();
            tags.borrow_mut().push(Tag {
                classes: element.classes.clone(),
                widget: Box::new(textview.clone()),
                tied_variables: Vec::new(),
            });

            css::perform_styling(element, &textview);

            textview
                .buffer()
                .set_text(element.children[0].text().unwrap_or(""));

            html_view.append(&textview);
        }
        "button" => {
            let button = gtk::Button::builder()
                .label(element.children[0].text().unwrap_or(""))
                .css_name("button")
                .css_classes(element.classes.clone())
                .halign(gtk::Align::Start)
                .valign(gtk::Align::Start)
                .build();
            tags.borrow_mut().push(Tag {
                classes: element.classes.clone(),
                widget: Box::new(button.clone()),
                tied_variables: Vec::new(),
            });

            css::perform_styling(element, &button);

            html_view.append(&button);
        }
        _ => {
            println!("INFO: Unknown element: {}", element.name);
        }
    }
}

fn render_a(el: &Element, element: &Element, label_box: gtk::Box, tags: Rc<RefCell<Vec<Tag>>>) {
    let uri = match el.attributes.get("href") {
        Some(Some(uri)) => uri.clone(),
        _ => {
            println!("INFO: <a> tag must have a href attribute");
            return;
        }
    };

    let link_button = gtk::LinkButton::builder()
        .label(el.children[0].text().unwrap_or(""))
        .uri(uri)
        .css_name("a")
        .css_classes(el.classes.clone())
        .halign(gtk::Align::Start)
        .build();

    tags.borrow_mut().push(Tag {
        classes: el.classes.clone(),
        widget: Box::new(link_button.clone()),
        tied_variables: Vec::new(),
    });

    css::perform_styling(element, &link_button);

    label_box.append(&link_button);
}

fn render_list(element: &Element, list_box: gtk::Box, tags: Rc<RefCell<Vec<Tag>>>) {
    for (i, child) in element.children.iter().enumerate() {
        match child {
            Node::Element(el) => {
                if el.name.as_str() == "li" {
                    let li = gtk::Box::builder().build();

                    let lead = gtk::Label::builder()
                        .label(match element.name.as_str() {
                            "ul" => "\t• ".to_string(),
                            "ol" => format!("\t{}. ", i + 1),
                            _ => panic!("Unknown list type"),
                        })
                        .css_name("li")
                        .css_classes(vec!["lead"])
                        .halign(gtk::Align::Start)
                        .build();

                    let label = gtk::Label::builder()
                        .label(el.children[0].text().unwrap_or(""))
                        .css_name("li")
                        .css_classes(el.classes.clone())
                        .halign(gtk::Align::Start)
                        .build();

                    tags.borrow_mut().push(Tag {
                        classes: el.classes.clone(),
                        widget: Box::new(label.clone()),
                        tied_variables: Vec::new(),
                    });
                    css::perform_styling(element, &label);

                    li.append(&lead);
                    li.append(&label);

                    list_box.append(&li);
                } else {
                    println!("INFO: Expected li inside ul/ol, instead got: {:?}", child);
                }
            }
            _ => {
                println!("INFO: Not an element: {:?}", child);
            }
        }
    }
}

fn fetch_image_to_pixbuf(url: String) -> Result<gdk_pixbuf::Pixbuf> {
    let handle = thread::spawn(move || {
        // TODO: erorr handling
        let result = reqwest::blocking::get(url)
            .map_err(|e| e.to_string())
            .and_then(|res| res.bytes().map_err(|e| e.to_string()))
            .unwrap_or_else(|e| {
                println!("ERROR: Failed to fetch image: {}", e);
                Vec::new().into()
            });
        result
    });

    let img_data = match handle.join() {
        Ok(data) => data,
        Err(_) => {
            println!("ERROR: Failed to join fetch_image_to_pixbuf thread.");
            Vec::new().into()
        }
    };

    let img_stream = gio::MemoryInputStream::from_bytes(&Bytes::from(&img_data));

    match gdk_pixbuf::Pixbuf::from_stream(&img_stream, Some(&gio::Cancellable::new())) {
        Ok(pixbuf) => Ok(pixbuf),
        Err(_) => {
            Err(html_parser::Error::Parsing("ERROR: Failed to load image".to_string()))
        }
    }

    
}

async fn fetch_file(url: String) -> String {
    if url.starts_with("file://") {
        let path = url.replace("file://", "");

        match fs::read_to_string(path) {
            Ok(text) => text,
            Err(_) => {
                eprintln!("ERROR: Failed to read file: {}", url);
                String::new()
            }
        }
    } else if url.starts_with("https://github.com") {
        fetch_from_github(url).await
    } else {

        if let Ok(response) = reqwest::get(url).await {
            if let Ok(text) = response.text().await {
                text
            } else {
                // TODO: error report
                String::new()
            }
        } else {
            // TODO: error report
            String::new()
        }
    }
}

async fn fetch_from_github(url: String) -> String {
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
            eprintln!("ERROR: Couldn't build reqwest client, returning empty string: {}", e);
            return String::new();
        }
    };

    if let Ok(response) = client.get(&url).send().await {
        if let Ok(json) = response.text().await {
            json
        } else {
            // TODO: error report
            String::new()
        }
    } else {
        // TODO: error report
        String::new()
    }
}
