mod option;

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

fn main() {
    // fizzbuzz::run();
    // ownership::run();
    func_ex_print_result(func_ex_div_result(10, 5));

    option::run();
}

