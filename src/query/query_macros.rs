#[macro_export]
macro_rules! on_query {
    ($variant:ident { $($pat:tt)* } @ $prio:expr => $handler:expr) => {
        (
            crate::query::query::Query::$variant { $($pat)* },
            crate::query::query::QueryKind::$variant,
            $prio,
            $handler
        )
    };

    ($variant:ident { $($pat:tt)* } => $handler:expr) => {
        on_query!($variant { $($pat)* } @ 0 => $handler)
    };

    ($variant:ident ( $($pat:tt)* ) @ $prio:expr => $handler:expr) => {
        (
            crate::query::query::Query::$variant,
            crate::query::query::QueryKind::$variant,
            $prio,
            $handler
        )
    };

    ($variant:ident ( $($pat:tt)* ) => $handler:expr) => {
        on_query!($variant ( $($pat)* ) @ 0 => $handler)
    };
}

#[macro_export]
macro_rules! impl_query_handler {
    ($name:ident, $( $query:expr ),* ) => {
        impl crate::query::query_handler::QueryHandler for $name {
            fn handle(&self, query: &crate::query::query::Query, state: &crate::battle::state::BattleState) {
                // match query {
                //     $(
                //         $(query[0]) => $query[3](query, state),
                        
                //         // {
                //         //     let (pat, _, _, handler) = $query;
                //         //     pat => handler(query, state),
                //         // }
                //     )*
                //     _ => panic!("Query not handled")
                // }
                $(
                    let (pat, _, _, handler) = $query;
                    match query {
                        pat => handler(query, state),
                        _ => panic!("a"),
                    }
                    // if matches!(query, pat) {
                    //     handler(query, state);
                    //     return;
                    // }
                )*
                panic!("Query priority for unhandled query")
            }
        }

        impl crate::common::subscriber::Subscriber<crate::query::query::Query> for $name {
            fn subscriptions(&self) -> &'static [crate::query::query::QueryKind] {
                &[
                    $(
                        {
                            let (_, kind, _, _) = $query;
                            kind
                        }
                    ),*
                ]
            }

            fn priority(&self, kind: &crate::query::query::QueryKind) -> i32 {
                $(
                    let (_, k, prio, _) = $query;

                    if matches!(kind, k) {
                        return prio;
                    }
                )*

                panic!("Query priority for unhandled query")

                // match kind {
                //     $(
                //         {
                //             let (_, k, prio, _) = $query;
                //             k => prio
                //         }
                //     ),*
                //     _ => panic!("Query priority for unhandled query")
                // }
            }
        }
    };
}

#[macro_export]
macro_rules! impl_empty_query_handler {
    ($name:ident) => {
        impl QueryHandler for $name {
            fn handle(&self, _: &Query, _: &BattleState) { panic!("Query not handled") }
        }

        impl Subscriber<Query> for $name {
            fn subscriptions(&self) -> &'static [QueryKind] {
                &[]
            }

            fn priority(&self, kind: QueryKind) -> i32 {
                panic!("Query priority for unhandled query")
            }
        }
    };
}
