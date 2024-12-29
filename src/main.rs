use std::{env, process};
use serde_json::Value;
use gen_alpha_dictionary::dictionary;

fn main() {
    let args: Vec<String> = env::args().collect();

    let word = dictionary::parse_args(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // let dict: Value = dictionary::parse_local_json("dict.json").unwrap_or_else(|err| {
    //     println!("Problem parsing dictionary: {err}");
    //     process::exit(1);
    // });

    let dict: Value = dictionary::parse_web_json("https://raw.githubusercontent.com/crnicholson/gen_alpha_dictionary/refs/heads/master/dict.json").unwrap_or_else(|err| {
        if err.to_string().contains("status code 500") {
            println!("Problem fetching JSON data.")
        } else {
            println!("Problem parsing dictionary: {err}");
        }
        process::exit(1);
    });

    let result = dictionary::define_word(&dict, &word).unwrap_or_else(|err | {
        println!("{err}");
        process::exit(1);
    });

    println!("\nThe defintion of {word} is {result}! Enjoy your day, skibidi sigma, and make sure to rizz up some level 10 gyatts!")
}

