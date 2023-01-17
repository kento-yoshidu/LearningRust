use std::ops::RemAssign;

// 色んな種類のエラーを定義
enum Error {
    // 0で割ったらエラーメッセージを返す
    DivZero(&'static str),
    // 分母も分子もマイナスだったらエラーメッセージを返す
    BothNegative(&'static str)
}

fn division(num1: i32, num2: i32) -> Result<i32, Error> {
    if num2 == 0 {
        Err(Error::DivZero("0除算です。"))
    } else if num1 < 0 && num2 < 0 {
        Err(Error::BothNegative("両方とも負の数です。"))
    } else {
        Ok(num1 / num2)
    }
}

fn main() {
    let result = division(-1, -1);

    match result {
        Ok(result ) => println!("割り算成功! {}", result),
        Err(Error::DivZero(message)) => println!("エラー発生 {}", message),
        Err(Error::BothNegative(message)) => println!("エラー発生 {}", message)
    }
}

// https://zenn.dev/ucwork/articles/6de5c9c2257f2d
// https://zenn.dev/tokeiya3/articles/8f9a9b9069c0db

