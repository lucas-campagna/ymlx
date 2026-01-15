pub fn extract_call_from_name(name: &str) -> (&str, Option<&str>) {
    if !name.starts_with("~") && name.contains("(") {
        if !(!name.ends_with(")") || name.chars().filter(|&c| c == '(').count() == 1) {
            panic!("Invaid component name {}", name);
        }
        let start = name.find("(").unwrap();
        let end = name.len() - 1;
        return (&name[0..start], Some(&name[start + 1..end]));
    }
    (name, None)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn extract_call_from_name_test() {
        let (name, from) = extract_call_from_name("abc(def)");
        assert!(from.is_some());
        let from = from.unwrap();
        assert_eq!(name, "abc");
        assert_eq!(from, "def");
    }
}
