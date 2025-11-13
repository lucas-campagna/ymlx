use std::collections::HashMap;

pub enum Properties {
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    Sequence(Vec<Properties>),
    Mapping(HashMap<String, Properties>),
    Function(fn(props: HashMap<String, Properties>) -> Option<Properties>),
}

pub struct Component {
    name: String,
    value: Option<Properties>,
}

pub struct EntryPoint {
    selector: String,
    component: Properties,
}

pub struct Document {
    templates: HashMap<String, Component>,
    components: HashMap<String, Component>,
    entry_points: HashMap<String, EntryPoint>,
}