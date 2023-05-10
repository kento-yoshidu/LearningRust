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

// 独自エラー
#[derive(Debug)]
enum NotBuyError {
    NotHandledFruits,
    SouldOutFruits,
}

// OddError::FoundOdd が println!() で参照されたときに出力する文字列を定義する
impl std::fmt::Display for NotBuyError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            NotBuyError::NotHandledFruits => write!(f, "その商品は取り扱いがありません"),
            NotBuyError::SouldOutFruits => write!(f, "その商品は売り切れました")
        }
    }
}

// 配列に奇数が含まれていたらエラーを発生させる
fn check_odd_digits(item: &str) -> Result<&str, NotBuyError> {
    if item == "Apple" {
        return Err(NotBuyError::NotHandledFruits)
    }
    Ok(item)
}

/*
fn buy_fruits(fruit: %str) => Result<String, NotHandledFruits> {
    match
}
*/

pub fn run() {
    let result = check_odd_digits("Apple");

    match result {
        Err(err) => println!("{}", err),
        Ok(ok) => println!("{}", ok),
    }

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

    /*
    /* 独自のエラー型を定義する */
    // これまではErr("エラーメッセージ")という感じでしたが、独自に拡張してバリエーションを持たせます

    // 様々なメソッド
    // unwrapで値を取り出すことができる

    println!("{}", devide2(100, 50).unwrap());

    //=> 0除算エラーです

    // Errを受け取るとpanicになる

    println!("{}", devide2(100, 0).unwrap());

    //=> 2
    */
}
