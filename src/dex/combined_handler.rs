use crate::{event::event_handler::EventHandler, query::query_handler::QueryHandler};

pub trait CombinedHandler: EventHandler + QueryHandler + Send + Sync {}