fn main() {
    let input = "skibidi";
    let output = pig_latin_converter(&input);
    println!("\nOriginal word: {input}, converted to Pig Latin: {output}.");
}

fn pig_latin_converter(input: &str) -> String {
    for i in 0..input.len() {
        if &input[i..i+1] != "a" && &input[i..i+1] != "e" && &input[i..i+1] != "i" && &input[i..i+1] != "o" && &input[i..i+1] != "u" {
            return format!("{}-{}ay", &input[i+1..], &input[i..i+1]);
        }
    }
    String::from("Boo your word is not skbidi enough")
}