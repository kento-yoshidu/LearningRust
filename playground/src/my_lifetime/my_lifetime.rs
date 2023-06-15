// 参照を受け取って出力する関数
fn my_print<T: std::fmt::Display>(msg: &T) {
    // デリファレンス
    // リファレンスが指し示す値をたどる
    println!("1: my_print関数 = {}", *msg);
}

// 可変参照を受けとり削除する
fn my_clear(x: &mut String) {
    x.clear();
    println!("2: my_clear関数 = {}", x);
}

pub fn run() {
    let s = "Hello World";

    my_print(&s);
    // 上では参照で渡しているため、所有権を失っていない
    my_print(&s);

    // イミュータブルなリファレンスは複数同時に作成できる
    let s_ref = &s;
    let s_ref2 = &s;
    my_print(s_ref);
    my_print(s_ref2);

    let mut s2 = String::from("Hello World");
    my_clear(&mut s2);
}
