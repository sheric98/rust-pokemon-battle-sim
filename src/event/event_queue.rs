use crate::event::event_type::Event;

pub struct EventQueue {
    events: Vec<Event>,
}

impl EventQueue {
    pub fn new() -> Self {
        EventQueue { events: Vec::new() }
    }

    pub fn add_event(&mut self, event: Event) {
        self.events.push(event);
    }
}