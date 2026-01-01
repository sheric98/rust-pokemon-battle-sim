#[macro_export]
macro_rules! ability_map {
    ($name:ident {
        $(
            $ability_name:ident
        ),* $(,)?
    }) => {
        LazyLock::new(|| enum_map!{
            $(
                Ability::$ability_name => Arc::new($ability_name) as Arc<dyn CombinedHandler>,
            )*
        })
    };
}