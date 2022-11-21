fn main() {
    greet_world(true);
    variables_1(true);
    variables_mutable(true);
    variables_mutable_2(true);
    variables_struct(true);
    consts(true);
    variables_shadowing(true);
}

fn greet_world(enabled: bool) {
    if !enabled {
        return;
    }
    let southern_germany = "Grüß Gott!";
    let chinese = "你好，世界！";
    let japanese = "こんにちは世界！";
    let polish = "Witaj świecie!";
    let spanish = "¡Hola mundo!";
    let french = "Bonjour le monde!";
    let portuguese = "Olá mundo!";
    let italian = "Ciao mondo!";
    let regions = [southern_germany, chinese, japanese, polish, spanish, french, portuguese, italian];
    for region in regions.iter() {
        println!("{}", region);
    }
}

fn variables_1(enabled: bool) {
    if !enabled {
        return
    }

    let penguin_data = "\
    common name,length (cm)
    Little penguin,33
    Yellow-eyed penguin,65
    Fiordland penguin,60
    Invalid,data
    ";

    let records = penguin_data.lines();

    for (i, record) in records.enumerate() {
        if i == 0 || record.trim().len() == 0 {
            continue;
        }

        let fields: Vec<_> = record
        .split(',')
        .map(|f| f.trim())
        .collect();

        if cfg!(debug_assertions) {
            eprintln!("record: {:?}, fields: {:?}", record, fields);
        }

        let name = fields[0];
        if let Ok(length) = fields[1].parse::<f32>() {
            println!("{}, {} cm", name, length);
        } else {
            eprintln!("Invalid record: {}", record);
        }
    }

}

fn variables_mutable(enabled: bool) {
    if !enabled {
        return
    }

    let mut a = 5;
    println!("a = {}", a);
    a = 6;
    println!("a = {}", a);
}

fn variables_mutable_2(enabled: bool) {
    if !enabled {
        return
    }
    let (a, mut b) = (true, false);
    println!("a = {}, b = {}", a, b);
    b = true;
    println!("a = {}, b = {}", a, b);
}

fn variables_struct(enabled: bool) {
    if !enabled {
        return
    }
    struct Point {
        x: f32,
        y: f32,
    }

    let (a, b, c, d, x, y);
    (a, b) = (0, 1);
    [.., c, d, _, _] = [0, 1, 2, 3, 4, 5];
    Point { x, y } = Point { x: 0.3, y: 0.4 };
    println!("a = {}, b = {}, c = {}, d = {}, x = {}, y = {}", a, b, c, d, x, y);
}

fn consts(enabled: bool) {
    if !enabled {
        return
    }
    const PI: f32 = 3.141592;
    println!("PI = {}", PI);
}

fn variables_shadowing(enabled: bool) {
    if !enabled {
        return
    }
    let x = 5;
    let x = x + 1;  // this is a new 'x', not the same as the previous 'x'
    {
        let x = x * 2;
        println!("x = {}", x);
    }
    println!("x = {}", x);
}
