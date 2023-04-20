pub fn run() {
    // &strから作成する方法
    let st1 = "Hello Rust".to_string();
    println!("1: st1 {}", st1);

    //String::fromから作成する方法
    let st2 = String::from("Hello Rust");
    println!("2: st2 {}", st2);

    // push_strで&strを連結させることができる
    let mut st3 = String::from("Hello");
    st3.push_str(" World");
    println!("3: st3 {}", st3);

    // スライスを使い部分的に取り出すことができる
    println!("4: &st3[0..3] {}", &st3[0..3]);

    let s1 = String::from("Hello World, I am Rustacean!");
    let s2 = String::from("Hello World");

    println!("3: s1のスタック上の先頭アドレス {:p}", &s1);
    println!("3: s2のスタック上の先頭アドレス {:p}", &s2);

    println!("4: s1のヒープ上の先頭アドレス {:p}", s1.as_ptr());
    println!("4: s2のヒープ上の先頭アドレス {:p}", s2.as_ptr());

    println!("5: s1の文字列のバイト数 {}", s1.len());
    println!("5: s2の文字列のバイト数 {}", s2.len());
    println!("5: こんにちはのバイト数 {}", String::from("こんにちは").len());

    println!("6: s1のヒープ領域に確保されているサイズ {}", s1.capacity());
    println!("6: s2のヒープ領域に確保されているサイズ {}", s2.capacity());
}
