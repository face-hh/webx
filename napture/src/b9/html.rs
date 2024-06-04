extern crate html_parser;

use crate::{lualog, Tab, globals::LUA_TIMEOUTS};

use super::{
    css::{self, Styleable},
    lua,
};

use std::{cell::RefCell, fs, rc::Rc, thread};

use gtk::{gdk::Display, gdk_pixbuf, gio, glib::Bytes, prelude::*, CssProvider};
use html_parser::{Dom, Element, Node, Result};

use lua::Luable;
use url::Url;

pub(crate) struct Tag {
    pub classes: Vec<String>,
    pub widget: Box<dyn Luable>,
    pub tied_variables: Vec<String>,
}

fn decode_html_entities<T: AsRef<str>>(s: T) -> String {
    use html_escape::decode_html_entities;
    decode_html_entities(s.as_ref()).to_string()
}

async fn parse_html(mut url: String) -> Result<(Node, Node)> {
    let mut is_html = true;
    let mut file_name = String::new();

    if let Ok(mut uri) = Url::parse(&url) {
       let last_seg = {
           uri.path_segments()
               .map(|seg| seg.last().unwrap_or(""))
               .unwrap_or("").to_string()
       };
       if let Ok(mut segments) = uri.path_segments_mut() {
           if !last_seg.contains(".") {
               segments.pop_if_empty();
               segments.push("index.html");
           } else {
               if !last_seg.ends_with(".html") {
                   is_html = false;
               }
           }
           file_name += &last_seg;
       }
       url = uri.into();
    }

    let mut html = fetch_file(url).await;

    if !is_html {
        html = format!("<html><head>{}</head><body><p>{}</p></body></html>", file_name, html_escape::encode_double_quoted_attribute(&html));
    }

    let dom = match !html.is_empty() {
        true => Dom::parse(&html),
        false => Dom::parse(include_str!("../resources/not_found.html")),
    }?;

    let head = match find_element_by_name(&dom.children, "head") {
        Some(head) => head,
        None => {
            return Err(html_parser::Error::Parsing(
                "Couldn't find head. Invalid HTML?".to_owned(),
            ))
        }
    };

    let body = match find_element_by_name(&dom.children, "body") {
        Some(body) => body,
        None => {
            return Err(html_parser::Error::Parsing(
                "Couldn't find body. Invalid HTML?".to_owned(),
            ))
        }
    };

    Ok((head, body))
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
pub async fn build_ui(
    tab: Tab,
    previous_css_provider: Option<CssProvider>,
    scroll: Rc<RefCell<gtk::ScrolledWindow>>,
    searchbar: Rc<RefCell<gtk::SearchEntry>>,
) -> Result<(gtk::Box, CssProvider)> {
    let furl = tab.url.split("?").nth(0).unwrap_or(&tab.url).strip_suffix("/").unwrap_or(&tab.url);

    css::reset_css();

    {
        let mut timeouts = LUA_TIMEOUTS.lock().unwrap();
        for timeout in timeouts.drain(..) { 
            timeout.destroy();
        }
    }

    let tags = Rc::new(RefCell::new(Vec::new()));

    let html_view = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .halign(gtk::Align::Fill)
        .hexpand(true)
        .valign(gtk::Align::Start)
        .spacing(6)
        .css_name("body")
        .css_classes(vec!["body"])
        .build();

    let mut css: String = css::reset_css();
    
    let (head, body) = match parse_html(furl.to_string()).await {
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
            return Err(html_parser::Error::Parsing(
                "Failed to get head element!".to_string(),
            ));
        }
    };

    let body_elements = match body.element() {
        Some(ok) => ok,
        None => {
            eprintln!("FATAL: Couldn't get body element, aborting!");
            return Err(html_parser::Error::Parsing(
                "Failed to get body element!".to_string(),
            ));
        }
    };

    for element in head_elements.children.iter() {
        if let Some(element) = element.element() {
            let contents = element.children.first();
            let aa = &Rc::new(RefCell::new(&tab));

            let tabb = Rc::clone(aa);
            render_head(element, contents, tabb, &furl.to_string()).await;
        }
    }

    css.push_str(&html_view.style());

    for element in body_elements.children.iter() {
        if let Some(element) = element.element() {
            let contents = element.children.first();

            render_html(
                element,
                contents,
                html_view.clone(),
                false,
                tags.clone(),
                &mut css,
                scroll.clone(),
                previous_css_provider.clone(),
                searchbar.clone(),
                Rc::new(RefCell::new(tab.clone())),
            );
        }
    }

    if previous_css_provider.is_some() {
        gtk::style_context_remove_provider_for_display(
            &Display::default().unwrap(),
            &previous_css_provider.unwrap(),
        );
    }
    let provider = css::load_css_into_app(&css);

    let mut src = String::new();
    for element in head_elements.children.iter() {
        if let Some(element) = element.element() {
            if element.name == "script" {
                if let Some(Some(src_attr)) = element.attributes.get("src") {
                    src = src_attr.to_string();
                    break;
                }
            }
        }
    }

    let tagss = Rc::clone(&tags);

    if !src.is_empty() {
        let luacode = if src.starts_with("https://") {
            fetch_file(src).await
        } else {
            fetch_file(format!("{}/{}", furl, src)).await
        };

        if let Err(e) = super::lua::run(luacode, tags, tab.url.clone()).await {
            println!("ERROR: Failed to run lua: {}", e);
        }
    }

    for tag in tagss.borrow_mut().iter_mut() {
        let mut tied_variables = Vec::new();

        let text = tag.widget.get_contents_();

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

    Ok((html_view, provider))
}

async fn render_head(element: &Element, contents: Option<&Node>, tab: Rc<RefCell<&Tab>>, furl: &String) {
    match element.name.as_str() {
        "title" => {
            if let Some(contents) = contents {
                tab.borrow()
                    .label_widget
                    .set_label(&decode_html_entities(contents.text().unwrap_or("")))
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
                        // todo: a mutex would be better here, since this has to go through async
                        let css = fetch_file(format!("{}/{}", furl, href)).await;

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
    css: &mut String,
    scroll: Rc<RefCell<gtk::ScrolledWindow>>,
    previous_css_provider: Option<CssProvider>,
    searchbar: Rc<RefCell<gtk::SearchEntry>>,
    current_tab: Rc<RefCell<Tab>>,
) {
    let mut html_view = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(6)
        .css_name("htmlview")
        .css_classes(vec!["body"])
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

            css.push_str(&div_box.style());

            html_view.append(&div_box);

            for child in element.children.iter() {
                if let Node::Element(el) = child {
                    render_html(
                        el,
                        el.children.first(),
                        div_box.clone(),
                        true,
                        tags.clone(),
                        css,
                        scroll.clone(),
                        previous_css_provider.clone(),
                        searchbar.clone(),
                        current_tab.clone(),
                    );
                }
            }

            tags.borrow_mut().push(Tag {
                classes: element.classes.clone(),
                widget: Box::new(div_box),
                tied_variables: Vec::new(),
            });
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
                            .selectable(true)
                            .build();

                        css.push_str(&label.style());

                        html_view.append(&label);

                        tags.borrow_mut().push(Tag {
                            classes: element.classes.clone(),
                            widget: Box::new(label),
                            tied_variables: Vec::new(),
                        });
                    }
                    Node::Element(el) => {
                        render_html(
                            el,
                            el.children.first(),
                            html_view,
                            true,
                            tags.clone(),
                            css,
                            scroll.clone(),
                            previous_css_provider.clone(),
                            searchbar,
                            current_tab.clone(),
                        );
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

            if element.children.len() == 0 {
                render_p(&Node::Text(String::new()), element, &label_box, css, &tags);
            }

            for child in element.children.iter() {
                match child {
                    Node::Text(_) => {
                        render_p(child, element, &label_box, css, &tags);
                    }
                    Node::Element(el) => {
                        if el.name.as_str() == "a" {
                            render_a(
                                el,
                                label_box.clone(),
                                tags.clone(),
                                css,
                                scroll.clone(),
                                previous_css_provider.clone(),
                                searchbar.clone(),
                                current_tab.clone(),
                            );
                        } else {
                            render_html(
                                el,
                                el.children.first(),
                                html_view.clone(),
                                true,
                                tags.clone(),
                                css,
                                scroll.clone(),
                                previous_css_provider.clone(),
                                searchbar.clone(),
                                current_tab.clone(),
                            );
                        }
                    }
                    Node::Comment(_) => {}
                }
            }
        }
        "a" => {
            render_a(
                element,
                html_view,
                tags,
                css,
                scroll.clone(),
                previous_css_provider.clone(),
                searchbar.clone(),
                current_tab.clone(),
            );
        }
        "ul" | "ol" => {
            let list_box = gtk::Box::builder()
                .orientation(gtk::Orientation::Vertical)
                .css_name(element.name.as_str())
                .css_classes(element.classes.clone())
                .build();

            css.push_str(&list_box.style());

            html_view.append(&list_box);
            render_list(element, &list_box, &tags, css);

            tags.borrow_mut().push(Tag {
                classes: element.classes.clone(),
                widget: Box::new(list_box),
                tied_variables: Vec::new(),
            });
        }
        "hr" => {
            let line = gtk::Separator::builder()
                .orientation(gtk::Orientation::Horizontal)
                .build();

            css.push_str(&line.style());

            html_view.append(&line);

            tags.borrow_mut().push(Tag {
                classes: element.classes.clone(),
                widget: Box::new(line),
                tied_variables: Vec::new(),
            });
        }
        "img" => {
            let url = match element.attributes.get("src") {
                Some(Some(url)) => url.clone(),
                _ => {
                    println!("INFO: <img> tag must have a src attribute");
                    return;
                }
            };

            let stream = match fetch_image_to_pixbuf(url.clone()) {
                Ok(s) => s,
                Err(e) => {
                    println!("ERROR: Failed to load image: {}", e);
                    return;
                }
            };

            let wrapper = gtk::Box::builder().build();

            let image = gtk::Picture::builder()
                .css_name("img")
                .alternative_text(url)
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

            css.push_str(&image.style());

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

                css.push_str(&entry.style());

                html_view.append(&entry);

                tags.borrow_mut().push(Tag {
                    classes: element.classes.clone(),
                    widget: Box::new(entry),
                    tied_variables: Vec::new(),
                });
            }
        }
        "select" => {
            let mut strings = Vec::new();

            for child in element.children.iter() {
                if let Node::Element(el) = child {
                    if el.name.as_str() == "option" {
                        // TODO: keep track of value
                        if let Some(node) = el.children.first() {
                            strings.push(decode_html_entities(node.text().unwrap_or("")))
                        } else {
                            strings.push("".to_string())
                        }
                    }
                }
            }

            let dropdown = gtk::DropDown::builder()
                .model(&gtk::StringList::new(&strings.iter().map(|s| &**s).collect::<Vec<&str>>()))
                .css_name("select")
                .css_classes(element.classes.clone())
                .halign(gtk::Align::Start)
                .build();

            css.push_str(&dropdown.style());

            html_view.append(&dropdown);

            tags.borrow_mut().push(Tag {
                classes: element.classes.clone(),
                widget: Box::new(dropdown),
                tied_variables: Vec::new(),
            });
        }
        "textarea" => {
            let textview = gtk::TextView::builder()
                .editable(true)
                .css_name("textarea")
                .css_classes(element.classes.clone())
                .halign(gtk::Align::Start)
                .valign(gtk::Align::Start)
                .build();

            css.push_str(&textview.style());

            textview
                .buffer()
                .set_text(&decode_html_entities(element.children.first().unwrap_or(&Node::Text(String::new())).text().unwrap_or("")));

            html_view.append(&textview);

            tags.borrow_mut().push(Tag {
                classes: element.classes.clone(),
                widget: Box::new(textview),
                tied_variables: Vec::new(),
            });
        }
        "button" => {
            let button = gtk::Button::builder()
                .label(
                    &decode_html_entities(element
                        .children
                        .first()
                        .unwrap_or(&Node::Text("".to_owned()))
                        .text()
                        .unwrap_or("")),
                )
                .css_name("button")
                .css_classes(element.classes.clone())
                .halign(gtk::Align::Start)
                .valign(gtk::Align::Start)
                .build();

            css.push_str(&button.style());

            html_view.append(&button);

            tags.borrow_mut().push(Tag {
                classes: element.classes.clone(),
                widget: Box::new(button),
                tied_variables: Vec::new(),
            });
        }
        _ => {
            println!("INFO: Unknown element: {}", element.name);
        }
    }
}

fn render_a(
    el: &Element,
    label_box: gtk::Box,
    tags: Rc<RefCell<Vec<Tag>>>,
    css: &mut String,
    scroll: Rc<RefCell<gtk::ScrolledWindow>>,
    previous_css_provider: Option<CssProvider>,
    searchbar: Rc<RefCell<gtk::SearchEntry>>,
    current_tab: Rc<RefCell<Tab>>,
) {
    let uri = match el.attributes.get("href") {
        Some(Some(uri)) => uri.clone(),
        _ => {
            println!("INFO: <a> tag must have a href attribute");
            return;
        }
    };

    let link_button = gtk::LinkButton::builder()
        .label(&decode_html_entities(el.children.first().unwrap_or(&Node::Text(String::new())).text().unwrap_or("")))
        .uri(uri)
        .css_name("a")
        .css_classes(el.classes.clone())
        .halign(gtk::Align::Start)
        .build();

    let rc_css_prov = Rc::new(RefCell::new(
        previous_css_provider.unwrap_or(CssProvider::new()),
    ));

    link_button.connect_activate_link(move |btn| {
        let scroll = Rc::clone(&scroll);
        let css_prov = Rc::clone(&rc_css_prov);

        let current_tab = Rc::clone(&current_tab);
        let searchbar = Rc::clone(&searchbar);

        let uri = btn.uri();

        if !uri.starts_with("buss://") {
            return glib::Propagation::Proceed;
        }

        let uri = uri.replace("buss://", "");

        searchbar.borrow_mut().set_text(&uri);

        crate::handle_search_update(scroll, css_prov, current_tab, searchbar);

        glib::Propagation::Stop
    });

    css.push_str(&link_button.style());

    label_box.append(&link_button);

    tags.borrow_mut().push(Tag {
        classes: el.classes.clone(),
        widget: Box::new(link_button),
        tied_variables: Vec::new(),
    });
}

fn render_list(
    element: &Element,
    list_box: &gtk::Box,
    tags: &Rc<RefCell<Vec<Tag>>>,
    css: &mut String,
) {
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
                        .label(&decode_html_entities(el.children.first().unwrap_or(&Node::Text(String::new())).text().unwrap_or("")))
                        .css_name("li")
                        .css_classes(el.classes.clone())
                        .halign(gtk::Align::Start)
                        .selectable(true)
                        .build();

                    css.push_str(&label.style());

                    li.append(&lead);
                    li.append(&label);

                    list_box.append(&li);

                    tags.borrow_mut().push(Tag {
                        classes: el.classes.clone(),
                        widget: Box::new(label),
                        tied_variables: Vec::new(),
                    });
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

pub(crate) fn fetch_image_to_pixbuf(url: String) -> Result<gdk_pixbuf::Pixbuf> {
    let img_data: Vec<u8> = if url.starts_with("data:") {
        let (_, data) = url.split_once("data:").unwrap();
        let (mime, data) = data.split_once(",").unwrap_or(("", ""));
        let properties = mime.split(";").collect::<String>();

        if properties.contains("base64") {
            use base64::prelude::*;
            match BASE64_STANDARD.decode(data) {
                Ok(data) => data,
                Err(e) => {
                    lualog!("error", format!("invalid base64: {}", e));
                    Vec::new()
                }
            }
        } else {
            Vec::new()
        }
    } else {
        let handle = thread::spawn(move || {
            reqwest::blocking::get(url)
                .map_err(|e| e.to_string())
                .and_then(|res| res.bytes().map_err(|e| e.to_string()))
                .unwrap_or_else(|e| {
                    lualog!("error", format!("Failed to fetch image: {}", e));
                    Vec::new().into()
                })
        });

        match handle.join() {
            Ok(data) => data.to_vec(),
            Err(_) => {
                lualog!("error", "Failed to join fetch_image_to_pixbuf thread.");
                Vec::new()
            }
        }
    };

    let img_stream = gio::MemoryInputStream::from_bytes(&Bytes::from(&img_data));

    match gdk_pixbuf::Pixbuf::from_stream(&img_stream, Some(&gio::Cancellable::new())) {
        Ok(pixbuf) => Ok(pixbuf),
        Err(_) => Err(html_parser::Error::Parsing(
            "ERROR: Failed to load image".to_string(),
        )),
    }
}

async fn fetch_file(url: String) -> String {
    println!("Attempting to navigate to {url}...");

    if url.starts_with("file://") {
        let path = url
            .replace("file:///", "")
            .replace("file://", "");
        
        println!("{path}");

        match fs::read_to_string(&format!("{}", path)) {
            Ok(text) => text,
            Err(_) => {
                eprintln!("ERROR: Failed to read file: {}", path);
                String::new()
            }
        }
    } else if url.starts_with("https://github.com") {
        fetch_from_github(url).await
    } else if let Ok(response) = reqwest::get(url.clone()).await {
        let status = response.status();

        if let Ok(text) = response.text().await {
            text
        } else {
            lualog!(
                "error",
                format!(
                    "Failed to parse response body from URL (\"{}\"), status code: {}",
                    url, status
                )
            );
            String::new()
        }
    } else {
        lualog!(
            "error",
            format!(
                "Failed to fetch URL (\"{}\"). Perhaps no internet connection?",
                url
            )
        );
        String::new()
    }
}

async fn fetch_from_github(url: String) -> String {
    let client: reqwest::ClientBuilder = reqwest::Client::builder();

    let branch = if url.contains("tree") {
        url.split('/').nth(6).unwrap_or("main")
    } else { "main" };

    let path = (if url.contains("tree") {
        url.split('/').skip(7).collect::<Vec<&str>>()
    } else {
        url.split('/').skip(5).collect::<Vec<&str>>()
    }).join("/");

    let url = format!(
        "https://raw.githubusercontent.com/{}/{}/{}/{}",
        url.split('/').nth(3).unwrap_or(""),
        url.split('/').nth(4).unwrap_or(""),
        branch, path,
    );

    let client = match client.build() {
        Ok(client) => client,
        Err(e) => {
            lualog!(
                "error",
                format!(
                    "Couldn't build reqwest client, returning empty string: {}",
                    e
                )
            );
            return String::new();
        }
    };

    if let Ok(response) = client.get(&url).send().await {
        let status = response.status();

        if let Ok(json) = response.text().await {
            json
        } else {
            lualog!(
                "error",
                format!(
                    "Failed to parse response body from URL (\"{}\"), status code: {}",
                    url, status
                )
            );
            String::new()
        }
    } else {
        lualog!(
            "error",
            format!(
                "Failed to fetch URL (\"{}\"). Perhaps no internet connection?",
                url
            )
        );

        String::new()
    }
}

fn render_p(child: &Node, element: &Element, label_box: &gtk::Box, css: &mut String, tags: &Rc<RefCell<Vec<Tag>>>){
    let label = gtk::Label::builder()
        .label(&decode_html_entities(child.text().unwrap_or("")))
        .css_name(element.name.as_str())
        .css_classes(element.classes.clone())
        .halign(gtk::Align::Start)
        .selectable(true)
        .wrap(true)
        .build();

    label_box.append(&label);
    css.push_str(&label.style());

    tags.borrow_mut().push(Tag {
        classes: element.classes.clone(),
        widget: Box::new(label),
        tied_variables: Vec::new(),
    });
}
