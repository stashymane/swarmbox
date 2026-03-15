use crate::yaml::YamlOwnedExt;
use saphyr::{MappingOwned, YamlOwned};

pub trait MappingExt {
    fn get_value(&self, key: impl Into<String>) -> Option<&YamlOwned>;
    fn get_value_mut(&mut self, key: impl Into<String>) -> Option<&mut YamlOwned>;
    fn merge_from(&mut self, other: &MappingOwned);
}

impl MappingExt for MappingOwned {
    fn get_value(&self, key: impl Into<String>) -> Option<&YamlOwned> {
        self.get(&YamlOwned::value_of(key))
    }

    fn get_value_mut(&mut self, key: impl Into<String>) -> Option<&mut YamlOwned> {
        self.get_mut(&YamlOwned::value_of(key))
    }

    fn merge_from(&mut self, source: &MappingOwned) {
        for (key, src_value) in source {
            match self.get_mut(key) {
                Some(dst_value) => match (dst_value, src_value) {
                    (YamlOwned::Mapping(dst_map), YamlOwned::Mapping(src_map)) => {
                        dst_map.merge_from(src_map);
                    }
                    (dst_value, src_value) => {
                        *dst_value = src_value.clone();
                    }
                },
                None => {
                    self.insert(key.clone(), src_value.clone());
                }
            }
        }
    }
}
