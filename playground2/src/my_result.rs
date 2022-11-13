use thiserror::Error;

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

#[derive(Error, Debug)]
enum DivError {
    #[error("{0} divided ny zero")]
    DivByZero(i32),
    #[error("Both numerator {0} and denominator {1} are negative")]
    BothNegative(i32, i32)
}

fn myDiv(x: i32, y: i32) -> Result<i32, DivError> {
    if y == 0 {
        Err(DivError::DivByZero(x))
    } else if x < 0 && y < 0 {
        Err(DivError::BothNegative(x, y))
    } else {
        Ok(x / y)
    }
}

fn print_mydiv(x: i32, y: i32) {
    match myDiv(x, y) {
        Ok(ans) => println!("no error. ans = {}", ans),
        Err(DivError::DivByZero(a)) => {
            println!("{} divided by zero", a)
        },
        Err(DivError::BothNegative(a, b)) => {
            println!(
                "Both numerator {} and denominator {} are negative", a, b
            )
        }
    }
}

pub fn run() {
    func_ex_print_result(func_ex_div_result(10, 5));
    func_ex_print_result(func_ex_div_result(10, 0));

    print_mydiv(5, 2);
    print_mydiv(5, 0);
    print_mydiv(-1, -1);
}