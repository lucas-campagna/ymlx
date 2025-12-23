use htymlx::parser::Parser;
use htymlx::render::html;
use rust_yaml::Value;

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
    let component = parser.call("Button", Value::Null).unwrap();
    let html = html(&component);
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
"#
    )
    .unwrap();
    let props = Parser::parse(
            r#"
url: "https://example.com"
text: "Example"
"#
    ).unwrap()
    .to_value();
    let component = parser.call("Link", props).unwrap();
    let html = html(&component);
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
    let props = Parser::parse(
            r#"
title: "Card Title"
content: "This is the card content."
"#,
        )
        .unwrap()
        .to_value();
    let component = parser.call("Card", props).unwrap();
    let html = html(&component);
    assert_eq!(
        html,
        r#"<div class="card"><h1>Card Title</h1><p>This is the card content.</p></div>"#
    );
}

#[test]
fn test_nested_implicit_components() {
    let parser = Parser::parse(r#"
Card:
  from: div
  class: card
  body:
    - h1: $title
    - p: $content
"#,
    )
    .unwrap();
    let props = Parser::parse(
            r#"
title: "Card Title"
content: "This is the card content."
"#,
        )
        .unwrap()
        .to_value();
    let component = parser.call("Card", props).unwrap();
    let html = html(&component);
    assert_eq!(
        html,
        r#"<div class="card"><h1>Card Title</h1><p>This is the card content.</p></div>"#
    );
}

#[test]
fn test_component_reference_by_name() {
    let parser = Parser::parse(
        r#"
comp1: comp2
comp2: comp3
"#,
    )
    .unwrap();
    let component = parser.call("comp1", Value::Null).unwrap();
    let html = html(&component);
    assert_eq!(
        html,
        r#"comp3"#
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
    let component = parser.call("Card", Value::Null).unwrap();
    let html = html(&component);
    assert_eq!(
        html,
        r#"<div class="card">test</div>"#
    );
}

#[test]
fn test_nested_implicit_templated_components() {
    let components = Parser::parse(
        r#"
$card:
  body:
    - h1: $title
    - p: $content

card:
  from: div
  class: card
"#,
    )
    .unwrap();
    let props = Parser::parse(
            r#"
title: "Card Title"
content: "This is the card content."
"#,
        )
        .unwrap()
        .to_value();
    let component = components.call("card", props).unwrap();
    let html = html(&component);
    assert_eq!(
        html,
        r#"<div class="card"><h1>Card Title</h1><p>This is the card content.</p></div>"#
    );
}

#[test]
fn test_shortcut_with_mapping_body_as_component_properties() {
    let components = Parser::parse(r#"
a: 123
b:
  a: 456
c:
  a:
    d: 123
    e: 456

"#)
    .unwrap();
    let b = components.call("b", Value::Null).unwrap();
    assert_eq!(
        html(&b),
        r#"456"#
    );
    let c = components.call("c", Value::Null).unwrap();
    assert_eq!(
        c.to_string(),
        r#"{"d": 123, "e": 456}"#
    );
}