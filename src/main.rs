fn main() {
    let code = r#"
a: ${abc}
b: ${[name,last].join("^")}"#;
    let map = ymx::json!({
        "abc": "John",
        "name": "Mark",
        "last": "Apple"
    });
    let props = Some(&map);
    let c = ymx::parse(code).unwrap();
    let res = c.call("b", props);
    println!("{:}", res);
}
