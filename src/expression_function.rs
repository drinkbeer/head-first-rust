fn main() {
    println!("{}", add_with_extra(1, 2));
    expression_operation(true);
    assert_eq!(ret_unit_type(), ());
}

fn add_with_extra(x: i32, y:i32) -> i32 {
    let x = x + 1;
    let y = y + 2;
    x + y
}

fn expression_operation(enabled: bool) {
    if !enabled {
        return
    }
    let x = 5;
    let y = {
        let x = x + 1;
        x * 2
    };
    println!("x = {}, y = {}", x, y);
}

fn ret_unit_type() {
    let x = 1;
    // if 语句块也是一个表达式，因此可以用于赋值，也可以直接返回
    // 类似三元运算符，在Rust里我们可以这样写
    let _y = if x % 2 == 1 {
        "odd"
    } else {
        "even"
    };
    // 或者写成一行
    let _z = if x % 2 == 1 { "odd" } else { "even" };
}
