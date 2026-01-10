use std::collections::VecDeque;

use crate::event::event_type::Event;

pub struct EventQueue {
    events: VecDeque<Event>,
}

impl EventQueue {
    pub fn new() -> Self {
        EventQueue {
            events: VecDeque::new(),
        }
    }

    pub fn add_event(&mut self, event: Event) {
        self.events.push_back(event);
    }

    pub fn dequeue(&mut self) -> Option<Event> {
        if self.events.is_empty() {
            None
        } else {
            Some(self.events.pop_front().unwrap())
        }
    }
}
