/*
use thiserror::Error;

fn func_ex_div_result(x: i32, y: i32) -> Result<i32, &'static str> {
    if y == 0 {
        Err("div by zero")
    } else {
        Ok(x / y)
    }
}

fn func_ex_print_result<T: std::fmt::Display, E: std::fmt::Display>(ans: Result<T, E>) {
    match ans {
        Ok(res) => println!("{}", res),
        Err(str) => println!("{}", str)
    }
}
*/

#[derive(Error, Debug)]
enum DivError {
    #[error("{0} divided ny zero")]
    DivByZero(i32),
    #[error("Both numerator {0} and denominator {1} are negative")]
    BothNegative(i32, i32)
}

fn myDiv(x: i32, y: i32) -> Result<i32, DivError> {
    if y == 0 {
        Err(DivError::DivByZero(x))
    } else if x < 0 && y < 0 {
        Err(DivError::BothNegative(x, y))
    } else {
        Ok(x / y)
    }
}

fn print_mydiv(x: i32, y: i32) {
    match myDiv(x, y) {
        Ok(ans) => println!("no error. ans = {}", ans),
        Err(DivError::DivByZero(a)) => {
            println!("{} divided by zero", a)
        },
        Err(DivError::BothNegative(a, b)) => {
            println!(
                "Both numerator {} and denominator {} are negative", a, b
            )
        }
    }
}

fn print_mydiv2(x: i32, y: i32) {
    let ret = myDiv(x, y);

    if ret.is_ok() {
        println!("no error. ans = {}", ret.unwrap())
    } else {
        println!("{}", ret.err().unwrap())
    }
}

fn repeat_mydiv(arr: &[(i32, i32)]) -> Result<Vec<i32>, DivError> {
    let mut ret = Vec::new();

    for aa in arr {
        // エラーが返されると中断。
        // mydiv()のエラーをrepeat_mydiv()のエラーとして返す
        let ans = myDiv(aa.0, aa.1)?;
        ret.push(ans);
        println!("pushed: {} / {} = {}", aa.0, aa.1, ans)
    }
    Ok(ret)
}

fn print_repeat_mydiv(result: Result<Vec<i32>, DivError>) {
    match result {
        Ok(v) => println!("{:?}", v),
        Err(e) => println!("{}", e)
    }
}

pub fn run() {
    func_ex_print_result(func_ex_div_result(10, 5));
    func_ex_print_result(func_ex_div_result(10, 0));

    print_mydiv(5, 2);
    print_mydiv(5, 0);
    print_mydiv(-1, -1);

    print_mydiv2(5, 2);
    print_mydiv2(5, 0);
    print_mydiv2(-1, -1);

    print_repeat_mydiv(repeat_mydiv(&[(2, 1), (9, 3)]));

    print_repeat_mydiv(repeat_mydiv(&[(2, 1), (-6, -3), (5, 2)]));

    print_repeat_mydiv(repeat_mydiv(&[(2, 1), (-6, 0), (6, 3)]));
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

fn run() {
    /* 独自のエラー型を定義する */
    // これまではErr("エラーメッセージ")という感じでしたが、独自に拡張してバリエーションを持たせます
    let result = check_odd_digits("Apple");

    match result {
        Err(err) => println!("{}", err),
        Ok(ok) => println!("{}", ok),
    }
}
