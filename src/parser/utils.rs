pub fn is_template(name: &str) -> bool {
    name.starts_with("$")
}

pub fn get_template_name(name: &str) -> String {
    "$".to_string() + name
}