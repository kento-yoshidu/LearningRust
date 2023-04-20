// &strは参照を表す型である。スライスである。
// プログラム実行時の初期に静的領域に保存され、そこを参照している形になっている
// ずっと静的に領域に置かれているため、ライフタイムが`staticになる
// 文字列操作は限られている
// スタックには静的領域の先頭アドレス8バイトと、長さ8バイトが積まれる

pub fn run() {
    // &'static str型と推論される
    let str = "Hello World";
    println!("1: str {}", str);

    // スライスっぽく取り出せる
    println!("2: str[0..2] {:?}", &str[0..2]);

    let str2 = "Hello";
    let str3 = "Hello World";

    println!("3: str2のスタックアドレス {:p}", &str2);
    println!("3: str3のスタックアドレス {:p}", &str3);

    println!("4: str2の静的領域の先頭アドレス {:p}", str2.as_ptr());
    println!("4: str2の静的領域のバイト数 {}", str2.len());
    println!("4: str3の静的領域の先頭アドレス {:p}", str3.as_ptr());
    println!("4: str3の静的領域のバイト数 {}", str3.len());

    println!("5: こんにちはの静的領域のバイト数 {}", "こんにちは".len());
}
