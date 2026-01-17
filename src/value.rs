use core::fmt;

use indexmap::IndexMap;

use crate::{context::Context, processing_context};

#[derive(Clone, Debug)]
pub enum Value {
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    Sequence(Vec<Value>),
    Mapping(IndexMap<String, Value>),
    LazyEval(LazyEval),
    // Function(fn(Value) -> Value),
    // Generic(Regex),
}

#[derive(Debug, Clone)]
pub struct LazyEval {
    pub code: String,
    pub props: IndexMap<String, Value>,
}

impl Value {
    /// Create a null value
    pub const fn null() -> Self {
        Self::Null
    }

    /// Create a boolean value
    pub const fn bool(b: bool) -> Self {
        Self::Bool(b)
    }

    /// Create an integer value
    pub const fn int(i: i64) -> Self {
        Self::Int(i)
    }

    /// Create a float value
    pub const fn float(f: f64) -> Self {
        Self::Float(f)
    }

    /// Create a string value
    pub fn string(s: impl Into<String>) -> Self {
        Self::String(s.into())
    }

    /// Create an empty sequence
    pub const fn sequence() -> Self {
        Self::Sequence(Vec::new())
    }

    /// Create a sequence with values
    pub const fn sequence_with(values: Vec<Self>) -> Self {
        Self::Sequence(values)
    }

    /// Create an empty mapping
    pub fn mapping() -> Self {
        Self::Mapping(IndexMap::new())
    }

    /// Create a mapping with key-value pairs
    pub fn mapping_with(pairs: Vec<(String, Self)>) -> Self {
        let mut map = IndexMap::new();
        for (key, value) in pairs {
            map.insert(key, value);
        }
        Self::Mapping(map)
    }

    /// Get the type name of this value
    pub const fn type_name(&self) -> &'static str {
        match self {
            Self::Null => "null",
            Self::Bool(_) => "bool",
            Self::Int(_) => "int",
            Self::Float(_) => "float",
            Self::String(_) => "string",
            Self::Sequence(_) => "sequence",
            Self::Mapping(_) => "mapping",
            Self::LazyEval(_) => "lazy_eval",
        }
    }

    /// Check if this value is null
    pub const fn is_null(&self) -> bool {
        matches!(self, Self::Null)
    }

    /// Check if this value is a boolean
    pub const fn is_bool(&self) -> bool {
        matches!(self, Self::Bool(_))
    }

    /// Check if this value is an integer
    pub const fn is_int(&self) -> bool {
        matches!(self, Self::Int(_))
    }

    /// Check if this value is a float
    pub const fn is_float(&self) -> bool {
        matches!(self, Self::Float(_))
    }

    /// Check if this value is a string
    pub const fn is_string(&self) -> bool {
        matches!(self, Self::String(_))
    }

    /// Check if this value is a sequence
    pub const fn is_sequence(&self) -> bool {
        matches!(self, Self::Sequence(_))
    }

    /// Check if this value is a mapping
    pub const fn is_mapping(&self) -> bool {
        matches!(self, Self::Mapping(_))
    }

    /// Check if this value is a number (int or float)
    pub const fn is_number(&self) -> bool {
        matches!(self, Self::Int(_) | Self::Float(_))
    }

    /// Check if this value is a LazyEval
    pub const fn is_lazy_eval(&self) -> bool {
        matches!(self, Self::LazyEval(_))
    }

    /// Get the length of sequences and mappings, None for scalars
    pub fn len(&self) -> Option<usize> {
        match self {
            Self::Sequence(seq) => Some(seq.len()),
            Self::Mapping(map) => Some(map.len()),
            _ => None,
        }
    }

    /// Check if sequences, mappings, or strings are empty
    pub fn is_empty(&self) -> bool {
        match self {
            Self::Sequence(seq) => seq.is_empty(),
            Self::Mapping(map) => map.is_empty(),
            Self::String(s) => s.is_empty(),
            _ => false,
        }
    }

    /// Get this value as a boolean, if possible
    pub const fn as_bool(&self) -> Option<bool> {
        match self {
            Self::Bool(b) => Some(*b),
            _ => None,
        }
    }

    /// Get this value as an integer, if possible
    pub const fn as_int(&self) -> Option<i64> {
        match self {
            Self::Int(i) => Some(*i),
            _ => None,
        }
    }

    /// Get this value as a float, if possible
    pub const fn as_float(&self) -> Option<f64> {
        match self {
            Self::Float(f) => Some(*f),
            Self::Int(i) => Some(*i as f64),
            _ => None,
        }
    }

    /// Get this value as a string reference, if possible
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Self::String(s) => Some(s),
            _ => None,
        }
    }

    /// Get this value as a sequence reference, if possible
    pub const fn as_sequence(&self) -> Option<&Vec<Self>> {
        match self {
            Self::Sequence(seq) => Some(seq),
            _ => None,
        }
    }

    /// Get this value as a mutable sequence reference, if possible
    pub const fn as_sequence_mut(&mut self) -> Option<&mut Vec<Self>> {
        match self {
            Self::Sequence(seq) => Some(seq),
            _ => None,
        }
    }

    /// Get this value as a mapping reference, if possible
    pub const fn as_mapping(&self) -> Option<&IndexMap<String, Self>> {
        match self {
            Self::Mapping(map) => Some(map),
            _ => None,
        }
    }

    /// Get this value as a mutable mapping reference, if possible
    pub const fn as_mapping_mut(&mut self) -> Option<&mut IndexMap<String, Self>> {
        match self {
            Self::Mapping(map) => Some(map),
            _ => None,
        }
    }

    /// Get this value as a mapping reference, if possible
    pub const fn as_lazy_eval(&self) -> Option<&LazyEval> {
        match self {
            Self::LazyEval(lazy_eval) => Some(lazy_eval),
            _ => None,
        }
    }

    /// Get this value as a mutable mapping reference, if possible
    pub const fn as_lazy_eval_mut(&mut self) -> Option<&mut LazyEval> {
        match self {
            Self::LazyEval(lazy_eval) => Some(lazy_eval),
            _ => None,
        }
    }

    /// Index into a sequence or mapping
    pub fn get(&self, index: &Self) -> Option<&Self> {
        match (self, index) {
            (Self::Sequence(seq), Self::Int(i)) => {
                if *i >= 0 && (*i as usize) < seq.len() {
                    seq.get(*i as usize)
                } else {
                    None
                }
            }
            (Self::Mapping(map), Self::String(key)) => map.get(key),
            _ => None,
        }
    }

    /// Convenience method to get a value by string key
    pub fn get_str(&self, key: &str) -> Option<&Self> {
        match self {
            Self::Mapping(map) => map.get(&key.to_string()),
            _ => None,
        }
    }

    /// Get a value by numeric index (for sequences)
    pub fn get_index(&self, index: usize) -> Option<&Self> {
        match self {
            Self::Sequence(seq) => seq.get(index),
            _ => None,
        }
    }

    /// Mutably index into a sequence or mapping
    pub fn get_mut(&mut self, index: &Self) -> Option<&mut Self> {
        match (self, index) {
            (Self::Sequence(seq), Self::Int(i)) => {
                if *i >= 0 && (*i as usize) < seq.len() {
                    seq.get_mut(*i as usize)
                } else {
                    None
                }
            }
            (Self::Mapping(map), Self::String(key)) => map.get_mut(key),
            _ => None,
        }
    }

    /// Force value to mapping
    pub fn force_mapping(value: Value) -> Value {
        match value {
            value if value.is_mapping() => value,
            value => {
                let mut index_map = IndexMap::new();
                index_map.insert("body".to_string(), value);
                Value::Mapping(index_map)
            }
        }
    }

    /// Merges two values
    pub fn extend(&mut self, value: Value) {
        match (self, value) {
            (Value::Mapping(target), Value::Mapping(value)) => {
                for (key, value) in value {
                    if let Some(target_value) = target.get_mut(&key) {
                        target_value.extend(value);
                    } else {
                        target.insert(key, value);
                    }
                }
            }
            (Value::Sequence(target), Value::Sequence(value)) => target.extend(value),
            (Value::Sequence(target), value) => target
                .into_iter()
                .for_each(|target| target.extend(value.clone())),
            (target, Value::Sequence(value)) => {
                value.into_iter().for_each(|value| target.extend(value))
            }
            (target, value) => {
                if target.is_string() {
                    *target = Value::from(LazyEval {
                        code: target.as_str().unwrap().to_owned(),
                        props: Value::force_mapping(value).as_mapping().unwrap().to_owned(),
                    });
                } else if value.is_string() {
                    *target = Value::from(LazyEval {
                        code: value.as_str().unwrap().to_owned(),
                        props: Value::force_mapping(target.to_owned())
                            .as_mapping()
                            .unwrap()
                            .to_owned(),
                    });
                } else if target.is_mapping() {
                    target.extend(Value::force_mapping(value));
                } else if value.is_mapping() {
                    *target = Value::force_mapping(target.clone());
                    target.extend(value);
                }
            }
        }
    }

    pub fn apply(&mut self, source: &Value, context: &Option<&Context>) {
        println!("Apply {:?}  with  {:?}", self, source);
        match self {
            Value::Sequence(values) => {
                for component in values {
                    component.apply(source, context);
                }
            }
            component if component.is_mapping() || component.is_lazy_eval() => {
                if let Some(LazyEval { code, props }) = component.as_lazy_eval_mut() {
                    for p in props.values_mut() {
                        p.apply(source, context);
                    }
                    *component = process_context_code(code, &props.clone().into(), context);
                } else {
                    let index_map = component.as_mapping_mut().unwrap();
                    for component in index_map.values_mut() {
                        component.apply(source, context);
                    }
                }
                println!("Apply result {:?}", component);
            }
            component if component.is_string() => {
                let text = component.as_str().unwrap();
                *component = process_context_code(text, source, context);
                println!("Apply result {:?}", component);
            }
            _ => {}
        };
    }
}

fn process_context_code(code: &str, props: &Value, context: &Option<&Context>) -> Value {
    let mut pc = processing_context::ProcessingContext::from(context);
    pc.bind(props);
    pc.parse(code)
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Null => write!(f, "null"),
            Self::Bool(b) => write!(f, "{}", b),
            Self::Int(i) => write!(f, "{}", i),
            Self::Float(fl) => write!(f, "{}", fl),
            Self::String(s) => write!(f, "\"{}\"", s),
            Self::Sequence(seq) => {
                write!(f, "[")?;
                for (i, item) in seq.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", item)?;
                }
                write!(f, "]")
            }
            Self::Mapping(map) => {
                write!(f, "{{")?;
                for (i, (key, value)) in map.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}: {}", key, value)?;
                }
                write!(f, "}}")
            }
            _ => Ok(()),
        }
    }
}

// Conversions from primitive types
impl From<()> for Value {
    fn from(_: ()) -> Self {
        Self::Null
    }
}

impl From<bool> for Value {
    fn from(b: bool) -> Self {
        Self::Bool(b)
    }
}

impl From<i64> for Value {
    fn from(i: i64) -> Self {
        Self::Int(i)
    }
}

impl From<i32> for Value {
    fn from(i: i32) -> Self {
        Self::Int(i64::from(i))
    }
}

impl From<f64> for Value {
    fn from(f: f64) -> Self {
        Self::Float(f)
    }
}

impl From<f32> for Value {
    fn from(f: f32) -> Self {
        Self::Float(f64::from(f))
    }
}

impl From<String> for Value {
    fn from(s: String) -> Self {
        Self::String(s)
    }
}

impl From<&str> for Value {
    fn from(s: &str) -> Self {
        Self::String(s.to_string())
    }
}

impl From<Vec<Self>> for Value {
    fn from(seq: Vec<Self>) -> Self {
        Self::Sequence(seq)
    }
}

impl From<IndexMap<String, Self>> for Value {
    fn from(map: IndexMap<String, Self>) -> Self {
        Self::Mapping(map)
    }
}

impl From<LazyEval> for Value {
    fn from(lazy_eval: LazyEval) -> Self {
        Self::LazyEval(lazy_eval)
    }
}

impl From<serde_yaml_ng::Value> for Value {
    fn from(value: serde_yaml_ng::Value) -> Self {
        match value {
            serde_yaml_ng::Value::Null => Value::Null,
            serde_yaml_ng::Value::Bool(b) => Value::Bool(b),
            serde_yaml_ng::Value::Number(number) => {
                if number.is_i64() {
                    Value::Int(number.as_i64().unwrap())
                } else if number.is_u64() {
                    Value::Int(number.as_u64().unwrap() as i64)
                } else {
                    Value::Float(number.as_f64().unwrap())
                }
            }
            serde_yaml_ng::Value::String(s) => Value::String(s),
            serde_yaml_ng::Value::Sequence(values) => {
                Value::Sequence(values.into_iter().map(|value| Value::from(value)).collect())
            }
            serde_yaml_ng::Value::Mapping(mapping) => Value::Mapping(
                mapping
                    .into_iter()
                    .map(|(k, v)| {
                        (
                            k.as_str()
                                .expect("Trying to use a non-string value as key")
                                .to_string(),
                            Value::from(v),
                        )
                    })
                    .collect(),
            ),
            serde_yaml_ng::Value::Tagged(_) => panic!("Trying to parse tag (invalid)"),
        }
    }
}

impl From<Value> for serde_yaml_ng::Value {
    fn from(value: Value) -> serde_yaml_ng::Value {
        match value {
            Value::Null => serde_yaml_ng::Value::Null,
            Value::Bool(b) => serde_yaml_ng::Value::from(b),
            Value::Int(n) => serde_yaml_ng::Value::from(n),
            Value::Float(n) => serde_yaml_ng::Value::from(n),
            Value::String(s) => serde_yaml_ng::Value::from(s),
            Value::Sequence(values) => serde_yaml_ng::Value::Sequence(
                values
                    .into_iter()
                    .map(|value| serde_yaml_ng::Value::from(value))
                    .collect(),
            ),
            Value::Mapping(index_map) => serde_yaml_ng::Value::Mapping(
                index_map
                    .into_iter()
                    .map(|(k, v)| (serde_yaml_ng::Value::from(k), v.into()))
                    .collect(),
            ),
            _ => serde_yaml_ng::Value::Null,
        }
    }
}

impl From<Value> for rust_yaml::Value {
    fn from(value: Value) -> Self {
        match value {
            Value::Null => rust_yaml::Value::Null,
            Value::Bool(b) => rust_yaml::Value::Bool(b),
            Value::Int(i) => rust_yaml::Value::Int(i),
            Value::Float(f) => rust_yaml::Value::Float(f),
            Value::String(s) => rust_yaml::Value::String(s),
            Value::Sequence(values) => rust_yaml::Value::Sequence(
                values
                    .into_iter()
                    .map(|v| rust_yaml::Value::from(v))
                    .collect(),
            ),
            Value::Mapping(index_map) => rust_yaml::Value::Mapping(
                index_map
                    .into_iter()
                    .map(|(k, v)| (rust_yaml::Value::String(k), rust_yaml::Value::from(v)))
                    .collect(),
            ),
            _ => rust_yaml::Value::Null,
        }
    }
}

impl From<boa_engine::JsValue> for Value {
    fn from(js_value: boa_engine::JsValue) -> Self {
        if js_value.is_null() {
            Value::Null
        } else if let Some(b) = js_value.as_boolean() {
            Value::Bool(b)
        } else if let Some(n) = js_value.as_number() {
            if n.fract().abs() > f64::EPSILON {
                Value::Float(n)
            } else {
                Value::Int(n.round() as i64)
            }
        } else if let Some(s) = js_value.as_string() {
            Value::String(s.to_std_string().unwrap())
        } else if let Some(obj) = js_value.as_object() {
            let mut context = boa_engine::Context::default();
            let value = boa_engine::JsValue::from(obj)
                .to_json(&mut context)
                .unwrap()
                .unwrap();
            if let Some(values) = value.as_array() {
                Value::Sequence(
                    values
                        .clone()
                        .into_iter()
                        .map(|v| Value::from(v.clone()))
                        .collect(),
                )
            } else if let Some(index_map) = value.as_object() {
                Value::Mapping(
                    index_map
                        .clone()
                        .into_iter()
                        .map(|(k, v)| (k, Value::from(v)))
                        .collect(),
                )
            } else {
                Value::Null
            }
        } else {
            Value::Null
        }
    }
}

impl From<Value> for boa_engine::JsValue {
    fn from(value: Value) -> Self {
        match value {
            Value::Null => boa_engine::JsValue::null(),
            Value::Bool(b) => boa_engine::JsValue::from(b),
            Value::Int(n) => boa_engine::JsValue::from(n),
            Value::Float(n) => boa_engine::JsValue::from(n),
            Value::String(s) => boa_engine::JsValue::from(boa_engine::JsString::from(s)),
            Value::Sequence(values) => {
                let mut context = boa_engine::Context::default();
                let json = values
                    .into_iter()
                    .map(|value| serde_json::Value::from(value))
                    .collect::<serde_json::Value>();
                boa_engine::JsValue::from_json(&json, &mut context)
                    .expect("Error while parsing array")
            }
            Value::Mapping(index_map) => {
                let mut context = boa_engine::Context::default();
                let json = index_map
                    .into_iter()
                    .map(|(k, v)| (k, serde_json::Value::from(v)))
                    .collect::<serde_json::Value>();
                boa_engine::JsValue::from_json(&json, &mut context)
                    .expect("Error while parsing array")
            }
            _ => boa_engine::JsValue::null(),
        }
    }
}

impl From<rust_yaml::Value> for Value {
    fn from(value: rust_yaml::Value) -> Self {
        match value {
            rust_yaml::Value::Null => Value::Null,
            rust_yaml::Value::Bool(b) => Value::Bool(b),
            rust_yaml::Value::Int(i) => Value::Int(i),
            rust_yaml::Value::Float(f) => Value::Float(f),
            rust_yaml::Value::String(s) => Value::String(s),
            rust_yaml::Value::Sequence(values) => {
                Value::Sequence(values.into_iter().map(|v| v.into()).collect())
            }
            rust_yaml::Value::Mapping(index_map) => Value::Mapping(
                index_map
                    .into_iter()
                    .filter_map(|(k, v)| {
                        if let rust_yaml::Value::String(name) = k {
                            Some((name, v.into()))
                        } else {
                            None
                        }
                    })
                    .collect(),
            ),
        }
    }
}

impl From<serde_json::Value> for Value {
    fn from(value: serde_json::Value) -> Self {
        match value {
            serde_json::Value::Null => Value::Null,
            serde_json::Value::Bool(b) => Value::Bool(b),
            serde_json::Value::Number(number) => {
                if let Some(n) = number.as_f64() {
                    Value::Float(n)
                } else if let Some(n) = number.as_i64() {
                    Value::Int(n)
                } else if let Some(n) = number.as_u64() {
                    Value::Int(n as i64)
                } else if let Some(n) = number.as_i128() {
                    Value::Int(n as i64)
                } else if let Some(n) = number.as_u128() {
                    Value::Int(n as i64)
                } else {
                    Value::Null
                }
            }
            serde_json::Value::String(s) => Value::String(s),
            serde_json::Value::Array(values) => {
                Value::Sequence(values.into_iter().map(|value| Value::from(value)).collect())
            }
            serde_json::Value::Object(map) => {
                Value::Mapping(map.into_iter().map(|(k, v)| (k, Value::from(v))).collect())
            }
        }
    }
}

impl From<Value> for serde_json::Value {
    fn from(value: Value) -> Self {
        match value {
            Value::Null => serde_json::Value::Null,
            Value::Bool(b) => serde_json::Value::Bool(b),
            Value::Int(n) => serde_json::Value::Number(serde_json::Number::from(n)),
            Value::Float(n) => serde_json::Value::Number(serde_json::Number::from_f64(n).unwrap()),
            Value::String(s) => serde_json::Value::String(s),
            Value::Sequence(values) => serde_json::Value::Array(
                values
                    .into_iter()
                    .map(|value| serde_json::Value::from(value))
                    .collect(),
            ),
            Value::Mapping(index_map) => serde_json::Value::Object(
                index_map
                    .into_iter()
                    .map(|(k, v)| (k, serde_json::Value::from(v)))
                    .collect(),
            ),
            _ => serde_json::Value::Null,
        }
    }
}

#[macro_export]
macro_rules! json {
    ($($json:tt)+) => {
        $crate::Value::from(serde_json::json!($($json)+))
    };
}
