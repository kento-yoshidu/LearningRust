fn devide(x:i32, y: i32) -> i32 {
    x / y
}

fn devide2(x: i32, y: i32) -> Result<i32, String> {
    if y == 0 {
        Err(String::from("0除算です"))
    } else {
        Ok(x / y)
    }
}

pub fn run() {
    // println!("{}", devide(100, 0))
    // ❌ thread 'main' panicked at 'attempt to divide by zero', src\my_error_handling\my_result.rs:2:5

    // Result型が返ってくるので出力できない
    // println!("{}", devide2(100, 0));
    // ❌ `Result<i32, String>` doesn't implement `std::fmt::Display`

    // 0を渡すとErr型が返る
    println!("{:?}", devide2(100, 0));
    //=> Err("0除算です")

    // それ以外はOk型が返る
    println!("{:?}", devide2(100, 50));
    //=> Ok(2)
}
