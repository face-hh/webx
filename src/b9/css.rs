use std::fs;

use gtk::prelude::*;

use ::lewp_css::Stylesheet;
use lewp_css::domain::{CssRule, HasCssRules, StyleRule};

use crate::parser;
pub(crate) trait Styleable {
    fn style(&self);
}

impl Styleable for gtk::Label {
    fn style(&self) {
        self.set_opacity(0.3);
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

pub(crate) fn get_css() {
    let stylesheet_utf8_string = fs::read_to_string("test/styles.css").unwrap();
    let res = parser::parse(&stylesheet_utf8_string).unwrap();

    println!("{:?}", res);
}

pub(crate) fn perform_styling<T: Styleable>(_element: &html_parser::Element, styleable: &T) {
    styleable.style();
}
