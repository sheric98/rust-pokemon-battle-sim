#[macro_export]
macro_rules! handler {
    // events + queries
    (
        $name:ident ( $self:ident, $state_var:tt ) {
            events {
                $(
                    $variant_e:ident $( ( $($pat_e:tt)* ) )? $([priority = $priority_e:expr])? => $handler_e:block
                ),* $(,)?
            } $(,)?
            queries {
                $(
                    $variant_q:ident ( $($pat_q:tt)* ) $([priority = $priority_q:expr])? => $handler_q:block
                ),* $(,)?
            }
        }
    ) => {
        struct $name {
            trainer_side: bool,
        }

        impl $name {
            pub fn new(trainer_side: bool) -> Self {
                Self {
                    trainer_side,
                }
            }
        }

        impl crate::dex::combined_handler::CombinedHandler for $name {}

        impl crate::query::query_handler::QueryHandler for $name {
            fn handle(&self, query: &mut crate::query::query::Query, state: &mut crate::battle::state::BattleState) {
                let $state_var = state;
                let $self = self;
                match query {
                    $(
                        crate::query::query::Query::$variant_q($($pat_q)*) => {
                            $handler_q
                        }
                    )*
                    _ => panic!("Query not handled"),
                }
            }
        }

        impl crate::event::event_handler::EventHandler for $name {
            fn handle(&self, event: &crate::event::event_type::Event, state: &mut crate::battle::state::BattleState) -> Vec<crate::event::event_handler_effect::EventHandlerEffect> {
                let $state_var = state;
                let $self = self;
                match event {
                    $(
                        crate::event::event_type::Event::$variant_e $( ( $($pat_e)* ) )? => {
                            $handler_e
                        }
                    )*
                    _ => panic!("Event not handled"),
                }
            }
        }

        impl crate::common::subscriber::Subscriber<crate::query::query::Query> for $name {
            fn subscriptions(&self) -> &'static [crate::query::query::QueryKind] {
                &[
                    $(
                        {
                            crate::query::query::QueryKind::$variant_q
                        }
                    ),*
                ]
            }

            fn priority(&self, kind: &crate::query::query::QueryKind) -> i32 {
                match kind {
                    $(
                        crate::query::query::QueryKind::$variant_q => crate::__priority!($([priority = $priority_q])?),
                    )*
                    _ => panic!("Query priority for unhandled query")
                }
            }
        }

        impl crate::common::subscriber::Subscriber<crate::event::event_type::Event> for $name {
            fn subscriptions(&self) -> &'static [crate::event::event_type::EventKind] {
                &[
                    $(
                        {
                            crate::event::event_type::EventKind::$variant_e
                        }
                    ),*
                ]
            }

            fn priority(&self, kind: &crate::event::event_type::EventKind) -> i32 {
                match kind {
                    $(
                        crate::event::event_type::EventKind::$variant_e => crate::__priority!($([priority = $priority_e])?),
                    )*
                    _ => panic!("Event priority for unhandled event")
                }
            }
        }
    };

    // only events
    (
        $name:ident ( $self:ident, $state_var:tt ) {
            events {
                $(
                    $variant_e:ident $( ( $($pat_e:tt)* ) )? $([priority = $priority_e:expr])? => $handler_e:block
                ),* $(,)?
            }
        }
    ) => {
        struct $name {
            trainer_side: bool,
        }

        impl $name {
            pub fn new(trainer_side: bool) -> Self {
                Self {
                    trainer_side,
                }
            }
        }

        impl crate::dex::combined_handler::CombinedHandler for $name {}

        impl crate::event::event_handler::EventHandler for $name {
            fn handle(&self, event: &crate::event::event_type::Event, state: &mut crate::battle::state::BattleState) -> Vec<crate::event::event_handler_effect::EventHandlerEffect> {
                let $state_var = state;
                let $self = self;
                match event {
                    $(
                        crate::event::event_type::Event::$variant_e $( ( $($pat_e)* ) )? => {
                            $handler_e
                        }
                    )*
                    _ => panic!("Event not handled"),
                }
            }
        }

        impl crate::common::subscriber::Subscriber<crate::event::event_type::Event> for $name {
            fn subscriptions(&self) -> &'static [crate::event::event_type::EventKind] {
                &[
                    $(
                        {
                            crate::event::event_type::EventKind::$variant_e
                        }
                    ),*
                ]
            }

            fn priority(&self, kind: &crate::event::event_type::EventKind) -> i32 {
                match kind {
                    $(
                        crate::event::event_type::EventKind::$variant_e => crate::__priority!($([priority = $priority_e])?),
                    )*
                    _ => panic!("Event priority for unhandled event")
                }
            }
        }

        crate::impl_empty_query_handler!($name);
    };

    // only queries
    // (
    //     $name:ident {
    //         queries { $( $query:expr ),* $(,)? }
    //     }
    // ) => {

    //     struct $name;
    //     crate::impl_empty_event_handler!($name);
    //     crate::impl_query_handler!($name, $( $query ),*);
    // };

    ($name:ident ( $self:ident, $state_var:tt ) {
        queries {
            $(
                $variant:ident ( $($pat:tt)* ) $([priority = $priority:expr])? => $handler:block
            ),* $(,)?
        }
    }) => {
        struct $name {
            trainer_side: bool,
        }

        impl $name {
            pub fn new(trainer_side: bool) -> Self {
                Self {
                    trainer_side,
                }
            }
        }

        impl crate::dex::combined_handler::CombinedHandler for $name {}

        impl crate::query::query_handler::QueryHandler for $name {
            fn handle(&self, query: &mut crate::query::query::Query, state: &mut crate::battle::state::BattleState) {
                let $state_var = state;
                let $self = self;
                match query {
                    $(
                        crate::query::query::Query::$variant($($pat)*) => {
                            $handler
                        }
                    )*
                    _ => panic!("Query not handled"),
                }
            }
        }

        impl crate::common::subscriber::Subscriber<crate::query::query::Query> for $name {
            fn subscriptions(&self) -> &'static [crate::query::query::QueryKind] {
                &[
                    $(
                        {
                            crate::query::query::QueryKind::$variant
                        }
                    ),*
                ]
            }

            fn priority(&self, kind: &crate::query::query::QueryKind) -> i32 {
                match kind {
                    $(
                        crate::query::query::QueryKind::$variant => crate::__priority!($([priority = $priority])?),
                    )*
                    _ => panic!("Query priority for unhandled query")
                }
            }
        }

        crate::impl_empty_event_handler!($name);
    };

    // neither (empty ability)
    (
        $name:ident
    ) => {
        struct $name;
        crate::impl_empty_event_handler!($name);
        crate::impl_empty_query_handler!($name);
    };
}

#[macro_export]
macro_rules! __priority {
    ([priority = $p:expr]) => {
        $p
    };
    () => {
        0
    };
}
