use saphyr::{Mapping, Yaml};

pub fn merge<'a>(target: &mut Mapping<'a>, source: &Mapping<'a>) {
    for (key, src_value) in source {
        match target.get_mut(key) {
            Some(dst_value) => match (dst_value, src_value) {
                (Yaml::Mapping(dst_map), Yaml::Mapping(src_map)) => {
                    merge(dst_map, src_map);
                }
                (dst_value, src_value) => {
                    *dst_value = src_value.clone();
                }
            },
            None => {
                target.insert(key.clone(), src_value.clone());
            }
        }
    }
}

#[inline(always)]
pub fn key_of(key: &'_ str) -> Yaml<'_> {
    Yaml::value_from_str(key)
}

trait YamlExt {}
impl YamlExt for Yaml<'_> {}
