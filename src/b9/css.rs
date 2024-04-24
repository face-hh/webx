use crate::parser::{self};
use std::{collections::HashMap, fs, hash::Hash, sync::Mutex};

use gtk::prelude::*;

static CSS_RULES: Mutex<Option<HashMap<String, Vec<(String, String)>>>> = Mutex::new(None);

pub(crate) trait Styleable {
    fn style(&self);
}

impl Styleable for gtk::Label {
    fn style(&self) {
        let guard = CSS_RULES.lock().unwrap();
        let css = guard.as_ref().unwrap();
        let mut classes = self.css_classes();

        classes.push(self.css_name());
        self.set_use_markup(true);

        for class in classes {
            if let Some(rules) = css.get(&class.to_string()) {
                let default_color = "#ffffff".to_string();
                let mut default_fontsize = "".to_string();

                match self.css_name().as_str() { // SHUT THE FUCK UP
                    "h1" => default_fontsize = "24pt".to_string(),
                    "h2" => default_fontsize = "22pt".to_string(),
                    "h3" => default_fontsize = "20pt".to_string(),
                    "h4" => default_fontsize = "18pt".to_string(),
                    "h5" => default_fontsize = "16pt".to_string(),
                    "h6" => default_fontsize = "14pt".to_string(),
                    _ => default_fontsize = "13pt".to_string(),
                };

                let default_line_height = "1".to_string();

                let color = get_rule(&rules, "color", &default_color);
                let font_size =
                    get_rule(&rules, "font-size", &default_fontsize).replace("px", "pt");
                let line_height = get_rule(&rules, "line-height", &default_line_height);

                let markup = &format!(
                    "<span foreground=\"{color}\" size=\"{font_size}\" line_height=\"{line_height}\">{}</span>",
                    self.label()
                );

                println!("{}: {}", class, markup);
                self.set_markup(markup);
            }
        }
    }
}

impl Styleable for gtk::DropDown {
    fn style(&self) {}
}

impl Styleable for gtk::LinkButton {
    fn style(&self) {}
}

impl Styleable for gtk::Box {
    fn style(&self) {}
}

impl Styleable for gtk::TextView {
    fn style(&self) {}
}

impl Styleable for gtk::Separator {
    fn style(&self) {}
}

impl Styleable for gtk::Picture {
    fn style(&self) {}
}

impl Styleable for gtk::Entry {
    fn style(&self) {}
}

pub(crate) fn load_css() {
    let stylesheet_utf8_string = fs::read_to_string("test/styles.css").unwrap();
    let res = parser::parse(&stylesheet_utf8_string).unwrap();

    CSS_RULES.lock().unwrap().replace(res);
}

pub(crate) fn perform_styling<T: Styleable>(_element: &html_parser::Element, styleable: &T) {
    styleable.style();
}

fn get_rule(rules: &Vec<(String, String)>, property: &str, default_value: &str) -> String {
    rules
        .iter()
        .find(|(name, _)| name.as_str() == property)
        .map(|(_, value)| value.as_str())
        .unwrap_or(default_value)
        .to_owned()
}