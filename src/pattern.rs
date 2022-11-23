

fn main() {
    match_operation();
}

enum Direction {
    East,
    West,
    North,
    South,
}

fn match_operation() {
    let dire = Direction::South;
    match dire {
        Direction::East => println!("East"),
        Direction::North | Direction::South => {
            println!("South or North");
        },
        _ => println!("West"),
    };
}
