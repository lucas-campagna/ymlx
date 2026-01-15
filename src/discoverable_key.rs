pub struct DiscoverableKey<'a>(pub &'a str);

impl<'a> DiscoverableKey<'a> {
    #[allow(dead_code)]
    pub fn matches(&self, key: &str) -> bool {
        let name = self.0;
        format!("{}!", name) == key
            || (name[0..=0].to_uppercase() + &name[1..]) == key
            || format!("yx-{}", name) == key
    }
    pub fn keys(&self) -> [String; 3] {
        let name = self.0;
        [
            format!("{}!", name),
            (name[0..=0].to_uppercase() + &name[1..]),
            format!("yx-{}", name),
        ]
    }
}
