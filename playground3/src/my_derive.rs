#[derive(Debug, PartialEq)]
struct S {
    val1: i32,
    val2: i32
}

pub fn run() {
    println!("{:?}", S { val1: 10, val2: 20 });

    let s1 = S {
        val1: 1,
        val2: 2
    };

    let s2 = S {
        val1: 1,
        val2: 2
    };


    println!("{}", s1 == s2);
}