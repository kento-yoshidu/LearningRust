fn echo(s: String) {
    println!("{}", s);
}

pub fn my_ownership() {
    let i1 = 1;
    let i2 = 2;

    println!("1: 変数i1のスタック上のアドレス = {:p}", &i1);
    //=> 1: 変数i1のスタック上のアドレス = 0xc6c74ff7b0
    println!("1: 変数i2のスタック上のアドレス = {:p}", &i2);
    //=> 1: 変数i2のスタック上のアドレス = 0xc6c74ff7b4

    let f1 = 1.0;
    let f2 = 2.0;

    println!("2: 変数f1のスタック上のアドレス : {:p}", &f1);
    //=> 2: 変数f1のスタック上のアドレス : 0xd8a56ff8d8

    println!("2: 変数f2のスタック上のアドレス : {:p}", &f2);
    //=> 2: 変数f2のスタック上のアドレス : 0xd8a56ff8e0

    // [i32; 3]と推論される
    let arr = [1, 3, 5];

    for (index, num) in arr.iter().enumerate() {
        println!("3: 配列arrの{}番目の値 {}", index + 1 , num);
        //=> 3: 配列arrの1番目の値 1
        //=> 3: 配列arrの2番目の値 3
        //=> 3: 配列arrの3番目の値 5
    }

    let arr = [1, 3, 5];

    println!("4: 配列arrの1番目の値のスタックアドレス {:p}", &arr[0]);
    println!("4: 配列arrの2番目の値のスタックアドレス {:p}", &arr[1]);
    println!("4: 配列arrの3番目の値のスタックアドレス {:p}", &arr[2]);
    //=> 4: 配列arrの1番目の値のスタックアドレス 0x8d514ff814
    //=> 4: 配列arrの2番目の値のスタックアドレス 0x8d514ff818
    //=> 4: 配列arrの3番目の値のスタックアドレス 0x8d514ff81c

    let s1 = String::from("Hello World, I am Rustacean!");
    let s2 = String::from("Hello Rust");

    println!("5: s1の値 {}", s1);
    //=> 5: s1の値 Hello World, I am Rustacean!
    println!("5: s2の値 {}", s2);
    //=> 5: s2の値 Hello Rust

    // これらの変数はスタックに積まれている
    println!("6: s1のスタックアドレスの値 {:p}", &s1);
    //=> 6: s1のスタックアドレスの値 0x325370f520

    println!("6: s2のスタックアドレスの値 {:p}", &s2);
    //=> 6: s2のスタックアドレスの値 0x325370f538

    // ヒープ領域に格納されているデータの先頭アドレス
    println!("7: s1のヒープ領域の実データの先頭アドレス {:p}", &s1.as_ptr());
    //=> 7: s1のヒープ領域の実データの先頭アドレス 0xb90c19fb68
    println!("7: s2のヒープ領域の実データの先頭アドレス {:p}", &s2.as_ptr());
    //=> 7: s2のヒープ領域の実データの先頭アドレス 0xb90c19fbb8

    println!("8: s1の実データのデータ長 {}", s1.len());
    //=> 8: s1の実データのデータ長 28
    println!("8: s2の実データのデータ長 {}", s2.len());
    //=> 8: s2の実データのデータ長 10

    println!("🦀 こんにちはのデータ長 {}", String::from("こんにちは").len());
    //=> 🦀 こんにちはのデータ長 15

    println!("🦀 こんにちはの文字数 {}", String::from("こんにちは").chars().count());
    //=> 🦀 こんにちはの文字数 5

    println!("9: s1のヒープ領域に確保される領域のサイズ {}", s1.capacity());
    //=> 9: s1のヒープ領域に確保される領域のサイズ 28
    println!("9: s2のヒープ領域に確保される領域のサイズ {}", s2.capacity());
    //=> 9: s2のヒープ領域に確保される領域のサイズ 10

    // 所有権の移動が起きる
    let s2 = s1;

    // println!("🦀❓ s1の値 = {}", s1);
    //                             ^^ value borrowed here after move

    let s3 = String::from("Hello Rust");

    println!("10: s3のヒープ領域上のメモリーアドレス = {:p}", s3.as_ptr());
    //=> 10: s3のヒープ領域上のメモリーアドレス = 0x1d3f997dbb0

    let s4 = s3;

    println!("10: s4のヒープ領域上のメモリーアドレス = {:p}", s4.as_ptr());
    //=> 10: s4のヒープ領域上のメモリーアドレス = 0x1d3f997dbb0

    // echo関数に所有権の移動が起きる
    echo(s4);

    // エラー
    // println!("🦀❓ s4にアクセスできる? {}", s4);
    //                                      ^^ value borrowed here after move

    /* コピートレイト */
    let i3 = 10;

    let i4 = i3;

    println!("i3とi4に同時にアクセスできる i3 = {}, i4 = {}", i3, i4);
    //=> i3とi4に同時にアクセスできる i3 = 10, i4 = 10
}
