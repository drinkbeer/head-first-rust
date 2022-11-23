#![allow(unused_variables)]
#![allow(dead_code)]

fn main() {
    match_operation();
    match_operation_2();
}

enum Direction {
    East,
    West,
    North,
    South,
}

struct Point {
    x: i32,
    y: i32,
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

    let x = 5;

    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    let x = 'c';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);
}

fn match_operation_2() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("six = {:?}, none = {:?}", six, none);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

