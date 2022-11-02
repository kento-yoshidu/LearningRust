enum List {
    Node(i32, Box<List>),
    Nil
}

pub fn box_test() {
    let t1: (i64, String) = (10, String::from("hello"));
    println!("{:p}", &t1);
    println!("{:?}", t1.1.as_ptr());

    let mut b1 = Box::new(t1);

    // 参照外しで要素にアクセスする
    (*b1).1 += "world";

    println!("{:?}", b1);
    //=> 10, helloworld

    println!("{:p}", &b1);
    // スタック上のアドレス

    println!("{:p}", b1);
    // ヒープの先頭アドレス
}
