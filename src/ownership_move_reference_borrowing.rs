fn main() {
    let enabled = true;
    num_ownership(enabled);
    str_ownership(enabled);
    str_deep_copy(enabled);
    ownership_and_copy(enabled);
    return_ownership(enabled);
    reference_1(enabled);
    reference_2(enabled);
    reference_3(enabled);
    reference_4(enabled);
    reference_5(enabled);
    reference_6(enabled);
    reference_7(enabled);
}

fn num_ownership(enabled: bool) {
    if !enabled {
        return;
    }

    let a = 5;
    let b = a; // a and b are both in stack, so it's a copy, no ownership transfer

    println!("a = {}, b = {}", a, b);
}

/*
A string has two parts:
* In stack: a pointer to the memory in heap, and the length of the string, the capacity of the string, each is 8 bit each
* In heap: the actual string
*/
fn str_ownership(enabled: bool) {
    if !enabled {
        return;
    }

    let s1 = String::from("hello");
    let _s2 = s1; // error: ownership moved from s1 to s2, s1 is invalid now

    // println!("s1 = {}, s2 = {}", s1, s2); // error: borrow of moved value: `s1`
}

/*
Here are some of the types that implement Copy trait:

    - All the integer types, such as u32.
    - The Boolean type, bool, with values true and false.
    - All the floating point types, such as f64.
    - The character type, char.
    - Tuples, if they only contain types that also implement Copy. For example, (i32, i32) implements Copy, but (i32, String) does not.

With copy trait, it will be copied when it's assigned to another variable, so the ownership is not transferred.
*/
fn str_deep_copy(enabled: bool) {
    if !enabled {
        return;
    }

    let s1 = String::from("hello");
    let s2 = s1.clone(); // deep copy, s1 and s2 are both valid

    println!("s1 = {}, s2 = {}", s1, s2);
}

// Understand the ownership and copy
fn ownership_and_copy(enabled: bool) {
    if !enabled {
        return;
    }

    let s = String::from("hello"); // s comes into scope

    takes_ownership(s); // some_string's value moves into the function...
                                 // ... and so is no longer valid here
    // println!("s = {}", s); // error: borrow of moved value: `s`, which is dropped at the end of the `takes_ownership` function

    let x = 5; // x comes into scope
    makes_copy(x);
    println!("x = {}", x); // x is still valid here, because it's a copy
}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing memory is freed.

fn makes_copy(some_integet: i32) { // some_integer comes into scope
    println!("{}", some_integet);
} // Here, some_integer goes out of scope. Nothing special happens.

fn return_ownership(enabled: bool) {
    if !enabled {
        return;
    }

    let s1 = gives_ownership(); // gives_ownership moves its return value into s1

    let s2 = String::from("world"); // s2 comes into scope

    let s3 = takes_and_gives_back(s2);

    println!("s1 = {}", s1);
    // println!("s2 = {}", s2); // error: borrow of moved value: `s2`, which is dropped at the end of the `takes_and_gives_back` function
    println!("s3 = {}", s3);
}

fn gives_ownership() -> String { // gives_ownership will move its return value into the function that calls it
    let some_string = String::from("hello"); // some_string comes into scope
    some_string // some_string is returned and moves out to the calling function
}

fn takes_and_gives_back(a: String) -> String {
    a // a is returned and moves out to the calling function
}

fn reference_1(enabled: bool) {
    if !enabled {
        return;
    }

    let x = 5;
    let y = &x; // y is a reference to x, y is a pointer to x, y is in stack, x is in heap
    assert_eq!(x, *y); // *y is the value of y, which is the value of x
    // assert_eq!(x, y); // y is a pointer, so it's not equal to x. error: no implementation for `{integer} == &{integer}`
}

fn reference_2(enabled: bool) {
    if !enabled {
        return;
    }

    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn reference_3(enabled: bool) {
    if !enabled {
        return;
    }

    let s = String::from("hello");
    change_3(&s);
    println!("s = {}", s);
}

fn change_3(_some_string: &String) {
    // _some_string.push_str(", world"); // error: cannot borrow as mutable, because it's a reference
}

// You can only create one mutable reference to a particular piece of data in a particular scope.
fn reference_4(enabled: bool) {
    if !enabled {
        return;
    }

    let mut s = String::from("hello");
    change_4(&mut s);
    println!("s = {}", s);
}

fn change_4(some_string: &mut String) {
    some_string.push_str(", world");
}

// Cannot have multiple mutable references.
fn reference_5(enabled: bool) {
    if !enabled {
        return;
    }

    let mut s = String::from("hello");
    let _r1 = &mut s;
    // let r2 = &mut s; // error: cannot borrow `s` as mutable more than once at a time
    // println!("{}, {}", r1, r2);
}

// Cannot have a mutable reference while we have an immutable one.
fn reference_6(enabled: bool) {
    if !enabled {
        return;
    }

    let s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    // let r3 = &mut s; // error: cannot borrow `s` as mutable because it is also borrowed as immutable
    // let r4 = &mut s; // error: cannot borrow `s` as mutable more than once at a time

    println!("{}, {}", r1, r2);
}

/**
 * The scopes of the immutable references r1 and r2 end after the println! where they are last used, which is before the
 * mutable reference r3 is created. These scopes don’t overlap, so this code is allowed.
 */
fn reference_7(enabled: bool) {
    if !enabled {
        return;
    }

    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2); // r1 and r2 are no longer used since here, so they are out of scope

    let r3 = &mut s;
    println!("{}", r3)
}
