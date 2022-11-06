pub fn run() {
    let res1 = division_option(5.0, 0.0);

    match res1 {
        Some(x) => println!("Result: {:.3}", x),
        None => println!("Error")
    }

    let res2 = devision_Result(5.0, 1.2);

    match res2 {
        Ok(x) => println!("amari is {:.4}", x),
        Err(e) => println!("{}", e)
    }
}

fn division_option(x: f64, y: f64) -> Option<f64> {
    if y == 0.0 {
        None
    } else {
        Some(x / y)
    }
}

fn devision_Result(x: f64, y: f64) -> Result<f64, String> {
    if y == 0.0 {
        Err(String::from("Not allowed"))
    } else {
        Ok(x / y)
    }
}
