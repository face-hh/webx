mod b9;
mod custom_window;
mod parser;

use custom_window::Window;
use gtk::gdk::{Cursor, Display};
use gtk::glib;
use gtk::{prelude::*, CssProvider};

const APP_ID: &str = "org.bussin.napture";

fn main() -> glib::ExitCode {
    gtk::gio::resources_register_include!("icons.gresource").unwrap();

    let app = adw::Application::builder().application_id(APP_ID).build();

    app.connect_startup(|_| load_css());
    app.connect_activate(build_ui);

    app.run()
}

fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_string(include_str!("style.css"));

    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

fn build_ui(app: &adw::Application) {
    let window = Window::new(app);

    let cursor_pointer = Cursor::from_name("pointer", None);

    let search = gtk::SearchEntry::builder().build();
    let separator = gtk::Separator::new(gtk::Orientation::Horizontal);
    let headerbar = gtk::HeaderBar::builder().build();

    let tabs = gtk::Box::builder().css_name("tabs").spacing(6).build();

    let tab1 = make_tab(tabs.clone(), "New Tab", "file.png", cursor_pointer.as_ref());
    let tab2 = make_tab(
        tabs.clone(),
        "New Tab 2",
        "file.png",
        cursor_pointer.as_ref(),
    );
    let tab3 = make_tab(
        tabs.clone(),
        "New Tab 3",
        "file.png",
        cursor_pointer.as_ref(),
    );
    let tab4 = make_tab(
        tabs.clone(),
        "New Tab 4",
        "file.png",
        cursor_pointer.as_ref(),
    );

    tabs.append(&tab1);
    tabs.append(&tab2);
    tabs.append(&tab3);
    tabs.append(&tab4);

    headerbar.set_title_widget(Some(&tabs));

    window.set_titlebar(Some(&headerbar));
    
    let htmlview = b9::build_ui().unwrap();
    let bruh = gtk::ScrolledWindow::builder().build();

    bruh.set_child(Some(&htmlview));
    bruh.set_vexpand(true);

    let nav = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(6)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    nav.append(&separator);
    nav.append(&search);
    nav.append(&bruh);

    window.set_child(Some(&nav));
    window.present();
}

fn make_tab(tabs: gtk::Box, label: &str, icon: &str, cursor_pointer: Option<&Cursor>) -> gtk::Box {
    let tab = gtk::Box::builder()
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .spacing(12)
        .css_name("tab")
        .build();

    let tab_copy = tab.clone();
    let x = gtk::Button::builder().css_name("tab-close").build();

    x.set_icon_name("close");

    x.connect_clicked(move |_| {
        tabs.remove(&tab_copy);
    });

    let tabicon = gtk::Image::from_file(icon);

    let tabname = gtk::Label::builder()
        .css_name("tab-label")
        .label(label)
        .build();

    let gesture = gtk::GestureClick::new();
    gesture.connect_pressed(|_gesture, _, _, _| {
        println!("I've been clicked! By tabname");
    });

    tabname.add_controller(gesture);

    tabname.set_cursor(cursor_pointer);
    x.set_cursor(cursor_pointer);

    tab.append(&tabicon);
    tab.append(&tabname);
    tab.append(&x);

    return tab;
}
