fn needEven(a: i32) -> Result<i32, String> {
    if a % 2 == 0 {
        Ok(a)
    } else {
        Err(String::from("偶数"))
    }
}

pub fn run() {
    // panic
    // println!("{}", [1, 2, 3][10]);
    // println!("{}", 1 / 0);

    // panic!("Panic!");
    //=> thread 'main' panicked at 'Panic!', src\my_error_handling.rs:6:5

    // 失敗する可能性のある処理

    println!("{:?}" ,needEven(2));
    println!("{:?}" ,needEven(1));

    let x = match needEven(2) {
        Ok(value) => value,
        Err(message) => {
            println!("Error message {}", message);
            panic!();
        }
    };

    println!("{}", x);

    let s = needEven(1);
    println!("{}", s.is_ok());
    //=> 偶数を渡しているからtrue
    println!("{}", s.is_err());
    //=> 偶数を渡しているからfalse

    // オプション型で返ってくる
    println!("{:?}", s.ok());
    //=> Some(2)
    //=> 偶数だったらNoneが返る

    // println!("{:?}", s.err());
    //=> Some("偶数")
}