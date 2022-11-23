#![allow(unused_variables)]

fn main() {
    if_operation();
    for_operation();
    while_operation();
}

fn if_operation() {
    let n = 12;

    if n % 4 == 0 {
        println!("n is divisible by 4");
    } else if n % 3 == 0 {
        println!("n is divisible by 3");
    } else if n % 2 == 0 {
        println!("n is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

fn for_operation() {
    for i in 0..=5 {
        println!("i = {}", i);
    }

    for i in 6..10 {
        if i == 7 {
            continue;
        }
        if i == 8 {
            break;
        }
        println!("i = {}", i);
    }

    let arr = [10, 20, 30, 40, 50];
    for (i, v) in arr.iter().enumerate() {
        println!("i = {}, v = {}", i, v);
    }
}

fn while_operation() {
    let mut n = 0;

    while n <= 5 {
        println!("n = {}", n);
        n += 1;
    }

    let mut n_1 = 0;

    loop {
        if n_1 == 5 {
            break;
        }
        println!("n_1 = {}", n_1);
        n_1 += 1;
    }
}
