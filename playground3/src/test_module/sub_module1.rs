pub fn test_fn1() {
    println!("Hello World");
}

fn test_fn2() {
    println!("Hello Rust");
}

#[derive(Debug)]
pub struct TestStruct {
    val1: i32,
    val2: i32
}

impl TestStruct {
    pub fn new(val1: i32, val2: i32) -> TestStruct {
        TestStruct { val1, val2 }
    }
}
