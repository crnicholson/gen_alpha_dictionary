use std::{env, process};
use serde_json::Value;

fn main() {
    let args: Vec<String> = env::args().collect();

    let word = sigma_dict::parse_args(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    let dict: Value = sigma_dict::parse_json("dict.json").unwrap_or_else(|err| {
        println!("Problem parsing dictionary: {err}");
        process::exit(1);
    });

    let result = sigma_dict::define_word(&dict, &word).unwrap_or_else(|err | {
        println!("{err}");
        process::exit(1);
    });

    println!("\nThe defintion of {word} is {result}! Enjoy your day, skibidi sigma, and make sure to rizz up some level 10 gyatts!")
}

