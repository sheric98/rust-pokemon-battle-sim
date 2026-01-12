use std::{collections::HashMap, sync::Arc};

use crate::common::{has_kind::HasKind, subscriber::Subscriber};

pub struct Registry<T, U>
where
    T: HasKind,
    U: ?Sized + Subscriber<T>,
{
    subscribers: HashMap<T::Kind, Vec<Arc<U>>>,
}

impl<T, U> Registry<T, U>
where
    T: HasKind,
    U: ?Sized + Subscriber<T>,
{
    pub fn new() -> Self {
        Self {
            subscribers: HashMap::new(),
        }
    }

    pub fn contains(&self, kind: &T::Kind) -> bool {
        self.subscribers.contains_key(kind)
    }

    pub fn get(&self, kind: &T::Kind) -> &Vec<Arc<U>> {
        &self.subscribers[kind]
    }

    pub fn add_handlers(&mut self, handlers: Vec<Arc<U>>) {
        for handler in handlers {
            self.add_handler(handler);
        }
    }

    pub fn add_handler(&mut self, handler: Arc<U>) {
        handler.as_ref().subscriptions().iter().for_each(|kind| {
            let vec = self.subscribers.entry(*kind).or_insert_with(Vec::new);

            let idx = vec
                .binary_search_by(|s| s.priority(kind).cmp(&handler.priority(kind)).reverse())
                .unwrap_or_else(|i| i);
            vec.insert(idx, handler.clone());
        });
    }

    pub fn remove_handler(&mut self, handler: Arc<U>) {
        handler
            .as_ref()
            .subscriptions()
            .iter()
            .for_each(|event_kind| {
                if let Some(subscribers) = self.subscribers.get_mut(event_kind) {
                    subscribers.retain(|s| !Arc::ptr_eq(s, &handler));
                }
            });
    }

    // TODO: Evaluate if this is the best way to remove handlers
    pub fn remove_handlers(&mut self, handlers: Vec<Arc<U>>) {
        for handler in handlers {
            self.remove_handler(handler);
        }
    }
}
