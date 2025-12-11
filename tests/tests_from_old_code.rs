use htymlx::parser::Parser;
use rust_yaml::Value;

#[test]
fn test_simple_component_parser() {
    let parser = Parser::parse(
        r#"
box:
  from: div
  class: bg-red-100
  body: My box
"#,
    )
    .unwrap();
    let component = parser.call("box", Value::Null).unwrap();
    let html = component.to_html();
    assert_eq!(html, r#"<div class="bg-red-100">My box</div>"#);
}

#[test]
fn test_simple_component_parser_with_variables() {
    let parser = Parser::parse(
        r#"
box:
  from: div
  class: bg-red-100
  body: "$text"
"#,
    )
    .unwrap();
    let props = Parser::parse(
        r#"
text: Hello
"#,
    )
    .unwrap()
    .to_value();
    let component = parser.call("box", props).unwrap();
    let html = component.to_html();
    assert_eq!(html, r#"<div class="bg-red-100">Hello</div>"#);
}

#[test]
fn test_converts_yaml_with_component_composition() {
    let parser = Parser::parse(
        r#"
box1:
  from: div
  class: bg-blue-100 p-2
  body: "$text"
box:
  from: box1
  text: My Comp
"#,
    )
    .unwrap();
    let component = parser.call("box", Value::Null).unwrap();
    let html = component.to_html();
    assert_eq!(html, r#"<div class="bg-blue-100 p-2">My Comp</div>"#);
}

#[test]
fn test_testing_deep_inheritance() {
    let parser = Parser::parse(
        r#"
final:
  from: any_name
  prop: 123
any_name:
  from: box2
  a: $prop
box2:
  from: box
  text: $a
box:
  from: div
  class: bg-green-100
  body: $text
"#,
    )
    .unwrap();
    let component = parser.call("final", Value::Null).unwrap();
    let html = component.to_html();
    assert_eq!(html, r#"<div class="bg-green-100">123</div>"#);
}

#[test]
fn test_testing_style_inheritance() {
    let parser = Parser::parse(
        r#"
box:
  from: box1
  class: width-[20px]
box1:
  from: div
  class: "bg-red-100 $class"
  body: My box
"#,
    )
    .unwrap();
    let component = parser.call("box", Value::Null).unwrap();
    let html = component.to_html();
    assert_eq!(html, r#"<div class="bg-red-100 width-[20px]">My box</div>"#);
}

#[test]
fn test_test_both_parameters() {
    let parser = Parser::parse(
        r#"
box:
  from: box1
  ss: width-[20px]
box1:
  from: div
  class: "$ss"
  body: "$ss"
"#,
    )
    .unwrap();
    let component = parser.call("box", Value::Null).unwrap();
    let html = component.to_html();
    assert_eq!(html, r#"<div class="width-[20px]">width-[20px]</div>"#);
}

#[test]
fn test_simple_test_without_style() {
    let parser = Parser::parse(
        r#"
box:
  from: div
  body: My Comp
"#,
    )
    .unwrap();
    let component = parser.call("box", Value::Null).unwrap();
    let html = component.to_html();
    assert_eq!(html, r#"<div>My Comp</div>"#);
}

#[test]
fn test_without_style_and_body_as_array_with_null() {
    let parser = Parser::parse(
        r#"
box:
  from: div
  id: parent
  body:
    - from: div
      body: first child
    - from: div
      body: second child
    - null
"#,
    )
    .unwrap();
    let component = parser.call("box", Value::Null).unwrap();
    let html = component.to_html();
    assert_eq!(
        html,
        r#"<div id="parent"><div>first child</div><div>second child</div></div>"#
    );
}

#[test]
fn test_render_list_of_children_with_multiple_values() {
    let parser = Parser::parse(
        r#"
box:
  from: div
  id: parent
  body:
    - from: div
      body: first child
    - from: div
      body: second child
    - third child
"#,
    )
    .unwrap();
    let component = parser.call("box", Value::Null).unwrap();
    let html = component.to_html();
    assert_eq!(
        html,
        r#"<div id="parent"><div>first child</div><div>second child</div>third child</div>"#
    );
}

#[test]
fn test_render_list_of_composable_children() {
    let parser = Parser::parse(
        r#"
box1:
  from: div
  body: "$text"
box:
  from: div
  id: parent
  body:
    - from: box1
      text: first child
    - from: box1
      text: second child
    - third child
"#,
    )
    .unwrap();
    let component = parser.call("box", Value::Null).unwrap();
    let html = component.to_html();
    assert_eq!(
        html,
        r#"<div id="parent"><div>first child</div><div>second child</div>third child</div>"#
    );
}

#[test]
fn test_render_list_of_multiple_composable_children() {
    let parser = Parser::parse(
        r#"
box1:
  from: div
  body: "$text"
box2:
  from: div
  body: "$value"
box:
  from: div
  id: parent
  body:
    - from: box1
      text: first child
    - from: box2
      value: second child
    - third child
"#,
    )
    .unwrap();
    let component = parser.call("box", Value::Null).unwrap();
    let html = component.to_html();
    assert_eq!(
        html,
        r#"<div id="parent"><div>first child</div><div>second child</div>third child</div>"#
    );
}

#[test]
fn test_render_list_of_multiple_composable_children_with_implicit_from_2() {
    let parser = Parser::parse(
        r#"
box1:
  from: div
  body: first child
box2:
  from: div
  body: second child
box:
  from: div
  id: parent
  body:
    - box1
    - box2
"#,
    )
    .unwrap();
    let component = parser.call("box", Value::Null).unwrap();
    let html = component.to_html();
    assert_eq!(
        html,
        r#"<div id="parent"><div>first child</div><div>second child</div></div>"#
    );
}

#[test]
fn test_render_composable_children() {
    let parser = Parser::parse(
        r#"
box1:
  from: div
  body: "$value"
box:
  from: div
  id: parent
  body:
    from: box1
    value: first child
"#,
    )
    .unwrap();
    let component = parser.call("box", Value::Null).unwrap();
    let html = component.to_html();
    assert_eq!(
        html,
        r#"<div id="parent"><div>first child</div></div>"#
    );
}

#[test]
fn test_combine_props() {
    let parser = Parser::parse(
        r#"
box1:
  from: div
  class: "$s1 $s2 $s3"
box:
  from: box1
  s1: bg-red-100
  s2: width-2
  s3: "$s3"
"#,
    )
    .unwrap();
    let props = Parser::parse(
        r#"
s3: height-3
"#,
    )
    .unwrap()
    .to_value();
    let component = parser.call("box", props).unwrap();
    let html = component.to_html();
    assert_eq!(html, r#"<div class="bg-red-100 width-2 height-3"></div>"#);
}

#[test]
fn test_without_style_and_implicit_body_as_array() {
    let parser = Parser::parse(
        r#"
box:
  body:
    - from: div
      body: first child
    - from: div
      body: second child
"#,
    )
    .unwrap();
    let component = parser.call("box", Value::Null).unwrap();
    let html = component.to_html();
    // The parser output likely omits the top-level 'box' if it doesn't have 'from'.
    assert_eq!(html, r#"<div>first child</div><div>second child</div>"#);
}

#[test]
fn test_handling_undefined_from_field() {
    let parser = Parser::parse(
        r#"
box:
  from: null
"#,
    )
    .unwrap();
    let component = parser.call("box", Value::Null).unwrap();
    let html = component.to_html();
    // Assuming an empty string is returned for an un-rendered component
    assert_eq!(html, r#""#);
}

#[test]
fn test_multi_level_body_with_implicit_from() {
    let parser = Parser::parse(
        r#"
box:
  from: div
  class: bg-red-100 p-1
  body:
    from: div
    class: bg-yellow-100 p-1
    body:
      from: div
      class: bg-green-100 p-1 size-1
"#,
    )
    .unwrap();
    let component = parser.call("box", Value::Null).unwrap();
    let html = component.to_html();
    assert_eq!(
        html,
        r#"<div class="bg-red-100 p-1"><div class="bg-yellow-100 p-1"><div class="bg-green-100 p-1 size-1"></div></div></div>"#
    );
}

#[test]
fn test_implicit_from_array() {
    let parser = Parser::parse(
        r#"
box:
  body:
    - from: div
      class: bg-red-100 p-1
    - from: div
      class: bg-yellow-100 p-1
    - from: div
      class: bg-green-100 p-1 size-1
"#,
    )
    .unwrap();
    let component = parser.call("box", Value::Null).unwrap();
    let html = component.to_html();
    assert_eq!(
        html,
        r#"<div class="bg-red-100 p-1"></div><div class="bg-yellow-100 p-1"></div><div class="bg-green-100 p-1 size-1"></div>"#
    );
}

#[test]
fn test_render_list_of_multiple_composable_children_with_implicit_from_and_no_body() {
    let parser = Parser::parse(
        r#"
box1:
  from: div
  id: first
  body: first child
box2:
  from: div
  id: second
  body: second child
box:
  body:
    - box1
    - box2
"#,
    )
    .unwrap();
    let component = parser.call("box", Value::Null).unwrap();
    let html = component.to_html();
    assert_eq!(
        html,
        r#"<div id="first">first child</div><div id="second">second child</div>"#
    );
}

#[test]
fn test_optional_props() {
    let parser = Parser::parse(
        r#"
box1:
  from: div
  class: "$a $b"
box:
  body:
    - from: box1
      a: bg-red-100
    - from: box1
      b: bg-blue-100
    - from: box1
      a: bg-red-100
      b: bg-yellow-100
"#,
    )
    .unwrap();
    let component = parser.call("box", Value::Null).unwrap();
    let html = component.to_html();
    assert_eq!(
        html,
        r#"<div class="bg-red-100"></div><div class="bg-blue-100"></div><div class="bg-red-100 bg-yellow-100"></div>"#
    );
}

#[test]
fn test_template_components_with_implicit_from() {
    let parser = Parser::parse(
        r#"
$box:
  from: div
  id: target
  class: "$color p-1 $size"
  body: template
box:
  color: bg-green-100
  size: size-1
  body: instance
"#,
    )
    .unwrap();
    let component = parser.call("box", Value::Null).unwrap();
    let html = component.to_html();
    assert_eq!(
        html,
        r#"<div id="target" class="bg-green-100 p-1 size-1">instance</div>"#
    );
}

#[test]
fn test_template_components_with_body_replacement_and_explicit_from() {
    let parser = Parser::parse(
        r#"
$box:
  from: div
  id: target
  body:
    - from: div
      body: $first
    - from: div
      body: $second
box:
  first: jesus
  second: christ
"#,
    )
    .unwrap();
    let component = parser.call("box", Value::Null).unwrap();
    let html = component.to_html();
    assert_eq!(
        html,
        r#"<div id="target"><div>jesus</div><div>christ</div></div>"#
    );
}

#[test]
fn test_template_components_with_body_replacement_as_string() {
    let parser = Parser::parse(
        r#"
$box:
  from: div
  id: target
  body: "$first $second"
box:
  first: jesus
  second: christ
"#,
    )
    .unwrap();
    let component = parser.call("box", Value::Null).unwrap();
    let html = component.to_html();
    assert_eq!(html, r#"<div id="target">jesus christ</div>"#);
}

#[test]
fn test_template_components_with_body_replacement_and_implicit_from() {
    let parser = Parser::parse(
        r#"
jesus:
  from: div
  class: bg-red-100
  body: jesus
christ:
  from: div
  class: bg-yellow-100
  body: christ
$box:
  from: div
  id: target
  body:
    - $first
    - $second
box:
  first: jesus
  second: christ
"#,
    )
    .unwrap();
    let component = parser.call("box", Value::Null).unwrap();
    let html = component.to_html();
    assert_eq!(
        html,
        r#"<div id="target"><div class="bg-red-100">Jesus</div><div class="bg-yellow-100">Christ</div></div>"#
    );
}

#[test]
fn test_template_component_with_implicit_from_and_list_children() {
    let parser = Parser::parse(
        r#"
$box:
  from: div
  class: "$color p-1 $size"
  body: "-> $text"
box:
  body:
    - color: bg-red-100
    - color: bg-yellow-100
    - size: h-2
    - color: bg-green-100
      size: size-1
    - text: test
"#,
    )
    .unwrap();
    let component = parser.call("box", Value::Null).unwrap();
    let html = component.to_html();
    assert_eq!(
        html,
        r#"<div class="bg-red-100 p-1 ">-> </div><div class="bg-yellow-100 p-1 ">-> </div><div class=" p-1 h-2">-> </div><div class="bg-green-100 p-1 size-1">-> </div><div class=" p-1 ">-> test</div>"#
    );
}

#[test]
fn test_template_component_with_list_implicit() {
    let parser = Parser::parse(
        r#"
$box:
  from: div
  class: flex justify-between
  body:
    - $prop1
    - $prop2
box:
  body:
    - prop1: CompanyA
      prop2: 2024-2025
    - prop1: CompanyB
      prop2: 2023-2024
"#,
    )
    .unwrap();
    let component = parser.call("box", Value::Null).unwrap();
    let html = component.to_html();
    assert_eq!(
        html,
        r#"<div class="flex justify-between">CompanyA2024-2025</div><div class="flex justify-between">CompanyB2023-2024</div>"#
    );
}

#[test]
fn test_shortcut_for_component_reference_in_body_array_1() {
    let parser = Parser::parse(
        r#"
box:
  from: div
  id: parent
  body:
    - div: first child
    - div: second child
    - p: third child
    - h2: fourth child
"#,
    )
    .unwrap();
    let component = parser.call("box", Value::Null).unwrap();
    let html = component.to_html();
    assert_eq!(
        html,
        r#"<div id="parent"><div>first child</div><div>second child</div><p>third child</p><h2>fourth child</h2></div>"#
    );
}

#[test]
fn test_shortcut_for_component_reference_in_body_array_with_other_properties() {
    let parser = Parser::parse(
        r#"
box:
  body:
    - div:
        body: unique child
        class: text-red-500
"#,
    )
    .unwrap();
    let component = parser.call("box", Value::Null).unwrap();
    let html = component.to_html();
    assert_eq!(html, r#"<div class="text-red-500">unique child</div>"#);
}

#[test]
fn test_shortcut_for_component_reference_in_body_array_with_other_properties_with_implicit_body() {
    let parser = Parser::parse(
        r#"
box:
  body:
    - div: unique child
      class: text-red-500
"#,
    )
    .unwrap();
    let component = parser.call("box", Value::Null).unwrap();
    let html = component.to_html();
    assert_eq!(html, r#"<div class="text-red-500">unique child</div>"#);
}

#[test]
fn test_renders_built_in_html_tags_p_h2_span_and_div_with_parameters_and_styles_with_implicit_body() {
    let parser = Parser::parse(
        r#"
box:
  from: div
  body:
    - p: $p_content
      class: text-red-500
    - h2: $h2_content
    - span: $span_content
      class: font-bold
    - div: $div_content
      class: bg-blue-100 p-2
"#,
    )
    .unwrap();
    let props = Parser::parse(
        r#"
p_content: Paragraph text
h2_content: Heading text
span_content: Span text
div_content: Div text
"#,
    )
    .unwrap()
    .to_value();
    let component = parser.call("box", props).unwrap();
    let html = component.to_html();
    assert_eq!(
        html,
        r#"<div><p class="text-red-500">Paragraph text</p><h2>Heading text</h2><span class="font-bold">Span text</span><div class="bg-blue-100 p-2">Div text</div></div>"#
    );
}

#[test]
fn test_render_array_of_elements() {
    let parser = Parser::parse(
        r#"
box1:
  from: ul
  body: $content
box:
  from: box1
  content:
    - from: li
      body: first child
    - from: li
      body: second child
    - from: li
      body: third child
"#,
    )
    .unwrap();
    let component = parser.call("box", Value::Null).unwrap();
    let html = component.to_html();
    assert_eq!(
        html,
        r#"<ul><li>first child</li><li>second child</li><li>third child</li></ul>"#
    );
}

#[test]
fn test_using_props_with_template() {
    let parser = Parser::parse(
        r#"
box:
  body:
    - box1
$box1:
  from: div
  body: $name
box1:
  name: test
"#,
    )
    .unwrap();
    let component = parser.call("box", Value::Null).unwrap();
    let html = component.to_html();
    assert_eq!(html, r#"<div>test</div>"#);
}

#[test]
fn test_allowing_any_props() {
    let parser = Parser::parse(r#"
box:
  body:
    - from: btn
      text: Hello
    - from: btn
      text: World
btn:
  from: button
  body: $text
  onclick: 'alert("$text")'
"#,
    )
    .unwrap();
    let component = parser.call("box", Value::Null).unwrap();
    let html = component.to_html();
    assert_eq!(
        html,
        r#"<button onclick="alert("Hello")">Hello</button><button onclick="alert("World")">World</button>"#
    );
}

#[test]
fn test_inherintance_with_properties_in_a_list() {
    let parser = Parser::parse(
        r#"
document:
  - experience
row:
  from: div
  class: flex justify-between
  body:
    - div: $left
    - div: $right
section:
  from: h2
  class: text-lg font-bold uppercase tracking-widest border-b-2 border-gray-900 pb-1 mb-1
  body: $title
$experiences:
  body:
    - from: row
      left: $company
      right: $type
    - from: row
      left: $position
      right: $date
    - from: p
      body: $description
experience:
  body:
    - from: section
      title: experience
    - from: div
      class: px-2
      body:
        - experiences
experiences:
  body:
    - company: HP
      date: 2020-2023
      position: Sofware Developer
      type: Remote
      description: short description
    - company: HP
      date: 2020-2023
      position: Sofware Developer
      type: Remote
      description: short description
"#,
    )
    .unwrap();
    eprintln!("{}", parser.to_json());
    let component = parser.call("document", Value::Null).unwrap();
    let html = component.to_html();
    eprintln!("{}", html);
    assert_eq!(
        html,
        r#"<h2 class="text-lg font-bold uppercase tracking-widest border-b-2 border-gray-900 pb-1 mb-1">experience</h2><div class="px-2"><div class="flex justify-between"><div>HP</div><div>Remote</div></div><div class="flex justify-between"><div>Sofware Developer</div><div>2020-2023</div></div><p>short description</p><div class="flex justify-between"><div>HP</div><div>Remote</div></div><div class="flex justify-between"><div>Sofware Developer</div><div>2020-2023</div></div><p>short description</p></div>"#
    );
}

#[test]
fn test_implicit_item_with_implicit_body() {
    let parser = Parser::parse(
        r#"
document:
  body:
    - bullet_point: item 1
    - bullet_point: item 2
    - bullet_point: item 3
bullet_point:
  from: li
  class: list-disc ml-5 my-1 text-base
"#,
    )
    .unwrap();
    let component = parser.call("document", Value::Null).unwrap();
    let html = component.to_html();
    assert_eq!(
        html,
        r#"<li class="list-disc ml-5 my-1 text-base">item 1</li><li class="list-disc ml-5 my-1 text-base">item 2</li><li class="list-disc ml-5 my-1 text-base">item 3</li>"#
    );
}

#[test]
fn test_implicit_item_with_implicit_body_in_list_and_args() {
    let parser = Parser::parse(
        r#"
document:
  body:
    - experience
experience:
  body:
    - h1: EXPERIENCES
    - div: experiences
      class: px-2
$experiences:
  body:
    - from: div
      body: $company
    - h2: $date
    - p: $description
experiences:
  body:
    - company: Company A
      date: 2024
      description: Description A
    - company: Company B
      date: 2025
      description:
        - Description B
        - p: Description B
        - Bullet: Description B
Bullet:
  from: li
  class: list-disc ml-5 my-1 text-base
"#,
    )
    .unwrap();
    let component = parser.call("document", Value::Null).unwrap();
    let html = component.to_html();
    assert_eq!(
        html,
        r#"<div><h1>EXPERIENCES</h1><div class="px-2"><div>Company A</div><h2>2024</h2><p>Description A</p><div>Company B</div><h2>2025</h2><p>Description B<p>Description B</p><li class="list-disc ml-5 my-1 text-base">Description B</li></p></div></div>"#
    );
}

#[test]
fn test_implementing_a_map_with_explicit_component_declaration() {
    let parser = Parser::parse(
        r#"
document:
  body:
    - box
$box:
  from: div
  class: p-2 bg-gray-100 my-1
  body: $item
box:
  - item: Item 1
  - item: Item 2
  - item: Item 3
"#,
    )
    .unwrap();
    let component = parser.call("document", Value::Null).unwrap();
    let html = component.to_html();
    assert_eq!(
        html,
        r#"<div class="p-2 bg-gray-100 my-1">Item 1</div><div class="p-2 bg-gray-100 my-1">Item 2</div><div class="p-2 bg-gray-100 my-1">Item 3</div>"#
    );
}

#[test]
fn test_implementing_a_map_with_implicitly_component_declaration() {
    let parser = Parser::parse(
        r#"
document:
  body:
    - box:
      - Item 1
      - Item 2
      - Item 3
$box:
  from: div
  class: p-2 bg-gray-100 my-1
"#,
    )
    .unwrap();
    let component = parser.call("document", Value::Null).unwrap();
    let html = component.to_html();
    assert_eq!(
        html,
        r#"<div><div class="p-2 bg-gray-100 my-1">Item 1</div><div class="p-2 bg-gray-100 my-1">Item 2</div><div class="p-2 bg-gray-100 my-1">Item 3</div></div>"#
    );
}