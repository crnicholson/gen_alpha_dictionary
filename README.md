# Gen-Alpha Dictionary

Easily learn gen-alpha (or, as found in the wild, iPad kids) phrases with a Rust Command Line Interface (CLI) and a hand-compiled list of 250 common words and prashes of your favorite 10-year-old. 

## Install

### To use as a dependency

```rust
[dependencies]
gen_alpha_dictionary = "0.1.0"
```

## Usage

### To use in a project

Import:

```rust
use std::process;
use serde_json::Value;
use gen_alpha_dictionary::dictionary;
```

Parse JSON from web:

```rust
let dict: Value = dictionary::parse_web_json("https://raw.githubusercontent.com/crnicholson/gen_alpha_dictionary/refs/heads/master/dict.json").unwrap_or_else(|err| {
    if err.to_string().contains("status code 500") {
        println!("Problem fetching JSON data.")
    } else {
        println!("Problem parsing dictionary: {err}");
    }
    process::exit(1);
});
```

Or, parse JSON locally:

```rust
let dict: Value = dictionary::parse_local_json("dict.json").unwrap_or_else(|err| {
    println!("Problem parsing dictionary: {err}");
    process::exit(1);
});
```

And finally, get the definition of the word:

```rust
let result = dictionary::define_word(&dict, &word).unwrap_or_else(|err | {
    println!("{err}");
    process::exit(1);
});
```

### To use as a CLI

Install:

```bash
git clone https://github.com/crnicholson/gen_alpha_dictionary.git
cd gen_alpha_dictionary
```

Run:

```bash
cargo run -- "skibid"
```

This will find the definition of skibidi. Double qoutes are unnecessary for single word prashes.

## License

Licensed under MIT. Copyright 2024 Charles Nicholson.