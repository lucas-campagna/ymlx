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
