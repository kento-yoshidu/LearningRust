/* 値がない可能性がある型を表現 */

enum Option<T> {
    None,
    Some(T)
}


fn func_ex_div_some(x: i32, y: i32) -> Option<i32> {
    if y == 0 {
        None
    } else {
        Some(x / y)
    } 
}

fn func_ex_div_some_match(x: i32, y: i32) -> Option<i32> {
    match y {
        0 => None,
        _ => Some(x / y)
    }
}

fn my_opiton_print<T: std::fmt::Display>(ans: Option<T>) {
    if let Some(x) = ans {
        println!("{}", x)
    } else {
        println!("None")
    }
}

pub fn run() {
    println!("{}", func_ex_div_some(6, 2));
}
