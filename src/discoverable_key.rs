use indexmap::IndexMap;

use crate::Value;

pub struct DiscoverableKey<'a>(&'a IndexMap<String, Value>);

impl<'a> DiscoverableKey<'a> {
    pub fn get(&self, key: &str) -> Option<&'a Value> {
        let keys = Self::build_keys(key);
        assert!(self.0.keys().filter(|k| keys.contains(*k)).count() < 2);

        let mut result = None;
        for key in keys.as_ref() {
            if let Some(value) = self.0.get(key) {
                result = Some(value);
                break;
            }
        }
        result
    }
    pub fn clear_and_get(&self, key: &str) -> Option<&'a Value> {
        if key.starts_with("yx-") {
            self.get(&key[3..])
        } else if key.ends_with("!") {
            self.get(&key[..key.len() - 1])
        } else if key.chars().next().map_or(false, |c| c.is_uppercase()) {
            self.get(key)
        } else {
            None
        }
    }
    fn build_keys(key: &str) -> [String; 3] {
        [
            format!("{}!", key),
            (key[0..=0].to_uppercase() + &key[1..]),
            format!("yx-{}", key),
        ]
    }
}

impl<'a> From<&'a IndexMap<String, Value>> for DiscoverableKey<'a> {
    fn from(map: &'a IndexMap<String, Value>) -> Self {
        DiscoverableKey(map)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn build_discoverable_component_names_test() {
        {
            let keys = DiscoverableKey::build_keys("from");
            assert!(keys[0].contains("from!"));
            assert!(keys[1].contains("From"));
            assert!(keys[2].contains("yx-from"));
        }
        {
            let keys = DiscoverableKey::build_keys("MyDb");
            assert!(keys[0].contains("MyDb!"));
            assert!(keys[1].contains("MyDb"));
            assert!(keys[2].contains("yx-MyDb"));
        }
    }
}
