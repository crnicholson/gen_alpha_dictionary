use std::{fs, env, process, error::Error};
use serde_json::Value;

fn main() {
    let args: Vec<String> = env::args().collect();

    let word = parse_args(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    let dict: Value = parse_json("dict.json").unwrap_or_else(|err| {
        println!("Problem parsing dictionary: {err}");
        process::exit(1);
    });

    let result = define_word(&dict, &word).unwrap_or_else(|err | {
        println!("{err}");
        process::exit(1);
    });

    println!("The defintion of {word} is {result}! Enjoy your day, skibidi sigma, and make sure to rizz up some level 10 gyatts!")
}

fn parse_args(args: &[String]) -> Result<String, &'static str> {
    if args.len() < 2 {
        return Err("Please enter the skibidi word you want defined.");
    }

    Ok(args[1].clone().to_lowercase())
}

fn parse_json(path: &str) -> Result<Value, Box<dyn Error>> {
    let json: String = fs::read_to_string(path)?;
    Ok(serde_json::from_str(&json)?)
}

fn define_word<'a>(dict: &'a Value, key: &'a str) -> Result<&'a str, &'static str> {
    match dict[key].as_str() {
        Some(value) => Ok(value),
        None => Err("Uh-oh my skibidi rizzler, that Ohio word is not in my sigma dictionary.")
    }
}