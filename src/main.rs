mod json;
mod key_value_store;

#[cfg(test)]
mod tests {
    use crate::json::validation::Validator;

    #[test]
    fn validator_test() {
        let valid_json_string = r#"{"a" : "b"}"#;
        let invalid_json_string = r#"{"a : "b"}"#;

        let mut res = Validator::validate_json(valid_json_string);

        assert_eq!(false, res.is_err());

        res = Validator::validate_json(invalid_json_string);

        assert_eq!(true, res.is_err());
    }
}

fn main() {
    println!("Hello, world!");
}
