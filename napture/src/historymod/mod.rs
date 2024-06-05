mod imp;

use glib::Object;
use serde::{Deserialize, Serialize};
use serde_json::Map;

glib::wrapper! {
    pub struct HistoryObject(ObjectSubclass<imp::HistoryObject>);
}

impl HistoryObject {
    pub fn new(url: String, position: i32, date: String) -> Self {
        Object::builder()
            .property("url", url)
            .property("position", position)
            .property("date", date)
            .build()
    }
}

use std::collections::VecDeque;

use crate::set_config;

#[derive(Default, Clone, Serialize, Deserialize, Debug)]
pub(crate) struct HistoryItem {
    pub(crate) position: i32,
    pub(crate) date: String,
    pub(crate) url: String,
}

impl HistoryItem {
    pub(crate) fn new(position: i32, url: String, date: String) -> HistoryItem {
        HistoryItem {
            position,
            url,
            date,
        }
    }
}

#[derive(Default, Clone, Serialize, Deserialize, Debug)]
pub(crate) struct History {
    pub(crate) items: VecDeque<HistoryItem>,
    current_position: usize,
}

impl History {
    pub(crate) fn new() -> History {
        History {
            items: VecDeque::new(),
            current_position: 0,
        }
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub(crate) fn add_to_history(
        &mut self,
        url: String,
        date: String,
        save_to_disk: bool,
    ) {
        while self.items.len() > self.current_position + 1 {
            self.items.pop_back();
        }

        let new_position = self.items.len();
        self.items.push_back(HistoryItem::new(
            new_position as i32,
            url.clone(),
            date.clone(),
        ));
        self.current_position = new_position;

        if save_to_disk {
            let mut map = Map::new();

            map.insert("url".to_owned(), serde_json::Value::String(url));
            map.insert("date".to_owned(), serde_json::Value::String(date));

            set_config("history".to_owned(), serde_json::Value::Object(map), true)
        }
    }

    pub(crate) fn go_back(&mut self) -> Option<&HistoryItem> {
        if self.current_position > 0 {
            self.current_position -= 1;
            println!(
                "Going back in history to: {:?}",
                self.items.get(self.current_position)
            );
            self.items.get(self.current_position)
        } else {
            println!("Already at the beginning of the history.");
            None
        }
    }

    pub(crate) fn go_forward(&mut self) -> Option<&HistoryItem> {
        if self.current_position + 1 < self.items.len() {
            self.current_position += 1;
            println!(
                "Going forward in history to: {:?}",
                self.items.get(self.current_position)
            );
            self.items.get(self.current_position)
        } else {
            println!("Already at the end of the history.");
            None
        }
    }

    pub(crate) fn current(&self) -> Option<&HistoryItem> {
        self.items.get(self.current_position)
    }

    pub(crate) fn on_history_end(&self) -> bool {
        self.current_position + 1 == self.items.len()
    }

    pub(crate) fn on_history_start(&self) -> bool {
        self.current_position == 0
    }
}
