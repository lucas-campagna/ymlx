macro_rules! obj {
    ($val:expr) => {
        match $val {
            std::vec(val) => rust_yaml::Value::Sequence(val),
            val => rust_yaml::Value::from(val)
        }
    };
    ($($val:expr),* $(,)?) => {
        {
            let mut arr = Vec::new();
            $(
                arr.push(obj!($val));
            )*
            rust_yaml::Value::Sequence(arr)
        }
    };
    ($($key:tt: $value:tt),* $(,)?) => {
        {
            let mut map = indexmap::IndexMap::new();
            $(
                map.insert(rust_yaml::Value::String(stringify!($key).into()), obj!($value));
            )*
            rust_yaml::Value::Mapping(map)
        }
    }

}

#[cfg(test)]
mod test {
    use rust_yaml::Value;
    use indexmap::IndexMap;
    #[test]
    fn test_simple() {
        // assert_eq!(obj!(1), Value::Int(1));
        assert_eq!(obj!(123), Value::Int(123));
        // let x = obj![1, 2.0, "3"];
        assert_eq!(obj![1, 2.0, "3"], {
            let mut v = Vec::new();
            v.push(Value::Int(1));
            v.push(Value::Float(2.0));
            v.push(Value::String("3".into()));
            Value::Sequence(v)
        });
        // let x = obj!{a: 2};
        // eprintln!("{:?}", obj!{a: 2});
        let m = obj!{
            a: 1,
            b: [1,2,3],
        };
        assert_eq!(*m.get_str("a").unwrap(), Value::Int(1));
        // assert_eq!(obj!{a: 1, b: obj![1,2,3], c: "d"}, {
        //     let mut m = IndexMap::new();
        //     m.insert(Value::String("a".into()), Value::Int(1));
        //     m.insert(Value::String("b".into()), Value::Int(1));
        //     m.insert(Value::String("c".into()), Value::String("d".into()));
        //     Value::Mapping(m)
        // });
    }
}