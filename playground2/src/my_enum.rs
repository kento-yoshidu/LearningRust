use std::cmp::Ordering;

enum Sign {
    Positive,
    Zero,
    Negative
}

/*
 * 列挙型とmatchでパターンを漏らさずに分岐処理を書ける
 */

fn determine_sign(x: i8) -> Sign {
    match x.cmp(&0) {
        Ordering::Greater => Sign::Positive,
        Ordering::Less    => Sign::Negative,
        Ordering::Equal   => Sign::Zero
    }
}

fn print_sign(s: Sign) {
    match s {
        Sign::Positive => println!("+"),
        Sign::Zero     => println!("0"),
        Sign::Negative => println!("-")
    }
}

// 列挙子にデータを持たせる
enum EnumExample {
    TupleTypeExample1(String),
    TupleTypeExample2(i32, bool),
    StructTypeExample {name: String, age: u8 }
}

pub fn run() {
    print_sign(determine_sign(1));
    print_sign(determine_sign(-1));
    print_sign(determine_sign(0));

    let mut v: Vec<EnumExample> = Vec::new();

    v.push(EnumExample::TupleTypeExample1(String::from("hello")));
    v.push(EnumExample::TupleTypeExample2(10, true));
    v.push(EnumExample::StructTypeExample {
        name: String::from("Kento"),
        age: 24
    });

    // if-letで列挙子のパターンを取り出す。
    for e in &v {
        // StructTypeExampleのみ出力
        if let EnumExample::StructTypeExample { name: n, age: a} = e {
            println!("StructTypeExample_iflet: name={}, age={}", n, a)
        }
    }

    for e in &v {
        match e {
            EnumExample::TupleTypeExample1(s) => {
                println!("TupleTypeExample1: s = {}", s)
            }
            EnumExample::TupleTypeExample2(n, b ) => {
                println!("TupleTypeExample2: n = {}, b = {}", n, b)
            }
            EnumExample::StructTypeExample { name: n , .. } => {
                println!("StructTypeExample: name = {}", n)
            }
        }
    }
}
