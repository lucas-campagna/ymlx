#[cfg(test)]
mod parser_test {
    use rust_yaml::Yaml;

    use crate::parser::runtime::Runtime;

    #[test]
    fn basic_component_render() {
        let code = r#"
box:
    from: div
    body: hello world
"#;
        let comps = Yaml::new().load_str(code).unwrap();
        let mut runtime = Runtime::new();
        runtime.add_many(&comps);
        let res = runtime.call("box", None);
        print!("{}", res);
        // assert!(*res);
        // let value = build(Input::Code(code)).unwrap().call("box", None);
    }
}