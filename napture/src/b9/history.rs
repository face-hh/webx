use std::collections::VecDeque;

#[derive(Debug)]
pub(crate) struct HistoryItem {
    pub(crate) position: usize,
    pub(crate) url: String,
}

impl HistoryItem {
    pub(crate) fn new(position: usize, url: String) -> HistoryItem {
        HistoryItem { position, url }
    }
}

pub(crate) struct History {
    items: VecDeque<HistoryItem>,
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

    pub(crate) fn add_to_history(&mut self, url: String) {
        while self.items.len() > self.current_position + 1 {
            self.items.pop_back();
        }

        let new_position = self.items.len();
        self.items.push_back(HistoryItem::new(new_position, url));
        self.current_position = new_position;

        println!("Added to history: {:?}", self.items.back().unwrap());
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
