use serde_json::{Result, Value};

pub struct Validator;

impl Validator {
    pub fn validate_json(input: &str) -> Result<()> {
        let _json: Value = serde_json::from_str(input)?;

        return Ok(());
    }
}
