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

fn main() {
    let dir = Direction::North;

    match dir {
        Direction::North => println!("Going up!"),
        Direction::South => println!("Heading down!"),
        Direction::East => println!("Moving right!"),
        Direction::West => println!("Moving left!"),
    }
}
