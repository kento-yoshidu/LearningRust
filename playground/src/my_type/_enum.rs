use std::cmp::Ordering;

#[derive(Debug)]
enum Sign {
    Positive,
    Zero,
    Negative
}

// Sign列挙体を返す関数
fn determine_sign(x: i32) -> Sign {
    if x > 0 {
        Sign::Positive
    } else if x < 0 {
        Sign::Negative
    } else {
        Sign::Zero
    }
}

fn determine_sign_2(x: i32) -> Sign {
    // cmp => xと引数の値を比べる。Ordering型が返ってくる
    match x.cmp(&0) {
        Ordering::Greater => Sign::Positive,
        Ordering::Less => Sign::Negative,
        Ordering::Equal => Sign::Zero
    }
}

pub fn run() {
    let num = determine_sign(10);


    // matchと組み合わせることで全てのパターンを網羅することができる
    // もし漏れがあるとコンパイル時に検出できる
    match num {
        Sign::Positive => println!("正の数"),
        Sign::Negative => println!("負の数"),
        Sign::Zero => println!("0")
    }

    let num2 = determine_sign_2(10);

    println!("{:?}", num2);
    //=> Positive
}
