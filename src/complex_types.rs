
fn main() {
    slice_1(true);
    struct_operation(true);
    enum_operation(true);
    array_operation();
}

/*
Slice is a kind of reference, so it doesn't have ownership.
*/
fn slice_1(enabled: bool) {
    if !enabled {
        return;
    }

    let a = [1, 2, 3, 4, 5];
    let b = &a[1..3];
    println!("b = {:?}", b);

    let s = String::from("hello world");
    let len = s.len();
    let s1 = &s[0..5];
    let s2 = &s[6..len];
    let s3 = &s[6..];
    let s4 = &s[..];
    let s5 = &s[..=4];
    let s6 = &s[11..];
    println!("s1 = {}, s2 = {}, s3 = {}, s4 = {}, s5 = {}, s6 = {}", s1, s2, s3, s4, s5, s6);

    println!("First word: {}", first_word(&s));
    let s_0 = "Hello";
    println!("First word: {}", first_word(&s_0.to_string()));

    let s_1 = String::from("hello world");
    first_word_2(&s_1[..]);
    first_word_2(&s_1[0..]);
    first_word_2(&s_1[..s_1.len()]);
    first_word_2(&s_1[0..s_1.len()]);
    first_word_2(&s_1);

    let s_2 = "hello world";
    first_word_2(&s_2[..]);
    first_word_2(&s_2[0..]);
    first_word_2(&s_2[..s_2.len()]);
    first_word_2(&s_2[0..s_2.len()]);
    first_word_2(&s_2);
    first_word_2(s_2);


}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn first_word_2(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}


fn struct_operation(enabled: bool) {
    if !enabled {
        return;
    }
    // Note that the entire instance must be mutable; Rust doesn’t allow us to mark only certain fields as mutable.
    let mut u1 = User {
        username: String::from("my_name"),
        email: String::from("my_email"),
        sign_in_count: 1,
        active: true,
    };
    u1.email = "my_email@gmail.com".to_string();
    // println!("u1 = {:?}", u1);

    let u2 = User {
        email: String::from("another_email"),
        ..u1
    };
    // println!("u2 = {:?}", u2);
    println!("u2.username = {}, u2.email = {}, u2.sign_in_count = {}, u2.active = {}", u2.username, u2.email, u2.sign_in_count, u2.active);

}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

enum PokerCard {
    Clubs(u8),
    Spades(u8),
    Diamonds(char),
    Hearts(char),
}

fn enum_operation(enabled: bool) {
    if !enabled {
        return;
    }
    let c0: PokerCard = PokerCard::Clubs(1);
    let c1 = PokerCard::Spades(5);
    let c2 = PokerCard::Diamonds('A');
    let c3 = PokerCard::Hearts('K');
}


fn array_operation() {
    // 编译器自动推导出one的类型
let one             = [1, 2, 3];
// 显式类型标注
let two: [u8; 3]    = [1, 2, 3];
let blank1          = [0; 3];
let blank2: [u8; 3] = [0; 3];

// arrays是一个二维数组，其中每一个元素都是一个数组，元素类型是[u8; 3]
let arrays: [[u8; 3]; 4]  = [one, two, blank1, blank2];

// 借用arrays的元素用作循环中
for a in &arrays {
  print!("{:?}: ", a);
  // 将a变成一个迭代器，用于循环
  // 你也可以直接用for n in a {}来进行循环
  for n in a.iter() {
    print!("\t{} + 10 = {}", n, n+10);
  }

  let mut sum = 0;
  // 0..a.len,是一个 Rust 的语法糖，其实就等于一个数组，元素是从0,1,2一直增加到到a.len-1
  for i in 0..a.len() {
    sum += a[i];
  }
  println!("\t({:?} = {})", a, sum);
}
}
