// #![windows_subsystem = "windows"]
mod b9;
mod globals;
mod imp;
mod parser;

#[macro_export]
macro_rules! lualog {
    ($type:expr, $s:expr) => {{
        let problem_type = match ($type) {
            "error" => "<span foreground=\"#ff3333\">ERROR:</span> ",
            "warning" => "<span foreground=\"#ffcc00\">WARNING</span>: ",
            "debug" => "<span foreground=\"#7bbcb6\">DEBUG</span>: ",
            "lua" => "<span foreground=\"#a89332\">LUA</span>: ",
            _ => "",
        };

        let now = chrono::Local::now();
        let log_msg = format!(
            "<span foreground=\"#FF0000\">[{}]</span> | {} {}\n",
            now.format("%Y-%m-%d %H:%M:%S"),
            problem_type,
            $s
        );

        if let Ok(mut lua_logs) = $crate::globals::LUA_LOGS.lock() {
            lua_logs.push_str(&log_msg);
        } else {
            eprintln!("FATAL: failed to lock lua logs mutex!");
        }

        println!("MIMICKING - TYPE: {}, INFO: {}", $type, $s);
    }};
}

use std::cell::RefCell;
use std::rc::Rc;

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends adw::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

static LOGO_PNG: &[u8] = include_bytes!("../file.png");

use b9::css;
use b9::css::Styleable;
use glib::Object;

use globals::LUA_LOGS;
use gtk::gdk;
use gtk::gdk::Display;
use gtk::gio;
use gtk::CssProvider;
use serde::Deserialize;

use gtk::glib;
use gtk::prelude::*;

const APP_ID: &str = "io.github.face_hh.Napture";

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

    app.connect_startup(|_| {
        let mut content = r"
        tab-label {
            margin-bottom: 2px;
        }
        tab {
            background-color: #424242;
            border-radius: 12px;
            padding: 10px;
        }
        search {
            background-color: #424242;
            border-radius: 12px;
            padding: 5px;
            color: white;
        }
        search image {
            margin-right: 5px;
        }
        "
        .to_string();

        if !gtk::Settings::for_display(&Display::default().unwrap())
            .is_gtk_application_prefer_dark_theme()
        {
            content = content
                .replace(r"#424242;", r"#d4d2d2;")
                .replace(r"white;", r"black;")
        }

        css::load_css_into_app(&content);
    });
    app.connect_activate(move |app| {
        let args_clone = Rc::clone(&args);
        build_ui(app, args_clone)
    });

    app.run_with_args(&[""])
}

fn handle_search_activate(
    scroll_clone: Rc<RefCell<gtk::ScrolledWindow>>,
    previous_css_provider: Rc<RefCell<CssProvider>>,
    current_tab: Rc<RefCell<Tab>>,
    query_: Rc<RefCell<gtk::SearchEntry>>,
) {
    let previous_css_provider_clone = Rc::clone(&previous_css_provider);

    let mut tab_in_closure = current_tab.borrow_mut();
    let c = query_.clone();
    let query = c.borrow_mut();

    let url = query.text().to_string();
    let dns_url = fetch_dns(url.clone());

    if dns_url.is_empty() {
        tab_in_closure.url = url.clone();
    } else {
        tab_in_closure.url = dns_url;
    }

    query.set_text(&url.replace("buss://", ""));
    query.set_position(-1);

    if let Some(root) = query.root() {
        root.set_focus(None as Option<&gtk::Widget>)
    } else {
        println!("ERROR: Failed to set focus on search bar. Root is None.");
    }

    match b9::html::build_ui(
        tab_in_closure.clone(),
        Some(previous_css_provider_clone.take()),
        scroll_clone.clone(),
        query_,
    ) {
        Ok((htmlview, provider)) => {
            scroll_clone.borrow_mut().set_child(Some(&htmlview));
            *previous_css_provider.borrow_mut() = provider;
        }
        Err(e) => {
            tab_in_closure.label_widget.set_label(&e.to_string());
        }
    };
}

fn build_ui(app: &adw::Application, args: Rc<RefCell<Vec<String>>>) {
    let tabs: Vec<Tab> = vec![];

    let window: Window = Object::builder().property("application", app).build();

    // let cursor_pointer = Cursor::from_name("pointer", None);

    let search = gtk::SearchEntry::builder()
        .css_name("search")
        .width_request(500)
        .build();
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

    let scroll = gtk::ScrolledWindow::builder().css_classes(vec!["body"]).build();

    scroll.style();

    let scroll_clone = scroll.clone();

    let rc_css_provider = Rc::new(RefCell::new(CssProvider::new()));
    let rc_scroll = Rc::new(RefCell::new(scroll.clone()));
    let rc_tab = Rc::new(RefCell::new(current_tab.clone()));
    let rc_search = Rc::new(RefCell::new(search.clone()));

    // CLI command
    if let Some(dev_build) = args.borrow().get(1) {
        tab1.url = dev_build.to_string();
    }

    let app_ = Rc::new(RefCell::new(app.clone()));

    let event_controller = gtk::EventControllerKey::new();

    event_controller.connect_key_pressed(move |_, key, _a, b| {
        let app_clone = Rc::clone(&app_);

        if b == (gdk::ModifierType::SHIFT_MASK | gdk::ModifierType::CONTROL_MASK)
            && key == gdk::Key::P
        {
            display_lua_logs(&app_clone);
        }

        glib::Propagation::Proceed
    });

    window.add_controller(event_controller);

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

    window.set_default_size(500, 500);
    window.present();
    
    if let Ok((htmlview, provider)) = b9::html::build_ui(tab1.clone(), None, rc_scroll.clone(), rc_search.clone()) {
        scroll_clone.set_child(Some(&htmlview));
        *rc_css_provider.borrow_mut() = provider;
    } else {
        println!("ERROR: HTML engine failed.");
    }
    
    // search bar
    search.connect_activate(move |query| {
        let pprevious_css_provider = Rc::clone(&rc_css_provider);
        let scroll_clonee = Rc::clone(&rc_scroll);
        let tabb = Rc::clone(&rc_tab);

        handle_search_activate(scroll_clonee, pprevious_css_provider, tabb, Rc::new(RefCell::new(query.clone())));
    });
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

    let gesture = gtk::GestureClick::new();

    let bytes = glib::Bytes::from_static(LOGO_PNG);
    let logo = gdk::Texture::from_bytes(&bytes).expect("gtk-rs.svg to load");

    gesture.connect_released(move |_, _, _, _| {
        let dialog = gtk::AboutDialog::builder()
            .modal(true)
            .program_name("Bussin Napture")
            .version("v1.2.1")
            .website("https://github.com/face-hh/webx")
            .website_label("GitHub")
            .license_type(gtk::License::Apache20)
            .authors(["facedev"])
            .logo(&logo)
            .build();

        dialog.present();
    });

    tabname.add_controller(gesture);
    tab.append(&tabicon);
    tab.append(&tabname);

    let res = Tab {
        url: "https://github.com/face-hh/dingle-frontend".to_string(),
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
    let mut url = url.replace("buss://", "");

    url = url.split("?").nth(0).unwrap_or(&url).to_owned();
    
    let client: reqwest::blocking::ClientBuilder = reqwest::blocking::Client::builder();

    let clienturl = format!(
        "https://api.buss.lol/domain/{}/{}",
        url.split('.').next().unwrap_or(""),
        url.split('.').nth(1).unwrap_or(""),
    );

    let client = match client.build() {
        Ok(client) => client,
        Err(e) => {
            eprintln!("ERROR: Couldn't build reqwest client: {}", e);
            return url;
        }
    };

    if let Ok(response) = client.get(clienturl).send() {
        let status = response.status();

        if let Ok(json) = response.json::<DomainInfo>() {
            json.ip
        } else {
            lualog!("debug", format!("Failed to parse response body from DNS API. Error code: {}.", status.as_u16()));
            String::new()
        }
    } else {
        lualog!(
            "debug",
            "Failed to send HTTP request to DNS API."
        );
        String::new()
    }
}

fn display_lua_logs(app: &Rc<RefCell<adw::Application>>) {
    let window: Window = Object::builder()
        .property("application", glib::Value::from(&*app.borrow_mut()))
        .build();

    let gtkbox = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(6)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let lualogs = LUA_LOGS.lock().unwrap();

    let label = gtk::Label::builder()
        .halign(gtk::Align::Start)
        .valign(gtk::Align::Start)
        .build();

    label.set_use_markup(true);
    label.set_markup(&lualogs);

    gtkbox.append(&label);

    let scroll = gtk::ScrolledWindow::builder().build();

    scroll.set_child(Some(&gtkbox));

    let event_controller = gtk::EventControllerKey::new();

    event_controller.connect_key_pressed(move |_, key, _a, b| {
        if b == (gdk::ModifierType::CONTROL_MASK) && key == gdk::Key::r {
            let lualogs = LUA_LOGS.lock().unwrap();

            label.set_markup(&lualogs);
        }

        glib::Propagation::Proceed
    });

    window.add_controller(event_controller);

    window.set_child(Some(&scroll));
    let labell = gtk::Label::new(Some("Napture logs"));
    let empty_label = gtk::Label::new(Some(""));
    let headerbar = gtk::HeaderBar::builder().build();

    headerbar.pack_start(&labell);
    headerbar.set_title_widget(Some(&empty_label));

    window.set_titlebar(Some(&headerbar));

    window.present();
}
