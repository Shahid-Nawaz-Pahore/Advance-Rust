enum Direction {
    North,
    South,
    East,
    West,
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
