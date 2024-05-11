// this code is held together with hopes, dreams and glue
use crate::parser;
use std::{collections::HashMap, sync::Mutex};

use gtk::{gdk::Display, prelude::*, CssProvider};

static CSS_RULES: Mutex<Option<HashMap<String, Vec<(String, String)>>>> = Mutex::new(None); // shut the fuck up

struct Properties {
    direction: String,
    align_items: String,

    width: i32,
    height: i32,

    line_height: String,
    color: String,
    wrap: String,
    background_color: String,
    font_family: String,
    font_weight: String,
    underline: String,
    underline_color: String,
    overline: String,
    overline_color: String,
    strikethrough: String,
    strikethrough_color: String,
    margin_top: String,
    margin_bottom: String,
    margin_left: String,
    margin_right: String,
    border_style: String,
    border_color: String,
    border_width: String,
    border_radius: String,
    padding: String,
    font_size: String,
    gap: i32,
}

pub(crate) trait Styleable {
    fn style(&self);
}

// h1, h2, h3, h4, h5, h6, p
impl Styleable for gtk::Label {
    fn style(&self) {
        let guard = CSS_RULES.lock().unwrap();
        if let Some(css) = guard.as_ref() {
            let mut classes = self.css_classes();
            let mut final_css = "".to_string();

            classes.push(self.css_name());

            self.set_use_markup(true);

            for class in classes {
                if let Some(rules) = css.get(&class.to_string()) {
                    let mut properties = get_properties(rules);

                    if properties.wrap == "wrap" {
                        self.set_wrap(true)
                    }

                    match self.css_name().as_str() {
                        "h1" => properties.font_size = "24px".to_string(),
                        "h2" => properties.font_size = "22px".to_string(),
                        "h3" => properties.font_size = "20px".to_string(),
                        "h4" => properties.font_size = "18px".to_string(),
                        "h5" => properties.font_size = "16px".to_string(),
                        "h6" => properties.font_size = "14px".to_string(),
                        _ => {}
                    };

                    properties.font_size = properties.font_size.replace("px", "pt");

                    let markup = &format!(
                    "<span foreground=\"{}\" size=\"{}\" line_height=\"{}\" font_family=\"{}\" font_weight=\"{}\" underline=\"{}\" underline_color=\"{}\" overline=\"{}\" overline_color=\"{}\" strikethrough=\"{}\" strikethrough_color=\"{}\">{}</span>",
                    properties.color,
                    properties.font_size,
                    properties.line_height,
                    properties.font_family,
                    properties.font_weight,
                    properties.underline,
                    properties.underline_color,
                    properties.overline,
                    properties.overline_color,
                    properties.strikethrough,
                    properties.strikethrough_color,
                    self.label(),
                );

                    self.set_markup(markup);

                    final_css += &format!(
                        "
                {} {{
                    margin-top: {};
                    margin-bottom: {};
                    margin-left: {};
                    margin-right: {};

                    border-style: {};
                    border-color: {};
                    border-width: {};
                    border-radius: {};
                    padding: {};
                }}
                ",
                        class,
                        properties.margin_top + "px",
                        properties.margin_bottom + "px",
                        properties.margin_left + "px",
                        properties.margin_right + "px",
                        properties.border_style,
                        properties.border_color,
                        properties.border_width,
                        properties.border_radius,
                        properties.padding
                    );
                }

                load_css_into_app(&final_css);
            }
        }
    }
}

// select
impl Styleable for gtk::DropDown {
    fn style(&self) {
        let guard = CSS_RULES.lock().unwrap();
        if let Some(css) = guard.as_ref() {
            let mut classes = self.css_classes();
            let mut final_css = "".to_string();

            classes.push(self.css_name());

            for class in classes {
                if let Some(rules) = css.get(&class.to_string()) {
                    let properties: Properties = get_properties(rules);

                    final_css += &format!(
                        "
                {} {{
                    color: {};
                    background-color: {};
                    font-size: {};
                    font-family: {};

                    margin-top: {};
                    margin-bottom: {};
                    margin-left: {};
                    margin-right: {};

                    border-style: {};
                    border-color: {};
                    border-width: {};
                    border-radius: {};
                    padding: {};
                }}
                ",
                        class,
                        properties.color,
                        properties.background_color,
                        properties.font_size,
                        properties.font_family,
                        properties.margin_top + "px",
                        properties.margin_bottom + "px",
                        properties.margin_left + "px",
                        properties.margin_right + "px",
                        properties.border_style,
                        properties.border_color,
                        properties.border_width,
                        properties.border_radius,
                        properties.padding
                    );
                }

                load_css_into_app(&final_css);
            }
        }
    }
}

// a
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

// div
impl Styleable for gtk::Box {
    fn style(&self) {
        let guard = CSS_RULES.lock().unwrap();

        if let Some(css) = guard.as_ref() {
            let mut classes = self.css_classes();
            let mut final_css = "".to_string();

            classes.push(self.css_name());

            for class in classes {
                if let Some(rules) = css.get(&class.to_string()) {
                    let properties: Properties = get_properties(rules);

                    self.set_spacing(properties.gap);

                    match properties.direction.as_str() {
                        "column" => self.set_orientation(gtk::Orientation::Vertical),
                        "row" => self.set_orientation(gtk::Orientation::Horizontal),
                        _ => {}
                    };

                    match properties.align_items.as_str() {
                        "fill" => self.set_halign(gtk::Align::Fill),
                        "start" => self.set_halign(gtk::Align::Start),
                        "end" => self.set_halign(gtk::Align::End),
                        "center" => self.set_halign(gtk::Align::Center),
                        _ => {}
                    };

                    final_css += &format!(
                        "
                {} {{
                    color: {};
                    background-color: {};
                    font-size: {};
                    font-family: {};

                    margin-top: {};
                    margin-bottom: {};
                    margin-left: {};
                    margin-right: {};

                    border-style: {};
                    border-color: {};
                    border-width: {};
                    border-radius: {};
                    padding: {};
                }}
                ",
                        class,
                        properties.color,
                        properties.background_color,
                        properties.font_size,
                        properties.font_family,
                        properties.margin_top + "px",
                        properties.margin_bottom + "px",
                        properties.margin_left + "px",
                        properties.margin_right + "px",
                        properties.border_style,
                        properties.border_color,
                        properties.border_width,
                        properties.border_radius,
                        properties.padding
                    );
                }

                load_css_into_app(&final_css);
            }
        }
    }
}

// textarea
impl Styleable for gtk::TextView {
    fn style(&self) {
        let guard = CSS_RULES.lock().unwrap();
        if let Some(css) = guard.as_ref() {
            let mut classes = self.css_classes();
            let mut final_css = "".to_string();

            classes.push(self.css_name());

            for class in classes {
                if let Some(rules) = css.get(&class.to_string()) {
                    let properties: Properties = get_properties(rules);

                    let width = properties.width;
                    let height = properties.height;

                    self.set_size_request(width, height);
                    final_css += &format!(
                        "
                {} {{
                    color: {};
                    background-color: {};
                    font-size: {};
                    font-family: {};

                    margin-top: {};
                    margin-bottom: {};
                    margin-left: {};
                    margin-right: {};

                    border-style: {};
                    border-color: {};
                    border-width: {};
                    border-radius: {};
                    padding: {};
                }}
                ",
                        class,
                        properties.color,
                        properties.background_color,
                        properties.font_size,
                        properties.font_family,
                        properties.margin_top + "px",
                        properties.margin_bottom + "px",
                        properties.margin_left + "px",
                        properties.margin_right + "px",
                        properties.border_style,
                        properties.border_color,
                        properties.border_width,
                        properties.border_radius,
                        properties.padding
                    );
                }

                load_css_into_app(&final_css);
            }
        }
    }
}

// hr
impl Styleable for gtk::Separator {
    fn style(&self) {
        let guard = CSS_RULES.lock().unwrap();
        if let Some(css) = guard.as_ref() {
            let mut classes = self.css_classes();
            let mut final_css = "".to_string();

            classes.push(self.css_name());
            for class in classes {
                if let Some(rules) = css.get(&class.to_string()) {
                    let properties: Properties = get_properties(rules);

                    final_css += &format!(
                        "
                {} {{
                    color: {};
                    background-color: {};
                    font-size: {};
                    font-family: {};

                    margin-top: {};
                    margin-bottom: {};
                    margin-left: {};
                    margin-right: {};

                    border-style: {};
                    border-color: {};
                    border-width: {};
                    border-radius: {};
                    padding: {};
                }}
                ",
                        class,
                        properties.color,
                        properties.background_color,
                        properties.font_size,
                        properties.font_family,
                        properties.margin_top + "px",
                        properties.margin_bottom + "px",
                        properties.margin_left + "px",
                        properties.margin_right + "px",
                        properties.border_style,
                        properties.border_color,
                        properties.border_width,
                        properties.border_radius,
                        properties.padding
                    );
                }

                load_css_into_app(&final_css);
            }
        }
    }
}

// img
impl Styleable for gtk::Picture {
    fn style(&self) {
        let guard = CSS_RULES.lock().unwrap();
        if let Some(css) = guard.as_ref() {
            let mut classes = self.css_classes();
            let mut final_css = "".to_string();

            classes.push(self.css_name());

            for class in classes {
                if let Some(rules) = css.get(&class.to_string()) {
                    let properties: Properties = get_properties(rules);

                    final_css += &format!(
                        "
                {} {{
                    color: {};
                    background-color: {};
                    font-size: {};
                    font-family: {};

                    margin-top: {};
                    margin-bottom: {};
                    margin-left: {};
                    margin-right: {};

                    border-style: {};
                    border-color: {};
                    border-width: {};
                    border-radius: {};
                    padding: {};
                }}
                ",
                        class,
                        properties.color,
                        properties.background_color,
                        properties.font_size,
                        properties.font_family,
                        properties.margin_top + "px",
                        properties.margin_bottom + "px",
                        properties.margin_left + "px",
                        properties.margin_right + "px",
                        properties.border_style,
                        properties.border_color,
                        properties.border_width,
                        properties.border_radius,
                        properties.padding
                    );
                }

                load_css_into_app(&final_css);
            }
        }
    }
}

// input
impl Styleable for gtk::Entry {
    fn style(&self) {
        let guard = CSS_RULES.lock().unwrap();
        if let Some(css) = guard.as_ref() {
            let mut classes = self.css_classes();
            let mut final_css = "".to_string();

            classes.push(self.css_name());

            for class in classes {
                if let Some(rules) = css.get(&class.to_string()) {
                    let properties: Properties = get_properties(rules);

                    let width = properties.width;
                    let height = properties.height;

                    self.set_size_request(width, height);

                    final_css += &format!(
                        "
                {} {{
                    color: {};
                    background-color: {};
                    font-size: {};
                    font-family: {};

                    margin-top: {};
                    margin-bottom: {};
                    margin-left: {};
                    margin-right: {};

                    border-style: {};
                    border-color: {};
                    border-width: {};
                    border-radius: {};
                    padding: {};
                }}
                ",
                        class,
                        properties.color,
                        properties.background_color,
                        properties.font_size,
                        properties.font_family,
                        properties.margin_top + "px",
                        properties.margin_bottom + "px",
                        properties.margin_left + "px",
                        properties.margin_right + "px",
                        properties.border_style,
                        properties.border_color,
                        properties.border_width,
                        properties.border_radius,
                        properties.padding
                    );
                }

                load_css_into_app(&final_css);
            }
        }
    }
}

// button
impl Styleable for gtk::Button {
    fn style(&self) {
        let guard = CSS_RULES.lock().unwrap();
        if let Some(css) = guard.as_ref() {
            let mut classes = self.css_classes();
            let mut final_css = "".to_string();

            classes.push(self.css_name());

            for class in classes {
                if let Some(rules) = css.get(&class.to_string()) {
                    let properties: Properties = get_properties(rules);

                    final_css += &format!(
                        "
                {} {{
                    color: {};
                    background-color: {};
                    font-size: {};
                    font-family: {};

                    margin-top: {};
                    margin-bottom: {};
                    margin-left: {};
                    margin-right: {};

                    border-style: {};
                    border-color: {};
                    border-width: {};
                    border-radius: {};
                    padding: {};
                }}
                ",
                        class,
                        properties.color,
                        properties.background_color,
                        properties.font_size,
                        properties.font_family,
                        properties.margin_top + "px",
                        properties.margin_bottom + "px",
                        properties.margin_left + "px",
                        properties.margin_right + "px",
                        properties.border_style,
                        properties.border_color,
                        properties.border_width,
                        properties.border_radius,
                        properties.padding
                    );
                }

                load_css_into_app(&final_css);
            }
        }
    }
}

pub(crate) fn load_css(css: String) {
    if let Ok(res) = parser::parse(&css) {
        *CSS_RULES.lock().unwrap() = Some(res);
    } else {
        eprintln!("Failed to parse CSS!");
    }
}

pub(crate) fn reset_css() {
    *CSS_RULES.lock().unwrap() = None;
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

pub(crate) fn load_css_into_app(content: &str) {
    let provider = CssProvider::new();
    provider.load_from_string(content);

    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

// shithole
fn get_properties(rules: &Vec<(String, String)>) -> Properties {
    let direction = get_rule(&rules, "direction", &"row");
    let align_items = get_rule(&rules, "align-items", &"fill");

    let wrap = get_rule(&rules, "wrap", &"nowrap");

    let line_height = get_rule(&rules, "line-height", &"1");
    let font_size = get_rule(&rules, "font-size", &"11px");
    let color = get_rule(&rules, "color", &"#ffffff");
    let background_color = get_rule(&rules, "background-color", &"#242424");
    let font_family = get_rule(&rules, "font-family", &"Noto Sans");
    let font_weight = get_rule(&rules, "font-weight", &"normal");
    let underline = get_rule(&rules, "underline", &"none");
    let underline_color = get_rule(&rules, "underline-color", &"black");
    let overline = get_rule(&rules, "overline", &"none");
    let overline_color = get_rule(&rules, "overline-color", &"black");
    let strikethrough = get_rule(&rules, "strikethrough", &"false");
    let strikethrough_color = get_rule(&rules, "strikethrough-color", &"black");

    let margin_top = get_rule(&rules, "margin-top", "0").replace("px", "");
    let margin_bottom = get_rule(&rules, "margin-bottom", "0").replace("px", "");
    let margin_left = get_rule(&rules, "margin-left", "0").replace("px", "");
    let margin_right = get_rule(&rules, "margin-right", "0").replace("px", "");

    let border_style = get_rule(&rules, "border-style", "none");
    let border_color = get_rule(&rules, "border-color", "black");
    let border_width = get_rule(&rules, "border-width", "0");
    let border_radius = get_rule(&rules, "border-radius", "0");
    let padding = get_rule(&rules, "padding", "0");

    let gap = get_rule(&rules, "gap", "0")
        .replace("px", "")
        .parse::<i32>()
        .unwrap_or(0);

    let width = get_rule(&rules, "width", "auto")
        .replace("px", "")
        .parse::<i32>()
        .unwrap_or(0);
    let height = get_rule(&rules, "height", "auto")
        .replace("px", "")
        .parse::<i32>()
        .unwrap_or(0);

    Properties {
        direction,
        align_items,
        width,
        height,
        line_height,
        wrap,
        color,
        background_color,
        font_family,
        font_weight,
        underline,
        underline_color,
        overline,
        overline_color,
        strikethrough,
        strikethrough_color,
        margin_top,
        margin_bottom,
        margin_left,
        margin_right,
        border_style,
        border_color,
        border_width,
        border_radius,
        padding,
        gap,
        font_size,
    }
}
