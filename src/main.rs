use std::io::{self, Write};

fn process_command(drink: &str) {
    match drink {
        "beer" => Drink::Beer,
        "vodka" => Drink::Vodka,
        "wine" => Drink::Wine,
        "drinks" => drinks(),
        "drink" => drank(drink.to_string()),
        "drink-drive" => println!("You're now a worthless corpse. Good job!"),
        "arson" => println!("The bar is now a worthless building. Good job!"),
        "13th-admendment" => println!("Sir, this is not a slave state."),
        "library" => println!("You're in a bar, not a library. It's just across the street.\nYou drank 10 beers anyway."),
        "sudo rm -rf /*" => println!("Magically the bar is still here. You're not a hacker."),
        "this is a robbery" => println!("You're not a robber. You're a drunk."),
        "can i have a uhhhh" => println!("Sir, this is not a McDonalds."),
        _ => println!("We don't have {}", drink),
    }
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    if drink == "drink" {
        prompt(false, drink);
    } else {
        prompt(true, drink);
    }
    
}

enum Drink {
    Beer,
    Vodka,
    Wine,
}

fn serve(drink: Drink) {
    match drink {
        Drink::Beer => println!("Here's your beer"),
        Drink::Vodka => println!("Here's your vodka"),
        Drink::Wine => println!("Here's your wine"),
    }
}

fn main() {
    serve(Drink::Beer);
    serve(Drink::Vodka);
    serve(Drink::Wine);
}

fn drinks() {
    println!("Beer\nVodka\nWine")
}

fn drank(drink: String) {
    println!("You savored the {}", drink);
}

fn prompt(bool: bool, udrink: &str) {
    if bool == false {
        clearscreen::clear().expect("failed to clear screen");
        println!("Welcome to the bar, what would you like?");
        print!("> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let drink = input.trim();
        process_command(drink);
    } else {
        clearscreen::clear().expect("failed to clear screen");
        println!("What would you like to do now?");
        print!("> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let drink = input.trim();
        process_command(drink);
    }
}

fn main() {
    prompt(false, "");
}