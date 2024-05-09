extern crate html_parser;

use super::{
    css::{self, Styleable},
    lua,
};

use std::{cell::RefCell, rc::Rc, thread};

use gtk::{gdk_pixbuf, gio, glib::Bytes, prelude::*};
use html_parser::{Dom, Element, Node, Result};

use lua::Luable;


pub(crate) struct Tag {
    pub classes: Vec<String>,
    pub widget: Box<dyn Luable>,
    pub tied_variables: Vec<String>,
}

fn parse_html_from_file() -> Result<(Node, Node)> {
    let html: String = std::fs::read_to_string("test/index.html")?;
    let dom = Dom::parse(&html)?;

    let head = find_element_by_name(&dom.children, "head").expect("Couldn't find head.");
    let body = find_element_by_name(&dom.children, "body").expect("Couldn't find body.");

    css::load_css();

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
    let tags = Rc::new(RefCell::new(Vec::new()));

    css::load_css();

    let html_view = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .halign(gtk::Align::Fill)
        .hexpand(true)
        .valign(gtk::Align::Start)
        .spacing(6)
        .css_name("body")
        .build();

    html_view.style();

    let (head, body) = parse_html_from_file()?;

    for element in body.element().unwrap().children.iter() {
        if let Some(element) = element.element() {
            let contents = element.children.get(0);

            render_html(element, contents, html_view.clone(), false, tags.clone());
        }
    }

    for element in head.element().unwrap().children.iter() {
        if let Some(element) = element.element() {
            let contents = element.children.get(0);

            render_head(element, contents, html_view.clone());
        }
    }

    let tagss = Rc::clone(&tags);

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

    if let Err(e) = super::lua::run(tags) {
        println!("ERROR: Failed to run lua: {}", e);
    }

    Ok(html_view)
}

fn render_head(element: &Element, contents: Option<&Node>, html_view: gtk::Box) {
    match element.name.as_str() {
        "title" => {
            // set the current `Tab` name to this
        }
        "link" => {
            if let Some(href) = element.attributes.get("href") {
                if let Some(href) = href.as_ref() {
                    // got the href here
                }
            }
        }
        _ => {}
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
                            .wrap(true)
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
                            .label(child.text().unwrap())
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
                            let uri = el.attributes.get("href").unwrap().clone().unwrap();

                            let link_button = gtk::LinkButton::builder()
                                .label(el.children[0].text().unwrap())
                                .uri(uri)
                                .css_name("a")
                                .css_classes(el.classes.clone())
                                .build();

                            tags.borrow_mut().push(Tag {
                                classes: el.classes.clone(),
                                widget: Box::new(link_button.clone()),
                                tied_variables: Vec::new(),
                            });

                            css::perform_styling(element, &link_button);

                            label_box.append(&link_button);
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
                .css_name("hr")
                .css_classes(element.classes.clone())
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
            let url = element.attributes.get("src").unwrap().clone().unwrap();

            let handle = thread::spawn(move || {
                let result = reqwest::blocking::get(url).unwrap().bytes().unwrap();
                result
            });

            let img_data = handle.join().unwrap();

            let img_stream = gio::MemoryInputStream::from_bytes(&Bytes::from(&img_data));

            let stream =
                gdk_pixbuf::Pixbuf::from_stream(&img_stream, Some(&gio::Cancellable::new()))
                    .unwrap();

            let wrapper = gtk::Box::builder().build();

            let image = gtk::Picture::builder()
                .css_name("img")
                .alternative_text(element.attributes.get("alt").unwrap().clone().unwrap())
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
            let input_type = element
                .attributes
                .get("type")
                .unwrap()
                .clone()
                .unwrap_or_else(|| "text".to_string());

            if input_type == "text" {
                let entry = gtk::Entry::builder()
                    .placeholder_text(
                        element
                            .attributes
                            .get("placeholder")
                            .unwrap()
                            .clone()
                            .unwrap(),
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
                            strings.push(el.children[0].text().unwrap())
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
                .set_text(element.children[0].text().unwrap());

            html_view.append(&textview);
        }
        "button" => {
            let button = gtk::Button::builder()
                .label(element.children[0].text().unwrap())
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
                        .label(el.children[0].text().unwrap())
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
