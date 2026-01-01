#[macro_export]
macro_rules! on_event_priority {
    ($variant:ident { $($pat:tt)* } @ $prio:expr => $handler:expr) => {
        (
            Event::$variant { $($pat)* },
            EventKind::$variant,
            $prio,
            $handler
        )
    };
}

#[macro_export]
macro_rules! on_event {
    ($variant:ident { $($pat:tt)* } => $handler:expr) => {
        on_event_priority!($variant { $($pat)* } @ 0 => $handler)
    };
}

#[macro_export]
macro_rules! impl_event_handler {
    ($name:ident, $( $event:expr ),* ) => {
        impl crate::event::event_handler::EventHandler for $name {
            fn handle(&self, event: &crate::event::event_type::Event, state: &crate::battle::state::BattleState) -> Vec<EventHandlerEffect> {
                match event {
                    $(
                        {
                            let (pat, _, _, handler) = $event;
                            pat => handler(event, state),
                        }
                    )*
                    _ => panic!("Event not handled")
                }
            }
        }

        impl Subscriber<crate::event::event_type::Event> for $name {
            fn subscriptions(&self) -> &'static [crate::event::event_type::EventKind] {
                &[
                    $(
                        {
                            let (_, kind, _, _) = $event;
                            kind
                        }
                    ),*
                ]
            }

            fn priority(&self, kind: &crate::event::event_type::EventKind) -> i32 {
                match kind {
                    $(
                        {
                            let (_, k, prio, _) = $event;
                            k => prio
                        }
                    ),*
                    _ => panic!("Event priority for unhandled event")
                }
            }
        }
    };
}

#[macro_export]
macro_rules! impl_empty_event_handler {
    ($name:ident) => {
        impl crate::event::event_handler::EventHandler for $name {
            fn handle(&self, _: &crate::event::event_type::Event, _: &crate::battle::state::BattleState) -> Vec<crate::event::event_handler_effect::EventHandlerEffect> { 
                panic!("Event not handled")
            }
        }

        impl crate::common::subscriber::Subscriber<crate::event::event_type::Event> for $name {
            fn subscriptions(&self) -> &'static [crate::event::event_type::EventKind] {
                &[]
            }

            fn priority(&self, _: &crate::event::event_type::EventKind) -> i32 {
                panic!("Event priority for unhandled event")
            }
        }
    };
}