#![allow(unused_variables)]
#![allow(dead_code)]

fn main() {
    lifetime_1();
    lifetime_2();
}

fn lifetime_1() {
    let x = 5;
    let r = &x;

    println!("r: {}", r);
}

/*
&i32        // 一个引用
&'a i32     // 具有显式生命周期的引用
&'a mut i32 // 具有显式生命周期的可变引用

至此，可以对生命周期进行下总结：生命周期语法用来将函数的多个引用参数和返回值的作用域关联到一起，一旦关联到一起后，Rust 就拥有充分的信息来确保我们的操作是内存安全的。
*/
fn lifetime_2() {
    let s1 = String::from("hello");
    let s2 = "wor";

    let result = longest(&s1, s2);
    println!("The longest string is {}", result);

    let result2 = longest(s1.as_str(), s2);
    println!("The longest string is {}", result2);

    let s3 = String::from("hello");
    // let result3;
    {
        let s4 = String::from("world!!!");
        // result3 = longest(s3.as_str(), s4.as_str()); // result3 的生命周期是 s4 的生命周期 (min in s3 and s4 lifetime)
    }
    // println!("The longest string is {}", result3);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
