use crate::event::event_type::EventType;

pub struct EventQueue {
    events: Vec<EventType>,
}

impl EventQueue {
    pub fn new() -> Self {
        EventQueue { events: Vec::new() }
    }

    pub fn add_event(&mut self, event: EventType) {
        self.events.push(event);
    }
}