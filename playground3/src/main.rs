use std::env;
use std::io;
use std::fs::File;
use std::io::prelude::*;
use std::fs;
use std::io::BufReader;

/*
mod option;
mod test_module;
mod my_trait;
mod my_derive;
mod my_generics;
mod my_error_handling;
mod my_clojure;
mod iterator;
mod my_vec;
mod my_queue;

mod my_map;


fn func_ex_div_result(x: i32, y: i32) -> Result<i32, &'static str> {
    if y == 0 {
        Err("div by zero")
    } else {
        Ok(x / y)
    }
}

fn func_ex_print_result<T: std::fmt::Display, E: std::fmt::Display>(ans: Result<T, E>) {
    match ans {
        Ok(res) => println!("{}", res),
        Err(str) => println!("{}", str)
    }
}

// mod fizzbuzz;
// mod ownership;

#[test]
fn test_sample() {
    let a = 1 + 1;
    let b = 2;

    assert_eq!(a, b);
    assert_ne!(a, 3);
}
*/

fn maybe_panic(flag: bool) {
    if flag == false {
        println!("sage!");
        panic!("dummy");
    } else {
        panic!("flag is true!!");
    }
}

#[cfg(test)]
mod test_module {
    #[test]
    #[should_panic(expected="flag is true")]
    fn test_maybe_panic() {
        super::maybe_panic(true);
    }
}

fn main() {
    /*
    fizzbuzz::run();
    ownership::run();
    func_ex_print_result(func_ex_div_result(10, 5));

    option::run();

    crate::test_module::sub_module1::test_fn1();
    self::test_module::sub_module1::test_fn1();
    self::test_module::sub_module2::test_fn1();

    let val = test_module::sub_module1::TestStruct::new(10, 20);

    println!("{:?}", val);

    my_trait::run();

    my_derive::run();

    my_generics::run();

    my_error_handling::run();

    my_clojure::run();

    iterator::run();

    my_vec::run();

    my_queue::run();

    my_map::run();

    let args: Vec<String> = env::args().collect();

    println!("{:?}", args)

    println!("文字列を入力してください。");
    let mut input = String::new();


    println!("{:?}", input);
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let num: i32 = input.trim().parse().unwrap();

    println!("{}", num * 10);
    */

    let mut f = File::open("src/sample1.txt").unwrap();
    let mut contents = String::new();

    f.read_to_string(&mut contents).unwrap();


    print!("{}", contents);

    let con = fs::read_to_string("src/sample1.txt").unwrap();

    println!("{}", con);

    let mut buffer_reader = BufReader::new(f);
    let mut line = String::new();
    buffer_reader.read_line(&mut line).unwrap();

    println!("{}", line);
    println!("{}", line);
    println!("{}", line);
}
