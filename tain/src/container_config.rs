use std::collections::HashMap;

use typed_builder::TypedBuilder;

#[derive(Debug)]
pub struct Mount {
    pub host:      String,
    pub container: String,
}

#[derive(Debug)]
pub struct Port {
    pub host:      u16,
    pub container: u16,
}

impl Port {
    pub fn postgres() -> Self {
        Self {
            host:      5432,
            container: 5432,
        }
    }
}

#[derive(TypedBuilder, Debug)]
pub struct ContainerConfig {
    #[builder(setter(into))]
    pub name: String,

    #[builder(setter(into))]
    pub image: String,

    pub port: Port,

    #[builder(default, setter(strip_option))]
    pub mount: Option<Mount>,

    #[builder(default, setter(into))]
    pub env: HashMap<String, String>,
}
