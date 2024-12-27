enum Animal {
    Named(String),
    None,
}

fn main() {
    let name = String::from("Rex");
    let dog: Animal = Animal::Named(name);
    let no_dog: Animal = Animal::None;
    match no_dog {
        Animal::Named(x) => println!("Got a dog named: {}", x),
        Animal::None => println!("No dog"),
    }
    match dog {
        Animal::Named(x) => println!("Got a dog named: {}", x),
        Animal::None => println!("No dog"),
    }
    let none: Option<i8> = Option::None;
    let five: Option<i8> = Option::Some(5);

    match none {
        Some(x) => println!("Got a value: {}", x),
        None => println!("No value"),
    };
    match five {
        Some(x) => println!("Got a value: {}", x),
        None => println!("No value"),
    };

    let dice_roll = 6;
    match dice_roll {
        1..=6 => println!("Got a number between 1 and 6"),
        _ => println!("Got something else"),
    }

    let max = Option::Some(10);
    if let Some(x) = max {
        println!("Got a value: {}", x);
    }
    if let None = max {
        println!("No value");
    } // can also write this way but we lose the exhaustive check benefits of match expressions
    // pretty much a match where we only have an arm for one value and we ignore all the rest
}
