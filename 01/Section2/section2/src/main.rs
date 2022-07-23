fn main() {
    let mut s1 = String::from("hello");
    let mut s2 = String::from("helloworld");

    // ヒープメモリーアドレス
    println!("{:?}", s1.as_ptr());
    println!("{}", s1.len());
    println!("{}", s1.capacity());
    
    s1.push_str("_new1");
}
