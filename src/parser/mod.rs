mod constants;
mod apply;
mod utils;
mod runtime;
mod component;

use std::ops::Deref;
use runtime::Runtime;
use rust_yaml::{Error, Value};
use component::Component;

pub struct Parser(Value);

impl Deref for Parser {
    type Target = Value;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Parser {
    pub fn load(file: &str) -> Result<Parser, Error> {
        let input = std::fs::read_to_string(file)?;
        Ok(Parser::parse(&input)?)
    }
    pub fn parse(input: &str) -> Result<Parser, Error> {
        let value = rust_yaml::Yaml::new().load_str(input)?;
        match value {
            Value::Mapping(value) => Ok(Parser(Value::Mapping(value))),
            _ => Err(Error::emission("Root YAML is not a mapping")),
        }
    }
    pub fn call(&self, name: &str, props: &Value) -> Result<Component, Error> {
        let runtime = Runtime::new(&self.0);
        let value = runtime.call(name, props)?;
        Ok(Component::new(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_simple_html() {
        let parser = Parser::parse(
            r#"
Button:
  from: button
  type: submit
  body: Click me!
"#,
        )
        .unwrap();
        let component = parser.call("Button", &Value::Null).unwrap();
        let html = component.to_html();
        assert_eq!(html, r#"<button type="submit">Click me!</button>"#);
    }

    #[test]
    fn test_html_with_props() {
        let parser = Parser::parse(
            r#"
Link:
  from: a
  href: "$url"
  body: "$text"
"#,
        )
        .unwrap();
        let props = rust_yaml::Yaml::new()
            .load_str(
                r#"
url: "https://example.com"
text: "Example"
"#,
            )
            .unwrap();
        let component = parser.call("Link", &props).unwrap();
        let html = component.to_html();
        assert_eq!(html, r#"<a href="https://example.com">Example</a>"#);
    }

    #[test]
    fn test_nested_components() {
        let parser = Parser::parse(
            r#"
Card:
  from: div
  class: card
  body:
    - from: h1
      body: $title
    - from: p
      body: $content
"#,
        )
        .unwrap();
        let props = rust_yaml::Yaml::new()
            .load_str(
                r#"
title: "Card Title"
content: "This is the card content."
"#,
            )
            .unwrap();
        let component = parser.call("Card", &props).unwrap();
        let html = component.to_html();
        assert_eq!(
            html,
            r#"<div class="card"><h1>Card Title</h1><p>This is the card content.</p></div>"#
        );
    }
    
    #[test]
    fn test_nested_implicit_components() {
        let parser = Parser::parse(
            r#"
Card:
  from: div
  class: card
  body:
    - h1: $title
  - p: $content
"#,
        )
        .unwrap();
        let props = rust_yaml::Yaml::new()
            .load_str(
                r#"
title: "Card Title"
content: "This is the card content."
"#,
            )
            .unwrap();
        let component = parser.call("Card", &props).unwrap();
        let html = component.to_html();
        assert_eq!(
            html,
            r#"<div class="card"><h1>Card Title</h1><p>This is the card content.</p></div>"#
        );
    }

    #[test]
    fn test_templated() {
        let parser = Parser::parse(
            r#"
$Card:
  body: test

Card:
  from: div
  class: card
"#,
        )
        .unwrap();
        let component = parser.call("Card", &Value::Null).unwrap();
        let html = component.to_html();
        assert_eq!(
            html,
            r#"<div class="card">test</div>"#
        );
    }

    #[test]
    fn test_nested_implicit_templated_components() {
        let components = Parser::parse(
            r#"
$Card:
  body:
    - h1: $title
    - p: $content

Card:
  from: div
  class: card
"#,
        )
        .unwrap();
        let props = rust_yaml::Yaml::new()
            .load_str(
                r#"
title: "Card Title"
content: "This is the card content."
"#,
            )
            .unwrap();
        let component = components.call("Card", &props).unwrap();
        let html = component.to_html();
        assert_eq!(
            html,
            r#"<div class="card"><h1>Card Title</h1><p>This is the card content.</p></div>"#
        );
    }
}
