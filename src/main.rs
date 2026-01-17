fn main() {
    let code = r#"
html: |
  <$tag${typeof props === 'object' ? " " + Object.entries(props).map(([key, value]) => `${key}="${value}"`, "").join(" ") : ""}>$body</$tag>
div:
  from!: html
  tag: div
  props: $props
  body: $body
DefaultProps:
  onclick: alert(123)
  class: bg-red-100
$app:
  div!: $text
app:
  props: DefaultProps!
$a: 
  - ${x + y}
  - ${x * y}
a:
  y: 3
  x: $x
"#;

    let map = ymx::json!({
        "text": 2,
    });
    let props = Some(&map);
    let ctx = ymx::parse(code).unwrap();
    for (k, v) in ctx.iter() {
        println!("{:#} {:}", k, v);
    }
    let res = ctx.call("app", props);
    println!("Result: {:}", res);
}
