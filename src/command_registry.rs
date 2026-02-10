#![allow(unused)]

#[derive(Debug, Clone)]
pub struct CommandModule {
    pub name: String,
    pub builder_path: &'static str,
    pub category: &'static str,
    pub traits: Vec<&'static str>,
    pub supported_flags: Vec<&'static str>,
}


macro_rules! register_command {
    ($name:expr, $builder:path, $category:expr) => {
        CommandModule {
            name: $name,
            builder_path: stringify!($builder),
            category: $category,
            supported_flags: $builder::flags(),
        }
    };
}


