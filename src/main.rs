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
}
