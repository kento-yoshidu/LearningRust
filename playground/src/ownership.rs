use std::mem::take;

fn take_ownership(s: String) {
  println!("{}", s);
}

pub fn run() {
  /*
    let s1 = String::from("hello");
    let s2 = s1;

    // println!("{}, {}", s1, s2);

    let i1 = 1;
    let i2 = i1;

    println!("{}, {}", i1, i2);
    //=> 1, 1
    println!("{:p}, {:p}", &i1, &i2);
    //=> 0xaa35d4f7a0, 0xaa35d4f7a4

    let s3 = String::from("hello");
    let s4 = s3.clone();

    println!("{}, {}", s3, s4);

    println!("{:p}, {:p}", &s3, &s4);
    //=> 0x8d39d6f6c8, 0x8d39d6f6e0
    */

    let s5 = String::from("hello");

    take_ownership(s5);

    println!("{}", s5);
}
