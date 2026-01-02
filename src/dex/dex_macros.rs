#[macro_export]
macro_rules! handler {
    // events + queries
    (
        $name:ident {
            events { $( $event:expr ),* $(,)? }
            queries { $( $query:expr ),* $(,)? }
        }
    ) => {
        struct $name;
        crate::impl_event_handler!($name, $( $event ),*);
        crate::impl_query_handler!($name, $( $query ),*);
    };

    // only events
    (
        $name:ident {
            events { $( $event:expr ),* $(,)? }
        }
    ) => {
        struct $name;
        crate::impl_event_handler!($name, $( $event ),*);
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

    ($name:ident ( $state_var:tt ) {
        queries {
            $(
                $variant:ident ( $($pat:tt)* ) $([priority = $priority:expr])? => $handler:block
            ),* $(,)?
        }
    }) => {
        struct $name;

        impl crate::dex::combined_handler::CombinedHandler for $name {}

        impl crate::query::query_handler::QueryHandler for $name {
            fn handle(&self, query: &mut crate::query::query::Query, state: &crate::battle::state::BattleState) {
                let $state_var = state;
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
                        crate::query::query::QueryKind::$variant => $($priority)? 0,
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
