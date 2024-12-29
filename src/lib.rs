pub mod dictionary {
    use std::{fs, error::Error};
    use serde_json::Value;

    pub fn parse_args(args: &[String]) -> Result<String, &'static str> {
        if args.len() < 2 {
            return Err("Please enter the skibidi word you want defined.");
        }

        Ok(args[1].clone().to_lowercase().replace('"', ""))
    }

    pub fn parse_local_json(path: &str) -> Result<Value, Box<dyn Error>> {
        let json: String = fs::read_to_string(path)?;
        Ok(serde_json::from_str(&json)?)
    }

    pub fn parse_web_json(url: &str) -> Result<Value, Box<dyn Error>> {
        let body: String = ureq::get(url)
            .call()?
            .into_string()?;
        Ok(serde_json::from_str(&body)?)
    }

    pub fn define_word<'a>(dict: &'a Value, key: &'a str) -> Result<&'a str, &'static str> {
        match dict[key].as_str() {
            Some(value) => Ok(value),
            None => Err("Uh-oh my skibidi rizzler, that Ohio word is not in my sigma dictionary.")
        }
    }
}