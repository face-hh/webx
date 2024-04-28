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
                let mut font_size = "11px".to_string();

                match self.css_name().as_str() {
                    // SHUT THE FUCK UP
                    "h1" => font_size = "24px".to_string(),
                    "h2" => font_size = "22px".to_string(),
                    "h3" => font_size = "20px".to_string(),
                    "h4" => font_size = "18px".to_string(),
                    "h5" => font_size = "16px".to_string(),
                    "h6" => font_size = "14px".to_string(),
                    _ => {}
                };

                // i need a ultrawide monitor to read this shit
                let line_height = get_rule(&rules, "line-height", &"1");

                let color = get_rule(&rules, "color", &"#ffffff");
                let font_family = get_rule(&rules, "font-family", &"Noto Sans");
                let font_weight = get_rule(&rules, "font-weight", &"normal");
                let underline = get_rule(&rules, "underline", &"none");
                let underline_color = get_rule(&rules, "underline-color", &"black");
                let overline = get_rule(&rules, "overline", &"none");
                let overline_color = get_rule(&rules, "overline-color", &"black");
                let strikethrough = get_rule(&rules, "strikethrough", &"false");
                let strikethrough_color = get_rule(&rules, "strikethrough-color", &"black");

                font_size = font_size.replace("px", "pt");

                let margin_top = get_rule(&rules, "margin-top", "0").replace("px", "");
                let margin_bottom = get_rule(&rules, "margin-bottom", "0").replace("px", "");
                let margin_left = get_rule(&rules, "margin-left", "0").replace("px", "");
                let margin_right = get_rule(&rules, "margin-right", "0").replace("px", "");

                let border_style = get_rule(&rules, "border-style", "none");
                let border_color = get_rule(&rules, "border-color", "black");
                let border_width = get_rule(&rules, "border-width", "0");
                let border_radius = get_rule(&rules, "border-radius", "0");
                let padding = get_rule(&rules, "padding", "0");

                let markup = &format!(
                    "<span foreground=\"{color}\" size=\"{font_size}\" line_height=\"{line_height}\" font_family=\"{font_family}\" font_weight=\"{font_weight}\" underline=\"{underline}\" underline_color=\"{underline_color}\" overline=\"{overline}\" overline_color=\"{overline_color}\" strikethrough=\"{strikethrough}\" strikethrough_color=\"{strikethrough_color}\">{}</span>",
                    self.label()
                );

                self.set_margin_top(margin_top.parse::<i32>().unwrap_or(0));
                self.set_margin_bottom(margin_bottom.parse::<i32>().unwrap_or(0));
                self.set_margin_start(margin_left.parse::<i32>().unwrap_or(0));
                self.set_margin_end(margin_right.parse::<i32>().unwrap_or(0));

                if border_style != "none" {
                    let frame = gtk::Frame::new(None);
                    frame.set_label_widget(Some(self));

                    
                }

                self.set_markup(markup);
            }
        }
    }
}

impl Styleable for gtk::DropDown {
    fn style(&self) {}
}

impl Styleable for gtk::LinkButton {
    fn style(&self) {
        let lbl = gtk::Label::builder()
            .css_name("a")
            .label(
                self.child()
                    .unwrap()
                    .downcast::<gtk::Label>()
                    .unwrap()
                    .label(),
            )
            .build();
        self.set_child(Some(&lbl));

        Styleable::style(&lbl);
    }
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
