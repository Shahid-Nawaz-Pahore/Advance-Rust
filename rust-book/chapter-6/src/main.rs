use std::option::Option<f64>;
enum Direction {
    North,
    South,
    East,
    West,
}
enum Message {
    Quit,                   // no data
    Move { x: i32, y: i32 },// named fields
    Write(String),          // single value
    ChangeColor(u8, u8, u8) // tuple
}

enum Option<T> {
    Some(T),
    None,
}


fn main() {

    let msgs = [
        Message::Quit,
        Message::Move { x: 10, y: 20 },
        Message::Write(String::from("Hello")),
        Message::ChangeColor(255, 0, 0),
    ];

    for msg in msgs {
        match msg {
            Message::Quit => println!("Quit message"),
            Message::Move { x, y } => println!("Move to ({}, {})", x, y),
            Message::Write(text) => println!("Text: {}", text),
            Message::ChangeColor(r, g, b) => println!("Color: rgb({}, {}, {})", r, g, b),
        }
    }

    let dir = Direction::North;

    match dir {
        Direction::North => println!("Going up!"),
        Direction::South => println!("Heading down!"),
        Direction::East => println!("Moving right!"),
        Direction::West => println!("Moving left!"),
    }

    fn divide(a: f64, b: f64) -> Option<f64> {
        if b == 0.0 {
            None
        } else {
            Some(a / b)
        }
    }
}
fn divide(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 {
        None
    } else {
        Some(a / b)
    }
}
