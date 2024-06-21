#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod b9;
mod globals;
mod historymod;
mod imp;
mod parser;
mod page_source_mod;

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
use std::fs;
use std::path::PathBuf;
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
use gtk::SignalListItemFactory;
use historymod::History;
use historymod::HistoryObject;
use page_source_mod::PageSource;

use globals::APPDATA_PATH;
use globals::DNS_SERVER;
use globals::LUA_TIMEOUTS;
use globals::LUA_LOGS;
use gtk::gdk;
use gtk::gdk::Display;
use gtk::gio;
use gtk::CssProvider;
use serde::Deserialize;

use gtk::prelude::*;

use directories::ProjectDirs;

const APP_ID: &str = "io.github.face_hh.Napture";
const DEFAULT_URL: &str = "dingle.it";

#[derive(Clone, Debug)]
struct Tab {
    url: String,
    widget: gtk::Box,
    label_widget: gtk::Label,
    icon_widget: gtk::Image,
    page_source: Rc<RefCell<PageSource>>,
    // id: String,
}

fn main() -> glib::ExitCode {
    init_config();

    let args = Rc::new(RefCell::new(std::env::args().collect::<Vec<String>>()));
    let config = Rc::new(RefCell::new(get_config()));
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
        let config_clone = Rc::clone(&config);
        build_ui(app, args_clone, config_clone);
    });

    app.run_with_args(&[""])
}

fn handle_search_update(
    scroll: Rc<RefCell<gtk::ScrolledWindow>>,
    css_provider: Rc<RefCell<CssProvider>>,
    current_tab: Rc<RefCell<Tab>>,
    searchbar: Rc<RefCell<gtk::SearchEntry>>,
) {
    let mut tab_in_closure = current_tab.borrow_mut();
    let searchbar_clone = searchbar.clone();
    let searchbar_mut = searchbar_clone.borrow_mut();

    let url = searchbar_mut.text().to_string();
    let dns_url = fetch_dns(url.clone());

    if dns_url.is_empty() {
        tab_in_closure.url = url.clone();
    } else {
        tab_in_closure.url = dns_url;
    }

    {
        //clear page_source info for tab before fetching new page files
        //enclosed to let borrowed mut go out of scope
        let mut page_source_in_closure = tab_in_closure.page_source.borrow_mut();
        page_source_in_closure.clear();
    }

    searchbar_mut.set_text(&url.replace("buss://", ""));
    searchbar_mut.set_position(-1);

    if let Some(root) = searchbar_mut.root() {
        root.set_focus(None as Option<&gtk::Widget>)
    } else {
        println!("ERROR: Failed to set focus on search bar. Root is None.");
    }

    match b9::html::build_ui(
        tab_in_closure.clone(),
        Some(css_provider.take()),
        scroll.clone(),
        searchbar,
    ) {
        Ok((htmlview, next_css_provider)) => {
            scroll.borrow_mut().set_child(Some(&htmlview));
            *css_provider.borrow_mut() = next_css_provider;
        }
        Err(e) => {
            tab_in_closure.label_widget.set_label(&e.to_string());
        }
    };
}

fn update_buttons(go_back: &gtk::Button, go_forward: &gtk::Button, history: &Rc<RefCell<History>>) {
    let history = history.borrow();
    go_back.set_sensitive(!history.is_empty() && !history.on_history_start());
    go_forward.set_sensitive(!history.is_empty() && !history.on_history_end());
}

fn get_time() -> String {
    chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string()
}

fn build_ui(app: &adw::Application, args: Rc<RefCell<Vec<String>>>, config: Rc<RefCell<serde_json::Value>>) {
    let history = Rc::new(RefCell::new(History::new()));
    let page_source = Rc::new(RefCell::new(PageSource::new()));

    let default_url = if let Some(dev_build) = args.borrow().get(1) { // cli
        dev_build.to_string()
    } else { DEFAULT_URL.to_string() };

    let default_dns_url = fetch_dns(default_url.clone());

    let default_tab_url = if default_dns_url.is_empty() {
        default_url.clone()
    } else {
        default_dns_url
    };

    let tabs: Vec<Tab> = vec![];

    let window: Window = Object::builder().property("application", app).build();

    // let cursor_pointer = Cursor::from_name("pointer", None);

    let search = gtk::SearchEntry::builder()
        .css_name("search")
        .width_request(500)
        .text(default_url.clone())
        .build();
    let empty_label = gtk::Label::new(Some(""));
    let headerbar = gtk::HeaderBar::builder().build();

    let tabs_widget = gtk::Box::builder().css_name("tabs").spacing(6).build();

    let tab1 = make_tab(
        // tabs_widget.clone(),
        "New Tab",
        "file.png",
        // cursor_pointer.as_ref(),
        tabs.clone(),
        default_tab_url.clone(),
        page_source.clone()
    );

    history
        .borrow_mut()
        .add_to_history(default_tab_url.clone(), get_time(), true);

    let current_tab = tab1.clone();

    let refresh_button = make_refresh_button();
    let home_button = make_home_button();
    let go_back = make_go_back_button();
    let go_forward = make_go_forward_button();

    tabs_widget.append(&go_back);
    tabs_widget.append(&go_forward);
    tabs_widget.append(&tab1.widget);
    tabs_widget.append(&search);
    tabs_widget.append(&refresh_button);
    tabs_widget.append(&home_button);

    headerbar.pack_start(&tabs_widget);
    headerbar.set_title_widget(Some(&empty_label));

    window.set_titlebar(Some(&headerbar));

    let scroll = gtk::ScrolledWindow::builder()
        .css_classes(vec!["body"])
        .build();
    scroll.style();

    let rc_css_provider = Rc::new(RefCell::new(CssProvider::new()));
    let rc_scroll = Rc::new(RefCell::new(scroll.clone()));
    let rc_tab = Rc::new(RefCell::new(current_tab.clone()));
    let rc_search = Rc::new(RefCell::new(search.clone()));

    let app_: Rc<RefCell<adw::Application>> = Rc::new(RefCell::new(app.clone()));

    let event_controller = gtk::EventControllerKey::new();
    let history_ = Rc::clone(&history);
    let page_source_ = Rc::clone(&page_source);

    // let lua_logs_showing = Rc::new(RefCell::new(false));
    // let settings_showing = Rc::new(RefCell::new(false));
    // let history_showing = Rc::new(RefCell::new(false));

    let source_viewer_showing = Rc::new(RefCell::new(false));
    let source_viewer_window: Rc<RefCell<Option<Window>>> = Rc::new(RefCell::new(None));

    let source_viewer_showing_ = source_viewer_showing.clone();
    let source_viewer_window_ = source_viewer_window.clone();

    event_controller.connect_key_pressed(move |_, key, _a, b| {
        let app_clone = Rc::clone(&app_);
        let history_clone = Rc::clone(&history_);
        let page_source_clone = Rc::clone(&page_source_);

        let source_viewer_showing_clone = source_viewer_showing_.clone();
        let mut source_viewer_window_ref = source_viewer_window_.borrow_mut();

        if b == (gdk::ModifierType::SHIFT_MASK | gdk::ModifierType::CONTROL_MASK)
            && key == gdk::Key::P
        {
            display_lua_logs(&app_clone);
        }

        if b == (gdk::ModifierType::SHIFT_MASK | gdk::ModifierType::CONTROL_MASK)
            && key == gdk::Key::S
        {
            display_settings_page(&app_clone);
        }

        if b == (gdk::ModifierType::SHIFT_MASK | gdk::ModifierType::CONTROL_MASK)
            && key == gdk::Key::H
        {
            display_history_page(&app_clone, history_clone);
        }

        if key == gdk::Key::F12 {
            if !*source_viewer_showing_clone.borrow_mut() {
                *source_viewer_window_ref = Some(display_source_viewer(&app_clone, page_source_clone, source_viewer_showing_clone));
            } else {
                if let Some(ref window) = *source_viewer_window_ref{
                    window.close();
                    *source_viewer_window_ref = Some(display_source_viewer(&app_clone, page_source_clone, source_viewer_showing_clone));
                }
            }
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

    window.set_default_size(1000, 700);
    window.present();

    if let Some(past_history) = &config.borrow()["history"].as_array() {
        for entry in past_history.iter() {
            if let Some(res) = entry.as_object() {
                if let (Some(raw_url), Some(raw_date)) = (res.get("url"), res.get("date")) {
                    if let (Some(url), Some(date)) = (raw_url.as_str(), raw_date.as_str()) {
                        history.borrow_mut().add_to_history(url.to_owned(), date.to_owned(), false);
                    }
                }
            }
        }
    }

    if let Some(dns) = &config.borrow()["dns"].as_str() {
        *DNS_SERVER.lock().unwrap() = dns.to_string();
    }

    if let Ok((htmlview, provider)) =
        b9::html::build_ui(tab1.clone(), None, rc_scroll.clone(), rc_search.clone())
    {
        rc_scroll.clone().borrow_mut().set_child(Some(&htmlview));
        *rc_css_provider.borrow_mut() = provider;
    } else {
        println!("ERROR: HTML engine failed.");
    }

    //main UI listeners

    // search bar
    
    let app_: Rc<RefCell<adw::Application>> = Rc::new(RefCell::new(app.clone()));
    search.connect_activate({
        let rc_scroll_search = rc_scroll.clone();
        let rc_css_provider_search = rc_css_provider.clone();
        let rc_tab_search = rc_tab.clone();
        let history = history.clone();
        let go_forward = go_forward.clone();
        let go_back = go_back.clone();
        let source_viewer_window_ = source_viewer_window.clone();
        let source_viewer_showing_= source_viewer_showing.clone();
        let page_source_ = page_source.clone();

        move |query| {
            update_buttons(&go_back, &go_forward, &history);
            handle_search_update(
                rc_scroll_search.clone(),
                rc_css_provider_search.clone(),
                rc_tab_search.clone(),
                Rc::new(RefCell::new(query.clone())),
            );
            //If open, refresh source viewer on new page
            if *source_viewer_showing_.clone().borrow_mut() {
                let mut source_viewer_window_ref = source_viewer_window_.borrow_mut();
                let page_source_clone = Rc::clone(&page_source_);
                let app_clone = app_.clone();

                if let Some(ref window) = *source_viewer_window_ref{
                    window.close();
                    *source_viewer_window_ref = Some(display_source_viewer(&app_clone, page_source_clone, source_viewer_showing_.clone()));
                }
            }
            history
                .borrow_mut()
                .add_to_history(query.text().to_string(), get_time(), true);
        }
    });

    let app_: Rc<RefCell<adw::Application>> = Rc::new(RefCell::new(app.clone()));
    refresh_button.connect_clicked({
        let rc_scroll_refresh = rc_scroll.clone();
        let rc_css_provider_refresh = rc_css_provider.clone();
        let rc_tab_refresh = rc_tab.clone();
        let rc_search_refresh = rc_search.clone();
        let history = history.clone();
        let source_viewer_window_ = source_viewer_window.clone();
        let source_viewer_showing_= source_viewer_showing.clone();
        let page_source_ = page_source.clone();
        history
            .borrow_mut()
            .add_to_history(rc_search_refresh.borrow().text().to_string(), get_time(), true);
        move |_button| {
            handle_search_update(
                rc_scroll_refresh.clone(),
                rc_css_provider_refresh.clone(),
                rc_tab_refresh.clone(),
                rc_search_refresh.clone(),
            );
            //If open, refresh source viewer on new page
            if *source_viewer_showing_.clone().borrow_mut() {
                let mut source_viewer_window_ref = source_viewer_window_.borrow_mut();
                let page_source_clone = Rc::clone(&page_source_);
                let app_clone = app_.clone();

                if let Some(ref window) = *source_viewer_window_ref{
                    window.close();
                    *source_viewer_window_ref = Some(display_source_viewer(&app_clone, page_source_clone, source_viewer_showing_.clone()));
                }
            }
        }
    });

    let app_: Rc<RefCell<adw::Application>> = Rc::new(RefCell::new(app.clone()));
    home_button.connect_clicked({
        let rc_scroll_home = rc_scroll.clone();
        let rc_css_provider_home = rc_css_provider.clone();
        let rc_tab_home = rc_tab.clone();
        let rc_search_home = rc_search.clone();
        let history = history.clone(); 
        let go_forward = go_forward.clone();
        let go_back = go_back.clone();
        let source_viewer_window_ = source_viewer_window.clone();
        let source_viewer_showing_= source_viewer_showing.clone();
        let page_source_ = page_source.clone();
        move |_button| {
            rc_search_home.borrow_mut().set_text(DEFAULT_URL);
            history
                .borrow_mut()
                .add_to_history(DEFAULT_URL.to_string(), get_time(), true);
            update_buttons(&go_back, &go_forward, &history);
            handle_search_update(
                rc_scroll_home.clone(),
                rc_css_provider_home.clone(),
                rc_tab_home.clone(),
                rc_search_home.clone(),
            );
            //If open, refresh source viewer on new page
            if *source_viewer_showing_.clone().borrow_mut() {
                let mut source_viewer_window_ref = source_viewer_window_.borrow_mut();
                let page_source_clone = Rc::clone(&page_source_);
                let app_clone = app_.clone();

                if let Some(ref window) = *source_viewer_window_ref{
                    window.close();
                    *source_viewer_window_ref = Some(display_source_viewer(&app_clone, page_source_clone, source_viewer_showing_.clone()));
                }
            }
        }
    });

    let app_: Rc<RefCell<adw::Application>> = Rc::new(RefCell::new(app.clone()));
    go_back.connect_clicked({
        let rc_scroll_back = rc_scroll.clone();
        let rc_css_provider_back = rc_css_provider.clone();
        let rc_tab_back = rc_tab.clone();
        let rc_search_back = rc_search.clone();
        let history = history.clone();
        let go_forward = go_forward.clone();
        let go_back = go_back.clone();
        let source_viewer_window_ = source_viewer_window.clone();
        let source_viewer_showing_= source_viewer_showing.clone();
        let page_source_ = page_source.clone();
        move |_| {
            history.borrow_mut().go_back();
            update_buttons(&go_back, &go_forward, &history);
            let current_url = history.borrow().current().unwrap().url.clone();
            rc_search_back.borrow_mut().set_text(&current_url);
            handle_search_update(
                rc_scroll_back.clone(),
                rc_css_provider_back.clone(),
                rc_tab_back.clone(),
                rc_search_back.clone(),
            );
            //If open, refresh source viewer on new page
            if *source_viewer_showing_.clone().borrow_mut() {
                let mut source_viewer_window_ref = source_viewer_window_.borrow_mut();
                let page_source_clone = Rc::clone(&page_source_);
                let app_clone = app_.clone();

                if let Some(ref window) = *source_viewer_window_ref{
                    window.close();
                    *source_viewer_window_ref = Some(display_source_viewer(&app_clone, page_source_clone, source_viewer_showing_.clone()));
                }
            }
        }
    });

    let app_: Rc<RefCell<adw::Application>> = Rc::new(RefCell::new(app.clone()));
    go_forward.connect_clicked({
        let rc_scroll_forward = rc_scroll.clone();
        let rc_css_provider_forward = rc_css_provider.clone();
        let rc_tab_forward = rc_tab.clone();
        let rc_search_forward = rc_search.clone();
        let history = history.clone();
        let go_forward = go_forward.clone();
        let go_back = go_back.clone();
        let source_viewer_window_ = source_viewer_window.clone();
        let source_viewer_showing_= source_viewer_showing.clone();
        let page_source_ = page_source.clone();
        move |_| {
            history.borrow_mut().go_forward();
            update_buttons(&go_back, &go_forward, &history);
            let current_url = history.borrow().current().unwrap().url.clone();
            rc_search_forward.borrow_mut().set_text(&current_url);
            handle_search_update(
                rc_scroll_forward.clone(),
                rc_css_provider_forward.clone(),
                rc_tab_forward.clone(),
                rc_search_forward.clone(),
            );
            //If open, refresh source viewer on new page
            if *source_viewer_showing_.clone().borrow_mut() {
                let mut source_viewer_window_ref = source_viewer_window_.borrow_mut();
                let page_source_clone = Rc::clone(&page_source_);
                let app_clone = app_.clone();

                if let Some(ref window) = *source_viewer_window_ref{
                    window.close();
                    *source_viewer_window_ref = Some(display_source_viewer(&app_clone, page_source_clone, source_viewer_showing_.clone()));
                }
            }
        }
    });

    //mouse button listeners for forward/back page navigation
    //mouse back - button "8"
    let gesture_back = gtk::GestureClick::new();
    gesture_back.set_button(8);

    let app_: Rc<RefCell<adw::Application>> = Rc::new(RefCell::new(app.clone()));
    gesture_back.connect_pressed({
        let rc_scroll_back = rc_scroll.clone();
        let rc_css_provider_back = rc_css_provider.clone();
        let rc_tab_back = rc_tab.clone();
        let rc_search_back = rc_search.clone();
        let history = history.clone();
        let go_forward = go_forward.clone();
        let go_back = go_back.clone();
        let source_viewer_window_ = source_viewer_window.clone();
        let source_viewer_showing_= source_viewer_showing.clone();
        let page_source_ = page_source.clone();
        move |gesture_back, n, _x, _y| {
            gesture_back.set_state(gtk::EventSequenceState::Claimed);
            
            if n < 2 {
                history.borrow_mut().go_back();
                update_buttons(&go_back, &go_forward, &history);
                let current_url = history.borrow().current().unwrap().url.clone();
                rc_search_back.borrow_mut().set_text(&current_url);
                handle_search_update(
                    rc_scroll_back.clone(),
                    rc_css_provider_back.clone(),
                    rc_tab_back.clone(),
                    rc_search_back.clone(),
                );
                //If open, refresh source viewer on new page
                if *source_viewer_showing_.clone().borrow_mut() {
                    let mut source_viewer_window_ref = source_viewer_window_.borrow_mut();
                    let page_source_clone = Rc::clone(&page_source_);
                    let app_clone = app_.clone();

                    if let Some(ref window) = *source_viewer_window_ref{
                        window.close();
                        *source_viewer_window_ref = Some(display_source_viewer(&app_clone, page_source_clone, source_viewer_showing_.clone()));
                    }
                }
        }
    }});

    window.add_controller(gesture_back);

    //mouse forward - button "9"
    let gesture_forward = gtk::GestureClick::new();
    gesture_forward.set_button(9);

    let app_: Rc<RefCell<adw::Application>> = Rc::new(RefCell::new(app.clone()));
    gesture_forward.connect_pressed({
        let rc_scroll_forward = rc_scroll.clone();
        let rc_css_provider_forward = rc_css_provider.clone();
        let rc_tab_forward = rc_tab.clone();
        let rc_search_forward = rc_search.clone();
        let history = history.clone();
        let go_forward = go_forward.clone();
        let go_back = go_back.clone();
        let source_viewer_window_ = source_viewer_window.clone();
        let source_viewer_showing_= source_viewer_showing.clone();
        let page_source_ = page_source.clone();
        move |gesture_forward, n, _x, _y|{

            gesture_forward.set_state(gtk::EventSequenceState::Claimed);

            if n < 2 {
                history.borrow_mut().go_forward();
                update_buttons(&go_back, &go_forward, &history);
                let current_url = history.borrow().current().unwrap().url.clone();
                rc_search_forward.borrow_mut().set_text(&current_url);
                handle_search_update(
                    rc_scroll_forward.clone(),
                    rc_css_provider_forward.clone(),
                    rc_tab_forward.clone(),
                    rc_search_forward.clone()
                );
                //If open, refresh source viewer on new page
                if *source_viewer_showing_.clone().borrow_mut() {
                    let mut source_viewer_window_ref = source_viewer_window_.borrow_mut();
                    let page_source_clone = Rc::clone(&page_source_);
                    let app_clone = app_.clone();

                    if let Some(ref window) = *source_viewer_window_ref{
                        window.close();
                        *source_viewer_window_ref = Some(display_source_viewer(&app_clone, page_source_clone, source_viewer_showing_.clone()));
                    }
                }
        }
    }});

    window.add_controller(gesture_forward);

    // every 5 seconds remove "stale" timeouts
    glib::source::timeout_add_local(std::time::Duration::from_millis(5000), move || {
        let mut timeouts = LUA_TIMEOUTS.lock().unwrap();
        timeouts.retain(|source| {
            !source.is_destroyed()
        });
        glib::ControlFlow::Continue
    });
}

// fn navigate_history (rc_scrolled_window: Rc<RefCell<ScrolledWindow>>, css_provider: Rc<RefCell<CssProvider>>){

// }

// commented code here was an attempt at implementing multiple tabs.
// it will be kept here in case I decide to implement multiple tabs again
fn make_tab(
    // tabs_widget: gtk::Box,
    label: &str,
    icon: &str,
    // cursor_pointer: Option<&Cursor>,
    mut tabs: Vec<Tab>,
    default_url: String,
    page_source: Rc<RefCell<PageSource>>,
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
            .version("v1.3.1")
            .website("https://github.com/face-hh/webx")
            .website_label("GitHub")
            .license_type(gtk::License::Apache20)
            .authors(["facedev"])
            .comments("Available shortcuts:\nNapture logs - CTRL SHIFT P\nNapture settings - CTRL SHIFT S\nGTK Inspector - CTRL SHIFT I")
            .logo(&logo)
            .build();

        dialog.present();
    });

    tabname.add_controller(gesture);
    tab.append(&tabicon);
    tab.append(&tabname);

    let res = Tab {
        url: default_url,
        widget: tab,
        // id: tabid,
        label_widget: tabname,
        icon_widget: tabicon,
        page_source: page_source
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

fn make_refresh_button() -> gtk::Button {
    let button = gtk::Button::from_icon_name("view-refresh");
    button.add_css_class("refresh-button");

    button
}

fn make_home_button() -> gtk::Button {
    let button = gtk::Button::from_icon_name("go-home");
    button.add_css_class("home-button");

    button
}

fn make_go_back_button() -> gtk::Button {
    let button = gtk::Button::from_icon_name("go-previous");
    button.add_css_class("go-back-button");

    //if history.is_empty or already at the beginning of the history, disable the
    let history = Rc::new(RefCell::new(History::new()));
    if history.borrow_mut().is_empty() || history.borrow_mut().on_history_end() {
        button.set_sensitive(false);
    }

    button
}

fn make_go_forward_button() -> gtk::Button {
    let button = gtk::Button::from_icon_name("go-next");
    button.add_css_class("go-forward-button");

    //if history.is_empty or already at the beginning of the history, disable the
    let history = Rc::new(RefCell::new(History::new()));
    if history.borrow_mut().is_empty() || history.borrow_mut().on_history_start() {
        button.set_sensitive(false);
    }

    button
}

#[derive(Deserialize)]
struct DomainInfo {
    ip: String,
}

fn fetch_dns(url: String) -> String {
    let mut url = url.replace("buss://", "");

    url = url.split("?").nth(0).unwrap_or(&url).to_owned();

    let client: reqwest::blocking::ClientBuilder = reqwest::blocking::Client::builder();

    let clienturl = format!(
        "{}/domain/{}/{}",
        DNS_SERVER.lock().unwrap().as_str(),
        url.split('.').next().unwrap_or(""),
        url.split('.').nth(1).unwrap_or("")
            .split('/').next().unwrap_or(""),
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
            let path = url.split_once('/')
                .unwrap_or(("", "")).1;
            
            //check if json.ip is a standalone IP address,
            //if so, prepend "http://" to make it parsable
            //as a url.
            let ip = match json.ip.parse::<std::net::IpAddr>() {
                Ok(_ip) => format!("http://{}", json.ip),
                Err(_) => json.ip.clone(),
            };

            ip + &format!("/{}", path)
        } else {
            lualog!(
                "debug",
                format!(
                    "Failed to parse response body from DNS API. Error code: {}.",
                    status.as_u16()
                )
            );
            String::new()
        }
    } else {
        lualog!("debug", "Failed to send HTTP request to DNS API.");
        String::new()
    }
}

fn display_source_viewer(app: &Rc<RefCell<adw::Application>>, page_source: Rc<RefCell<PageSource>>, showing: Rc<RefCell<bool>>) -> Window{
    
    let window: Window = Object::builder()
        .property("application", glib::Value::from(&*app.borrow_mut()))
        .build();

    window.set_default_size(800, 900);
    window.set_height_request(500);
    window.set_width_request(800);

    // Create the main container box with horizontal orientation
    let main_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .spacing(6)
        .margin_top(6)
        .margin_bottom(6)
        .margin_start(6)
        .margin_end(6)
        .build();

    // Create the menu box for the left side
    let menu_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(6)
        .width_request(140)
        .build();

    // scroller for text area
    let content_scroll = gtk::ScrolledWindow::builder().build();

    let content_text_view = gtk::TextView::builder()
        .wrap_mode(gtk::WrapMode::Word)
        .editable(false)
        .hexpand(true)
        .build();

    content_scroll.set_child(Some(&content_text_view));

    // let files = page_source.get_files();

    // create buffers and listeners to update the text area buffer
    let content_buffer = content_text_view.buffer();

    for file in page_source.borrow().get_files() {

        let button = gtk::Button::with_label(file.get(0).expect(""));

        let file_clone = file.clone();

        button.connect_clicked({
            let content_buffer = content_buffer.clone();

            move |_| {
                content_buffer.set_text(file_clone.get(1).expect(""));
            }
        });

        menu_box.append(&button);
    }

    // Add menu box and content area to the main box
    main_box.append(&menu_box);
    main_box.append(&content_scroll);

    window.set_child(Some(&main_box));

    let label = gtk::Label::new(Some("Source"));
    let empty_label = gtk::Label::new(Some(""));
    let headerbar = gtk::HeaderBar::builder().build();

    headerbar.pack_start(&label);
    headerbar.set_title_widget(Some(&empty_label));

    window.set_titlebar(Some(&headerbar));

    let showing_clone = showing.clone();
    window.connect_close_request(move |_|{
        
        *showing_clone.borrow_mut() = false;

        gtk::glib::Propagation::Proceed
    });

    window.present();

    *showing.borrow_mut() = true;

    window
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

fn display_settings_page(app: &Rc<RefCell<adw::Application>>) {
    let window: Window = Object::builder()
        .property("application", glib::Value::from(&*app.borrow_mut()))
        .build();

    window.set_default_size(500, 300);

    let gtkbox = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(6)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let line = gtk::Separator::builder()
        .orientation(gtk::Orientation::Horizontal)
        .build();

    let label = gtk::Label::builder()
        .halign(gtk::Align::Start)
        .valign(gtk::Align::Start)
        .build();

    label.set_use_markup(true);
    label.set_markup("<span size=\"16pt\" font_weight=\"heavy\">DNS</span>");

    gtkbox.append(&label);
    gtkbox.append(&line);

    let dns_label = gtk::Label::builder()
        .halign(gtk::Align::Start)
        .valign(gtk::Align::Start)
        .build();

    dns_label.set_use_markup(true);
    dns_label.set_markup("DNS Server:");

    gtkbox.append(&dns_label);

    let dns_entry = gtk::Entry::builder()
        .halign(gtk::Align::Start)
        .valign(gtk::Align::Start)
        .build();

    dns_entry.set_text(DNS_SERVER.lock().unwrap().as_str());

    gtkbox.append(&dns_entry);

    dns_entry.connect_changed(move |entry| {
        let dns = &entry.text();

        // set the DNS server to the new value
        DNS_SERVER.lock().unwrap().clear();
        DNS_SERVER.lock().unwrap().push_str(&dns);
        set_config(String::from("dns"), serde_json::Value::String(dns.to_string()), false)
    });

    let scroll = gtk::ScrolledWindow::builder().build();

    scroll.set_child(Some(&gtkbox));

    window.set_child(Some(&scroll));
    let labell = gtk::Label::new(Some(" Napture settings"));
    let empty_label = gtk::Label::new(Some(""));
    let headerbar = gtk::HeaderBar::builder().build();

    headerbar.pack_start(&labell);
    headerbar.set_title_widget(Some(&empty_label));

    window.set_titlebar(Some(&headerbar));

    window.present();
}

fn display_history_page(app: &Rc<RefCell<adw::Application>>, history: Rc<RefCell<History>>) {
    let window: Window = Object::builder()
        .property("application", glib::Value::from(&*app.borrow_mut()))
        .build();

    window.set_default_size(500, 300);
    let vector: Vec<HistoryObject> = history
        .borrow()
        .clone()
        .items
        .into_iter()
        .rev()
        .map(|item| HistoryObject::new(item.url, item.position as i32, item.date))
        .collect();

    let model = gio::ListStore::new::<HistoryObject>();

    model.extend_from_slice(&vector);

    let factory = SignalListItemFactory::new();
    factory.connect_setup(move |_, list_item| {
        let label = gtk::Label::new(None);
        list_item
            .downcast_ref::<gtk::ListItem>()
            .expect("Needs to be ListItem")
            .set_child(Some(&label));
    });

    factory.connect_bind(move |_, list_item| {
        let history_object = list_item
            .downcast_ref::<gtk::ListItem>()
            .expect("Needs to be ListItem")
            .item()
            .and_downcast::<HistoryObject>()
            .expect("The item has to be an `HistoryObject`.");

        let label = list_item
            .downcast_ref::<gtk::ListItem>()
            .expect("Needs to be ListItem")
            .child()
            .and_downcast::<gtk::Label>()
            .expect("The child has to be a `Label`.");

        label.set_halign(gtk::Align::Start);

        label.set_use_markup(true);
        label.set_markup(&format!(
            "<span color=\"grey\">{}</span>. <span color=\"cyan\">{}</span> | <span>{}</span>",
            &history_object.position() + 1,
            &history_object.date(),
            &history_object.url().to_string()
        ));
    });

    let selection_model = gtk::SingleSelection::new(Some(model));
    let list_view = gtk::ListView::new(Some(selection_model), Some(factory));

    let scrolled_window = gtk::ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never) // Disable horizontal scrolling
        .min_content_width(360)
        .margin_bottom(10)
        .margin_end(10)
        .margin_start(10)
        .margin_top(10)
        .child(&list_view)
        .build();

    let labell = gtk::Label::new(Some(" Napture settings"));
    let empty_label = gtk::Label::new(Some(""));
    let headerbar = gtk::HeaderBar::builder().build();

    headerbar.pack_start(&labell);
    headerbar.set_title_widget(Some(&empty_label));

    window.set_child(Some(&scrolled_window));
    window.set_titlebar(Some(&headerbar));

    window.present();
}

fn init_config() {
    if let Some(proj_dirs) = ProjectDirs::from("com", "Bussin", "Napture") {
        let dir = proj_dirs.data_dir();
        let exists = &dir.join("config.json").exists();

        if *exists {
            let mut path = APPDATA_PATH.lock().unwrap();
            *path = dir.to_str().unwrap().to_string();
        } else {
            if let Err(error) = fs::create_dir_all(&dir) {
                println!("FATAL: COULD NOT CREATE APPDATA FOLDER, STACK: {}", error);
            } else {
                println!("Created config folder in AppData: {:?}", &dir.as_os_str());

                if let Err(error2) = fs::write(
                    PathBuf::from(&dir).join("config.json"),
                    format!(
                        "{{\"history\": [],\"dns\": \"{}\" }}",
                        DNS_SERVER.lock().unwrap()
                    ),
                ) {
                    println!(
                        "FATAL: COULD NOT CREATE CONFIG IN APPDATA, STACK: {}",
                        error2
                    );
                }

                let mut path = APPDATA_PATH.lock().unwrap();
                *path = dir.to_str().unwrap().to_string();
            }
        }
    }
}

fn get_config() -> serde_json::Value {
    let json_path = PathBuf::from(APPDATA_PATH.lock().unwrap().clone()).join("config.json");
    let contents = fs::read_to_string(&json_path).expect("Failed to read configuration for theme.");

    println!("{:?}", json_path);
    let json_contents: serde_json::Value = serde_json::from_str(&contents).expect("Failed to parse JSON");

    json_contents
}

fn set_config(property: String, value: serde_json::Value, array: bool) {
    let json_path = PathBuf::from(APPDATA_PATH.lock().unwrap().clone()).join("config.json");
    let contents = fs::read_to_string(&json_path).expect("Failed to read configuration for theme.");

    let mut json_contents: serde_json::Value = serde_json::from_str(&contents).expect("Failed to parse JSON");

    if array {
        if json_contents[property.clone()].is_array() {
            json_contents[property].as_array_mut().unwrap().push(value);
        }
    } else {
        json_contents[property] = value;
    }

    if let Ok(updated_json) = serde_json::to_string_pretty(&json_contents) {
        match fs::write(&json_path, &updated_json) {
            Ok(_) => {},
            Err(err) => {
                eprintln!("ERROR: Failed to save config to disk. Error: {}", err);
            }
        }
    }
}
