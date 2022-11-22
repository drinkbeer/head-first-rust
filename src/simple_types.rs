fn main() {
    num_variables(true);
    bit_operation(true);
    range_operation(true);
    char_opeartion(true);
    bool_operation(true);
    string_operation(true);
}

fn num_variables(enabled: bool) {
    if !enabled {
        return;
    }

    // 编译器会进行自动推导，给予twenty i32的类型
    let twenty = 20;
    // 类型标注
    let twenty_one: i32 = 21;
    // 通过类型后缀的方式进行类型标注：22是i32类型
    let twenty_two = 22i32;

    // 只有同样类型，才能运算
    let addition = twenty + twenty_one + twenty_two;
    println!("{} + {} + {} = {}", twenty, twenty_one, twenty_two, addition);

    // 对于较长的数字，可以用_进行分割，提升可读性
    let one_million: i64 = 1_000_000;
    println!("{}", one_million.pow(2));

    // 定义一个f32数组，其中42.0会自动被推导为f32类型
    let forty_twos = [
        42.0,
        42f32,
        42.0_f32,
    ];

    // 打印数组中第一个值，并控制小数位为2位
    println!("{:.2}", forty_twos[0]);
}

fn bit_operation(enabled: bool) {
    if !enabled {
        return;
    }

    // 二进制为00000010
    let a:i32 = 2;
    // 二进制为00000011
    let b:i32 = 3;

    println!("(a & b) value is {}", a & b);

    println!("(a | b) value is {}", a | b);

    println!("(a ^ b) value is {}", a ^ b);

    println!("(!b) value is {} ", !b);

    println!("(a << b) value is {}", a << b);

    println!("(a >> b) value is {}", a >> b);

    let mut a = a;
    // 注意这些计算符除了!之外都可以加上=进行赋值 (因为!=要用来判断不等于)
    a <<= b;
    println!("(a << b) value is {}", a);
}

fn range_operation(enabled: bool) {
    if !enabled {
        return;
    }

    for i in 1..5 {
        print!("{} ", i);
    }

    println!();

    for i in 1..=5 {
        print!("{} ", i);
    }

    println!();

    for c in 'a'..'e' {
        print!("{} ", c);
    }

    println!();
}

fn char_opeartion(enabled: bool) {
    if !enabled {
        return;
    }

    let c = 'z';
    let z = 'ℤ';
    let g = '国';
    let heart_eyed_cat = '😻';

    println!("{} {} {} {}", c, z, g, heart_eyed_cat);
}

fn bool_operation(enabled: bool) {
    if !enabled {
        return;
    }

    let t = true;

    let f: bool = false; // 使用类型标注,显式指定f的类型

    if f {
        println!("f = {}", f);
    } else if t {
        println!("t = {}", t);
    } else {
        println!("这是段无意义的代码");
    }
}

fn string_operation(enabled: bool) {
    if !enabled {
        return;
    }

    let s1 = "hello"; // s1 is '&str' type, which is hard-coding the length and value of the string
    // s1.push_str(", world!"); // error: `s1` does not have a method named `push_str`

    let s2 = String::from("hello"); // s2 is 'String' type, which is dynamically allocated
    // s2.push_str(", world!"); // error: cannot borrow `s2` as mutable, as it is not declared as mutable

    let mut s3 = String::from("hello");
    s3.push_str(", world!"); // ok

    println!("{} {} {} ", s1, s2, s3);

    let mut s4 = "hello";
    println!("s4 = {}", s4);
    s4 = "world"; // s4 is a pointer to a string in stack, so it's a copy, no ownership transfer
    println!("s4 = {}", s4);
    // s4.push_str(", world!"); // error: `s4` does not have a method named `push_str`
}
