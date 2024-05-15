mod b9;
mod parser;
mod imp;

use std::cell::RefCell;
use std::rc::Rc;

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends adw::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

use glib::Object;



use gtk::gio;
use serde::Deserialize;

use gtk::glib;
use gtk::prelude::*;

const APP_ID: &str = "org.bussin.napture";

#[derive(Clone, Debug)]
struct Tab {
    url: String,
    widget: gtk::Box,
    label_widget: gtk::Label,
    icon_widget: gtk::Image,
    // id: String,
}

fn main() -> glib::ExitCode {
    let args = Rc::new(RefCell::new(std::env::args().collect::<Vec<String>>()));

    let app = adw::Application::builder().application_id(APP_ID).build();

    app.connect_startup(|_| b9::css::load_css_into_app(include_str!("style.css")));
    app.connect_activate(move |app| {
        let args_clone = Rc::clone(&args);
        build_ui(app, args_clone)
    });

    app.run_with_args(&[""])
}

fn build_ui(app: &adw::Application, args: Rc<RefCell<Vec<String>>>) {
    let tabs: Vec<Tab> = vec![];

    let window: Window = Object::builder().property("application", app).build();

    // let cursor_pointer = Cursor::from_name("pointer", None);

    let search = gtk::SearchEntry::builder().width_request(500).build();
    let empty_label = gtk::Label::new(Some(""));
    let headerbar = gtk::HeaderBar::builder().build();

    let tabs_widget = gtk::Box::builder().css_name("tabs").spacing(6).build();

    let mut tab1 = make_tab(
        // tabs_widget.clone(),
        "New Tab",
        "file.png",
        // cursor_pointer.as_ref(),
        tabs.clone(),
    );

    let current_tab = tab1.clone();

    tabs_widget.append(&tab1.widget);
    tabs_widget.append(&search);

    headerbar.pack_start(&tabs_widget);
    headerbar.set_title_widget(Some(&empty_label));

    window.set_titlebar(Some(&headerbar));

    let scroll = gtk::ScrolledWindow::builder().build();

    let scroll_clone = scroll.clone();

    if let Some(dev_build) = args.borrow().get(1) {
        println!("{dev_build}");
        tab1.url = dev_build.to_string();

        if let Ok(htmlview) = b9::html::build_ui(tab1) {
            scroll_clone.set_child(Some(&htmlview));
        } else {
            println!("ERROR: HTML engine failed.");
        }
    }

    search.connect_activate(move |query| {
        let mut tab_in_closure = current_tab.clone();

        let url = query.text().to_string();
        let dns_url = fetch_dns(url.clone());

        tab_in_closure.url = dns_url;

        query.set_text(&url.replace("buss://", ""));
        query.set_position(-1);

        if let Some(root) = query.root() {
            root.set_focus(None as Option<&gtk::Widget>)
        } else {
            println!("ERROR: Failed to set focus on search bar. Root is None.");
        }

        match b9::html::build_ui(tab_in_closure.clone()) {
            Ok(htmlview) => {
                scroll_clone.set_child(Some(&htmlview));
            },
            Err(e) => {
                tab_in_closure.label_widget.set_label(&e.to_string());
                return;
            }
        };
    });

    scroll.set_vexpand(true);

    let nav = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(6)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    nav.append(&scroll);

    window.set_child(Some(&nav));
    window.present();
}

// commented code here was an attempt at implementing multiple tabs.
// it will be kept here in case I decide to implement multiple tabs again
fn make_tab(
    // tabs_widget: gtk::Box,
    label: &str,
    icon: &str,
    // cursor_pointer: Option<&Cursor>,
    mut tabs: Vec<Tab>,
) -> Tab {
    // let tabid = gen_tab_id();

    let tab = gtk::Box::builder()
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .spacing(6)
        .css_name("tab")
        // .css_classes(vec![tabid.clone()])
        .build();

    // let tabs_widgett = Rc::new(RefCell::new(tabs_widget));
    // let tabb = Rc::new(RefCell::new(tab.clone()));
    // let tabss = Rc::new(RefCell::new(tabs.clone()));

    // x.connect_clicked(move |_| {
    //     let tabs_widgett = Rc::clone(&tabs_widgett);
    //     let tabbb = Rc::clone(&tabb);
    //     let tabsss = Rc::clone(&tabss);

    //     remove_tab(tabbb, tabs_widgett, &mut tabsss.borrow_mut());
    // });

    let tabicon = gtk::Image::from_file(icon);

    let tabname = gtk::Label::builder()
        .css_name("tab-label")
        .label(label)
        .build();

    tab.append(&tabicon);
    tab.append(&tabname);

    let res = Tab {
        url: "".to_string(),
        widget: tab,
        // id: tabid,
        label_widget: tabname,
        icon_widget: tabicon,
    };

    tabs.push(res.clone());

    res
}

// fn remove_tab(tab: Rc<RefCell<gtk::Box>>, tabs_widget: Rc<RefCell<gtk::Box>>, tabs: &mut Vec<Tab>) {
//     tabs_widget.borrow_mut().remove(&tab.borrow().clone());

//     tabs.retain(|potential_tab| tab.borrow().css_classes()[0] != potential_tab.id);
// }

// fn gen_tab_id() -> String {
//     use uuid::Uuid;

//     Uuid::new_v4().to_string()
// }

#[derive(Deserialize)]
struct DomainInfo {
    ip: String,
}

fn fetch_dns(url: String) -> String {
    let url = url.replace("buss://", "");

    let client: reqwest::blocking::ClientBuilder = reqwest::blocking::Client::builder();

    let url = format!(
        "https://api.buss.lol/domain/{}/{}",
        url.split('.').nth(0).unwrap_or(""),
        url.split('.').nth(1).unwrap_or(""),
    );

    let client = match client.build() {
        Ok(client) => client,
        Err(e) => {
            eprintln!(
                "ERROR: Couldn't build reqwest client, returning empty string: {}",
                e
            );
            return String::new();
        }
    };

    if let Ok(response) = client.get(&url).send() {
        if let Ok(json) = response.json::<DomainInfo>() {
            json.ip
        } else {
            // TODO: error report
            String::new()
        }
    } else {
        // TODO: error report
        String::new()
    }
}
