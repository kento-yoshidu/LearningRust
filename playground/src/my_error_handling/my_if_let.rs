#[allow(dead_code)]
pub fn run() {
    // ifは「式」である
    let result = if 1 == 1 {
        "1 = 1!"
    } else {
        "Something is wrong..."
    };

    println!("{}", result);
    //=> 1 = 1!

    // letはパターンマッチ
    // let PATTERN = EXPRESSION;

    // 論駁不可能
    // `PATTERN`はどんな値もとり得る
    let _x = 1;
    let _x = String::from("Hello World");
    let (_x, _y, _z) = (10, 10, -5);

    // PATTERNとEXPRESSIONの全体の型が一致しないので、コンパイルエラーになる
    // let (x, y) = (10, 10, -5);

    // 論駁可能
    // マッチしない可能性がある
    // （下記の型アノテーションが意味不明)
    let i: Result<i32, _> = Ok::<i32, String>(1);
    // let Ok(x) = i;
    //  ^^^^^^^ pattern `None` not covered
    // Noneをカバーしてないと言っている

    // if let構文はパターンマッチの網羅性が要らない場合に使える
    if let Ok(x) = i {
        println!("{}", x);
        //=> 1
    }
}

// https://qiita.com/nirasan/items/321e7cc42e0e0f238254

// https://qiita.com/plotter/items/0d8dc2782f21178d64f1
