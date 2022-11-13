fn concat(a: String, b: String) -> String {
    let c = format!("{}, {}", a, b);

    c
}

pub fn run() {
    // 変数aが100の所有者となる
    let a = 100;

    {
        // 変数v1がベクター値の所有者となる
        let mut v1 = vec![1, 2, 3];

        // ベクター値の所有権がv2に移動する
        let v2 = v1;

        // v1は所有権を持っていないため、コンパイルエラー
        // v1.push(4)

    }

    let s1 = String::from("Hello");
    let s2 = String::from("Rust");
    let s = concat(s1, s2);

    // 所有権が移動している
    // println!("{}", s1);
    // println!("{}", s2);

    let mut v1 = vec![1, 2, 3];

    // v1が指す値の参照
    println!("v1 ptr : {:?}", v1.as_ptr());
    // 値のアドレス
    println!("v1[0] : {:p}", &v1[0]);


    println!("v1 ptr : {:?}", v1.as_ptr());
    //=> v1 ptr : 0x2ad7de5dbd0

    let v2 = v1;
    println!("v2 ptr : {:?}", v2.as_ptr());
    //=> v2 ptr : 0x2ad7de5dbd0

    let v3 = v2.clone();

    println!("v2 ptr : {:?}", v2.as_ptr());
    //=> v2 ptr : 0x1d355ebded0
    println!("v3 ptr : {:?}", v3.as_ptr());
    //=> v3 ptr : 0x1d355ebdc10
}
