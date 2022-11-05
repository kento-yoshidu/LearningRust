/*
use std::mem::take;

fn take_ownership(s: String) {
  println!("{}", s);
}

fn take_giveback_ownership(s: String) -> String {
  s
}

fn calculate_length(s: &String) -> usize {
  s.len()
}

fn change(s: &mut String) {
  s.push_str(" World!");
}
*/

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

    let s5 = String::from("hello");

    take_ownership(s5);

    println!("{}", s5);

    let s6 = String::from("test");
    println!("{:p}", s6.as_ptr());
    let s7 = take_giveback_ownership(s6);
    println!("{:p}", s7.as_ptr());

    let s8 = String::from("Hello");
    let s8len = calculate_length(&s8);
    println!("{}, {}", s8, s8len)

    let mut s9 = String::from("Hello");

    change(&mut s9);

    println!("{}", s9);

    let s10 = String::from("hello");
    let r1 = &s10;
    let r2 = &s10;
    println!("{}, {}, {}", s10, r1, r2);

    // let mut s10 = String::from("Hello");
    // let r = &s10;
    // let m = &mut s10;

    // println!("{}, {}", r, m);

    let mut s11 = String::from("Hello");
    let m = &mut s11;
    println!("{}", m);
    println!("{}", s11);
    */

    let mut s12 = String::from("Hello");
    let r1 = &s12;
    let r2 = &s12;

    println!("{}{}{}", s12, r1, r2);

    let m = &mut s12;
    *m = String::from("I love Rust.");
    println!("{}", m);
}
