#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(States),
}

#[derive(Debug)]
enum States {
    Alaska, 
    Alabama,
}

fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => {
            println!("You got a nickel!");
            5
        },
        Coin::Dime => {
            println!("You got a dime!");
            10
        },
        Coin::Quarter(state) => {
            println!("You got a quarter from {:?}!", state);
            25
        },
    }
}

fn main() {
    let pocket_change = Coin::Quarter(States::Alaska);

    println!("\nPocket change: {:?}", pocket_change);

    println!("You coin is worth {} cent(s).", value_in_cents(&pocket_change));
}

// let dice_roll = 9;
// match dice_roll {
//     3 => add_fancy_hat(),
//     7 => remove_fancy_hat(),
//     _ => (),
// }