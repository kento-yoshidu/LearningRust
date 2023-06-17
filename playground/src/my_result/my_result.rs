/* 基本文法 */

#[allow(dead_code)]
fn devide(x: i32, y: i32) -> i32 {
    x / y
}

#[allow(dead_code)]
fn devide2(x: i32, y: i32) -> Result<i32, String> {
    if y == 0 {
        Err(String::from("0除算エラーです"))
    } else {
        Ok(x / y)
    }
}

#[allow(dead_code)]
fn call_devide2(x: i32, y: i32) -> Result<i32, String> {
    // エラーだった場合は、call_devide2の呼び出しもとにErrが返る
    let ok = devide2(x, y)?;

    println!("call_deive2 = {}", ok);

    Ok(ok)
}

#[allow(dead_code)]
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
    match devide2(100, 2) {
        Ok(value) => println!("{}", value),
        Err(err) => println!("{}", err)
        //=> 0除算エラーです
    }

    /* if let構文 */
    // パターンマッチの網羅性が要らない場合に使える
    // Result型やOption形で使っても効果は薄いが、Enumと組み合わせるといい

    // Okの時の処理だけを記述する(Errの処理は書かなくていい)
    if let Ok(v) = devide2(100, 2) {
        println!("if...letでOkの時の処理を記述 {:?}", v);
        //=> if...letでOkの時の処理を記述 50
    }

    // if...let構文でErrになる場合
    // これは何も起こらない
    if let Ok(value) = devide2(100, 0) {
        println!("🦀👀 {:?}", value);
    }

    // Errの時に拾いたいなら以下のようにする
    if let Err(err) = devide2(100, 0) {
        println!("if letでErrの時の処理を記述 {}", err);
        //=> if letでErrの時の処理を記述 0除算エラーです
    }

    // エラーの移譲
    match call_devide2(100, 0) {
        Ok(value) => println!("call_devide2の呼び出しの結果 Ok {}", value),
        Err(err) => println!("call_devide2の呼び出しの結果 Err {}", err)
        //=> call_devide2の呼び出しの結果 Err 0除算エラーです
    }
}
