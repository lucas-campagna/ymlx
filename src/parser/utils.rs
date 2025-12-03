use rust_yaml::Value;

pub fn is_template(name: &str) -> bool{
    name.starts_with("$")
}

enum GetFieldError {
    KeyNotFound,
    InvalidType,
}

pub fn get_field<'a>(value: &'a Value, key: &str) -> Result<&'a Value, GetFieldError> {
    match value{
        Value::Mapping(m) =>
            m.get(&Value::String(String::from(key))).ok_or(GetFieldError::KeyNotFound),
        _ => Err(GetFieldError::InvalidType)
    }
}