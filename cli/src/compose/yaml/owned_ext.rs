use saphyr::{ScalarOwned, YamlOwned};

pub trait YamlOwnedExt {
    fn value_of(value: impl Into<String>) -> YamlOwned {
        YamlOwned::Value(ScalarOwned::String(value.into()))
    }
}

impl YamlOwnedExt for YamlOwned {}
