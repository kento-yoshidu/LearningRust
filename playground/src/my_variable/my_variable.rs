pub fn run() {
    // プリミティブ型の束縛は、値がコピーされる
    // この動きをコピーセマンティクスという
    let s = 1;
    let t = s;

    println!("s = {}, t = {}", s, t);

    // String型の束縛は、値はコピーされない
    // String型にはCopyトレイトが実装されていないため
    // 所有権の移動が起きる
    // この動きをムーブセマンティクスという
    let s = String::from("Hello");
    let t = s;

    println!("t = {}", t);
    // 🦀❌ 所有権に関するエラーが発生する
    // println!("s = {}", s);
    //                    ^ value borrowed here after move
}
