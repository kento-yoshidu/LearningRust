pub fn run() {
    let v = vec![1, 2, 11];
    // valはOption型
    let val = v.get(2);

    // matchと組み合わせて、値がある場合とない場合で処理を分ける
    // 可能性があるパターンを全て書く必要がある
    match val {
        Some(x) => println!("{}", x),
        None => println!("No Data")
    }

    // if-let
    if let Some(x) = val {
        println!("{}", x)
    }

    // マッチガード
    match val {
        // 1の時
        Some(1) => println!("value is 1!"),

        // 2から10の時
        Some(2..=10) => println!("2~10"),

        // それ以外の時
        Some(x) => println!("value is {}", x),

        // 値がない時
        None => {println!("None")}
    }
}
