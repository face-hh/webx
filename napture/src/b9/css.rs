// this code is held together with hopes, dreams and glue
use crate::parser;
use std::{collections::HashMap, sync::Mutex};

use glib::GString;
use gtk::{gdk::Display, prelude::*, CssProvider};

type CssRules = Mutex<Option<HashMap<String, Vec<(String, String)>>>>;

static CSS_RULES: CssRules = Mutex::new(None); // shut the fuck up

static DEFAULT_CSS: &str = r"
body {
    gap: 10;
    background-color: transparent;
    direction: column;
    align-items: fill;
}
h1 {
    font-size: 24pt;
}

h2 {
    font-size: 22pt;
}

h3 {
    font-size: 20pt;
}

h4 {
    font-size: 18pt;
}

h5 {
    font-size: 16pt;
}

h6 {
    font-size: 14pt;
}

a {
    border: none;
    color: #67B7D1;
    text-decoration: underline;
}

input {
    padding: 5px;
    border-color: #616161;
    border-width: 1px;
    border-style: solid;
    border-radius: 12px;

}

textarea {
    padding: 5px;
    border-color: #616161;
    border-width: 1px;
    border-style: solid;
    border-radius: 12px;

    width: 400px;
    height: 100px;
}
";

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
    opacity: f64,
}

pub(crate) trait Styleable {
    fn style(&self) -> String;
}

// h1, h2, h3, h4, h5, h6, p
impl Styleable for gtk::Label {
    fn style(&self) -> String {
        let guard = match CSS_RULES.lock() {
            Ok(guard) => guard,
            Err(_) => {
                println!("FATAL: failed to lock CSS_RULES mutex! Aborting function at GtkLabel.");
                return String::new();
            }
        };

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

                    if properties.font_size == "11px" {
                        match self.css_name().as_str() {
                            "h1" => properties.font_size = "24px".to_string(),
                            "h2" => properties.font_size = "22px".to_string(),
                            "h3" => properties.font_size = "20px".to_string(),
                            "h4" => properties.font_size = "18px".to_string(),
                            "h5" => properties.font_size = "16px".to_string(),
                            "h6" => properties.font_size = "14px".to_string(),
                            _ => {}
                        }
                    }

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

                    self.set_margin_top(properties.margin_top.parse::<i32>().unwrap_or(0));
                    self.set_margin_bottom(properties.margin_bottom.parse::<i32>().unwrap_or(0));
                    self.set_margin_start(properties.margin_left.parse::<i32>().unwrap_or(0));
                    self.set_margin_end(properties.margin_right.parse::<i32>().unwrap_or(0));

                    self.set_opacity(properties.opacity);

                    final_css += &format!(
                        "
                .{} {{
                    border-style: {};
                    border-color: {};
                    border-width: {};
                    border-radius: {};
                    padding: {};
                    background-color: {};
                }}
                ",
                        class,
                        properties.border_style,
                        properties.border_color,
                        properties.border_width,
                        properties.border_radius,
                        properties.padding,
                        properties.background_color,
                    );
                }
            }

            final_css
        } else {
            String::new()
        }
    }
}

// select
impl Styleable for gtk::DropDown {
    fn style(&self) -> String {
        let guard = match CSS_RULES.lock() {
            Ok(guard) => guard,
            Err(_) => {
                println!(
                    "FATAL: failed to lock CSS_RULES mutex! Aborting function at GtkDropDown."
                );
                return String::new();
            }
        };

        if let Some(css) = guard.as_ref() {
            let mut classes = self.css_classes();
            let mut final_css = "".to_string();

            classes.push(self.css_name());

            for class in classes {
                if let Some(rules) = css.get(&class.to_string()) {
                    let properties: Properties = get_properties(rules);

                    self.set_opacity(properties.opacity);

                    final_css += &format!(
                        "
                .{} {{
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
                        properties.padding,
                    );
                }
            }

            final_css
        } else {
            String::new()
        }
    }
}

// a
impl Styleable for gtk::LinkButton {
    fn style(&self) -> String {
        let lbl = gtk::Label::builder()
            .css_name("a")
            .label(
                if let Some(child) = self.child() {
                    if let Some(label) = child.downcast_ref::<gtk::Label>() {
                        label.label()
                    } else {
                        println!("FATAL: GtkLinkButton child is not a GtkLabel! Silencing the error by returning empty string.");
                        GString::new()
                    }
                } else {
                    println!("FATAL: GtkLinkButton has no child! Silencing the error by returning empty string.");
                    GString::new()
                }
            )
            .build();
        self.set_child(Some(&lbl));

        Styleable::style(&lbl)
    }
}

// div
impl Styleable for gtk::Box {
    fn style(&self) -> String {
        let guard = match CSS_RULES.lock() {
            Ok(guard) => guard,
            Err(_) => {
                println!("FATAL: failed to lock CSS_RULES mutex! Aborting function at GtkBox.");
                return String::new();
            }
        };

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

                    self.set_margin_top(properties.margin_top.parse::<i32>().unwrap_or(0));
                    self.set_margin_bottom(properties.margin_bottom.parse::<i32>().unwrap_or(0));
                    self.set_margin_start(properties.margin_left.parse::<i32>().unwrap_or(0));
                    self.set_margin_end(properties.margin_right.parse::<i32>().unwrap_or(0));

                    self.set_opacity(properties.opacity);

                    final_css += &compute_styling(class, &properties);
                }
            }

            final_css
        } else {
            String::new()
        }
    }
}

impl Styleable for gtk::ScrolledWindow {
    fn style(&self) -> String {
        let guard = match CSS_RULES.lock() {
            Ok(guard) => guard,
            Err(_) => {
                println!("FATAL: failed to lock CSS_RULES mutex! Aborting function at GtkBox.");
                return String::new();
            }
        };

        if let Some(css) = guard.as_ref() {
            let mut classes = self.css_classes();
            let mut final_css = "".to_string();

            classes.push(self.css_name());

            for class in classes {
                if let Some(rules) = css.get(&class.to_string()) {
                    let properties: Properties = get_properties(rules);

                    final_css += &format!(
                        "
                .{} {{
                    background-color: {};
                }}
                ",
                        class, properties.background_color,
                    );
                }
            }

            final_css
        } else {
            String::new()
        }
    }
}
// textarea
impl Styleable for gtk::TextView {
    fn style(&self) -> String {
        let guard = match CSS_RULES.lock() {
            Ok(guard) => guard,
            Err(_) => {
                println!(
                    "FATAL: failed to lock CSS_RULES mutex! Aborting function at GtkTextView."
                );
                return String::new();
            }
        };

        if let Some(css) = guard.as_ref() {
            let mut classes = self.css_classes();
            let mut final_css = "".to_string();

            classes.push(self.css_name());

            for class in classes {
                if let Some(rules) = css.get(&class.to_string()) {
                    let properties: Properties = get_properties(rules);

                    let width = properties.width;
                    let height = properties.height;

                    if width > 0 || height > 0 {
                        let normalized_width = if width > 0 { width } else { -1 };
                        let normalized_height = if height > 0 { height } else { -1 };

                        self.set_size_request(normalized_width, normalized_height);
                    }

                    self.set_margin_top(properties.margin_top.parse::<i32>().unwrap_or(0));
                    self.set_margin_bottom(properties.margin_bottom.parse::<i32>().unwrap_or(0));
                    self.set_margin_start(properties.margin_left.parse::<i32>().unwrap_or(0));
                    self.set_margin_end(properties.margin_right.parse::<i32>().unwrap_or(0));

                    self.set_opacity(properties.opacity);

                    final_css += &compute_styling(class, &properties);
                }
            }

            final_css
        } else {
            String::new()
        }
    }
}

// hr
impl Styleable for gtk::Separator {
    fn style(&self) -> String {
        // hr won't support customization.
        String::new()
    }
}

// img
impl Styleable for gtk::Picture {
    fn style(&self) -> String {
        let guard = match CSS_RULES.lock() {
            Ok(guard) => guard,
            Err(_) => {
                println!("FATAL: failed to lock CSS_RULES mutex! Aborting function at GtkPicture.");
                return String::new();
            }
        };

        if let Some(css) = guard.as_ref() {
            let mut classes = self.css_classes();
            let mut final_css = "".to_string();

            classes.push(self.css_name());

            for class in classes {
                if let Some(rules) = css.get(&class.to_string()) {
                    let properties: Properties = get_properties(rules);

                    self.set_margin_top(properties.margin_top.parse::<i32>().unwrap_or(0));
                    self.set_margin_bottom(properties.margin_bottom.parse::<i32>().unwrap_or(0));
                    self.set_margin_start(properties.margin_left.parse::<i32>().unwrap_or(0));
                    self.set_margin_end(properties.margin_right.parse::<i32>().unwrap_or(0));

                    self.set_opacity(properties.opacity);

                    final_css += &compute_styling(class, &properties);
                }
            }

            final_css
        } else {
            String::new()
        }
    }
}

// input
impl Styleable for gtk::Entry {
    fn style(&self) -> String {
        let guard = match CSS_RULES.lock() {
            Ok(guard) => guard,
            Err(_) => {
                println!("FATAL: failed to lock CSS_RULES mutex! Aborting function at GtkEntry.");
                return String::new();
            }
        };

        if let Some(css) = guard.as_ref() {
            let mut classes = self.css_classes();
            let mut final_css = "".to_string();

            classes.push(self.css_name());

            for class in classes {
                if let Some(rules) = css.get(&class.to_string()) {
                    let properties: Properties = get_properties(rules);

                    let width = properties.width;
                    let height = properties.height;
                    
                    if width > 0 || height > 0 {
                        let normalized_width = if width > 0 { width } else { -1 };
                        let normalized_height = if height > 0 { height } else { -1 };

                        self.set_size_request(normalized_width, normalized_height);
                    }

                    self.set_margin_top(properties.margin_top.parse::<i32>().unwrap_or(0));
                    self.set_margin_bottom(properties.margin_bottom.parse::<i32>().unwrap_or(0));
                    self.set_margin_start(properties.margin_left.parse::<i32>().unwrap_or(0));
                    self.set_margin_end(properties.margin_right.parse::<i32>().unwrap_or(0));

                    self.set_opacity(properties.opacity);

                    final_css += &compute_styling(class, &properties);
                }
            }

            final_css
        } else {
            String::new()
        }
    }
}

// button
impl Styleable for gtk::Button {
    fn style(&self) -> String {
        let guard = match CSS_RULES.lock() {
            Ok(guard) => guard,
            Err(_) => {
                println!("FATAL: failed to lock CSS_RULES mutex! Aborting function at GtkButton.");
                return String::new();
            }
        };

        if let Some(css) = guard.as_ref() {
            let mut classes = self.css_classes();
            let mut final_css = "".to_string();

            classes.push(self.css_name());

            for class in classes {
                if let Some(rules) = css.get(&class.to_string()) {
                    let properties: Properties = get_properties(rules);

                    self.set_margin_top(properties.margin_top.parse::<i32>().unwrap_or(0));
                    self.set_margin_bottom(properties.margin_bottom.parse::<i32>().unwrap_or(0));
                    self.set_margin_start(properties.margin_left.parse::<i32>().unwrap_or(0));
                    self.set_margin_end(properties.margin_right.parse::<i32>().unwrap_or(0));
                    
                    self.set_opacity(properties.opacity);

                    final_css += &compute_styling(class, &properties);
                }
            }

            final_css
        } else {
            String::new()
        }
    }
}

pub(crate) fn load_css(css: String) {
    let css_: String = DEFAULT_CSS.to_string() + &css;

    if let Ok(res) = parser::parse(&css_) {
        match CSS_RULES.lock() {
            Ok(mut rules) => {
                let mut converted_res: HashMap<String, Vec<(String, String)>> = HashMap::new();

                for (key, inner_map) in res {
                    let mut inner_vec: Vec<(String, String)> = Vec::new();

                    for (inner_key, inner_value) in inner_map {
                        inner_vec.push((inner_key, inner_value));
                    }

                    converted_res.insert(key, inner_vec);
                }

                *rules = Some(converted_res);
            }
            Err(poisoned_error) => {
                eprintln!(
                    "FATAL: Failed to acquire lock on CSS_RULES: {:?}",
                    poisoned_error
                );
            }
        }
    } else {
        eprintln!("Failed to parse CSS!");
    }
}

pub(crate) fn reset_css() -> String {
    match CSS_RULES.lock() {
        Ok(mut rules) => {
            *rules = None;
        }
        Err(poisoned_error) => {
            eprintln!(
                "FATAL: Failed to acquire lock on CSS_RULES while resetting: {:?}",
                poisoned_error
            );
        }
    }

    DEFAULT_CSS.to_string()
}

fn get_rule(rules: &[(String, String)], property: &str, default_value: &str) -> String {
    rules
        .iter()
        .find(|(name, _)| name.as_str() == property)
        .map(|(_, value)| value.as_str())
        .unwrap_or(default_value)
        .to_owned()
}

pub(crate) fn load_css_into_app(content: &str) -> CssProvider {
    let provider = CssProvider::new();

    provider.load_from_string(content);

    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    provider
}

fn compute_styling(class: GString, properties: &Properties) -> String {
    let mut borders = String::new();

    if properties.border_color != "black" {
        borders.push_str(&format!("border-color: {};", properties.border_color));
    }
    if properties.border_width != "0" {
        borders.push_str(&format!("border-width: {};", properties.border_width));
    }
    if properties.border_radius != "0" {
        borders.push_str(&format!("border-radius: {};", properties.border_radius));
    }
    if properties.border_style != "none" {
        borders.push_str(&format!("border-style: {};", properties.border_style));
    }
    
    format!(
        "
.{} {{
    color: {};
    background-color: {};
    font-size: {};
    font-family: {};

    {}
    padding: {};
}}
",
        class,
        properties.color,
        properties.background_color,
        properties.font_size,
        properties.font_family,
        borders,
        properties.padding,
    )
}

// shithole
fn get_properties(rules: &[(String, String)]) -> Properties {
    let direction = get_rule(rules, "direction", "row");
    let align_items = get_rule(rules, "align-items", "fill");

    let wrap = get_rule(rules, "wrap", "nowrap");

    let line_height = get_rule(rules, "line-height", "1");
    let font_size = get_rule(rules, "font-size", "11px");
    let color = get_rule(rules, "color", "#ffffff");
    let background_color = get_rule(rules, "background-color", "transparent");
    let font_family = get_rule(rules, "font-family", "Noto Sans");
    let font_weight = get_rule(rules, "font-weight", "normal");
    let underline = get_rule(rules, "underline", "none");
    let underline_color = get_rule(rules, "underline-color", "black");
    let overline = get_rule(rules, "overline", "none");
    let overline_color = get_rule(rules, "overline-color", "black");
    let strikethrough = get_rule(rules, "strikethrough", "false");
    let strikethrough_color = get_rule(rules, "strikethrough-color", "black");

    let margin_top = get_rule(rules, "margin-top", "0").replace("px", "");
    let margin_bottom = get_rule(rules, "margin-bottom", "0").replace("px", "");
    let margin_left = get_rule(rules, "margin-left", "0").replace("px", "");
    let margin_right = get_rule(rules, "margin-right", "0").replace("px", "");

    let border_style = get_rule(rules, "border-style", "none");
    let border_color = get_rule(rules, "border-color", "black");
    let border_width = get_rule(rules, "border-width", "0");
    let border_radius = get_rule(rules, "border-radius", "0");
    let padding = get_rule(rules, "padding", "0");

    let gap = get_rule(rules, "gap", "0")
        .replace("px", "")
        .parse::<i32>()
        .unwrap_or(0);

    let width = get_rule(rules, "width", "auto")
        .replace("px", "")
        .parse::<i32>()
        .unwrap_or(0);
    let height = get_rule(rules, "height", "auto")
        .replace("px", "")
        .parse::<i32>()
        .unwrap_or(0);
    let opacity = get_rule(rules, "opacity", "1.0")
    .parse::<f64>()
    .unwrap_or(1.0);

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
        opacity
    }
}
