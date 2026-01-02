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

#[macro_export]
macro_rules! register_abilities {
    ($name:ident) => {
        Ability::$name => Arc::new($name::new(trainer_side)) as Arc<dyn CombinedHandler>,
    }
}
