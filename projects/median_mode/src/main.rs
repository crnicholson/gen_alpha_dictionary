use std::{collections::HashMap, f32::consts::TAU};

fn main() {
    let ints = [1, 4, 3, 2, 1];

    let mode_option = mode(&ints);
    let median_val = median(&ints);

    let mut mode_val = 0;

    match mode_option {
        Some(t) => {
            println!("{t}");
            mode_val = t;
        }
        None => println!("No value!")
    } 

    println!("\nMode: {mode_val}, median: {median_val}.")
}

fn mode(ints: &[i32; 5]) -> Option<i32> {
    let mut nums: HashMap<i32, i32> = HashMap::new();
    
    for i in ints {
        let count = nums.entry(*i).or_insert(0);
        *count += 1;
    }

   nums.into_iter().max_by_key(|&(_, count)| count).map(|(val, _)| val)
}

fn median(ints: &[i32; 5]) -> i32 {
    let mut nums = Vec::new();
    for i in ints {
        nums.push(i);
    }
    nums.sort();
    let length = nums.len();
    let middle = length / 2;
    let middle_val = nums.get(middle);
    match middle_val {
        Some(&x) => return *x,
        None => return 0,
    }
}
