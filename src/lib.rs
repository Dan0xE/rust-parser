use std::collections::HashMap;
pub mod parser;
pub use parser::*;

#[derive(Debug, PartialEq)]
pub enum Value {
    Null,
    Bool(bool),
    Float(f64),
    Int(isize),
    Str(String),
    Array(Vec<Value>),
    Object(HashMap<String, Value>),
}

#[cfg(test)]
mod tests {
    use super::*;
    use gobble::traits::*;
    use Value::*;
    #[test]
    fn test_array_and_awkward_spacing() {
        let a = r#"
        [    "dave"
        ,       "sam", 34.5, [1,2,      3       ]]

        "#;
        let v = JsonValue.parse_s(a).unwrap();
        assert_eq!(
            v,
            Array(vec![
                Str("dave".to_string()),
                Str("sam".to_string()),
                Float(34.5),
                Array(vec![Int(1), Int(2), Int(3)])
            ])
        );
    }

    #[test]
    fn test_map_and_escapes() {
        let a = r#"{"hello":"\u0057orld","animals":"cat\ndog\tfish"}"#;
        let v = JsonValue.parse_s(a).unwrap();
        let mut hm = HashMap::new();
        hm.insert("hello".to_string(), Str("World".to_string()));
        hm.insert("animals".to_string(), Str("cat\ndog\tfish".to_string()));
        assert_eq!(v, Object(hm));
    }
}
