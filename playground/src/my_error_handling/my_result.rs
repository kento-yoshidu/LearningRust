fn devide(x: i32, y: i32) -> i32 {
    x / y
}

fn devide2(x: i32, y: i32) -> Result<i32, String> {
    if y == 0 {
        Err(String::from("0除算エラーです"))
    } else {
        Ok(x / y)
    }
}

fn call_devide2(x: i32, y: i32) -> Result<i32, String> {
    // エラーだった場合は、call_devide2の呼び出しもとにErrが返る
    let ok = devide2(x, y)?;

    println!("call_deive2 = {}", ok);

    Ok(ok)
}

enum NotHandledFruits {
    Apple
}

pub fn run() {
    // 0除算でパニックになる
    //println!("{}", devide(100, 0));

    // Result型のインスタンスが返ってくるため、エラーになる
    // println!("{}", devide2(100, 0))

    // println!("{:?}", devide2(100, 0));
    //=> Err("0除算エラーです")

    println!("{:?}", devide2(100, 50));
    //=> Ok(2)

    // match式と組み合わせる
    match devide2(100, 0) {
        Ok(value) => println!("{}", value),
        Err(err) => println!("{}", err)
        //=> 0除算エラーです
    }

    // エラーの移譲
    match call_devide2(100, 0) {
        Ok(value) => println!("call_devide2の呼び出しの結果 Ok {}", value),
        Err(err) => println!("call_devide2の呼び出しの結果 Err {}", err)
        //=> call_devide2の呼び出しの結果 Err 0除算エラーです
    }

}
