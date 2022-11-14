fn my_match_iflet(num: i32) {
    let x = num % 2;

    let result = match x {
        0 => Some("even"),
        _ => None
    };

    match result {
        Some(x) => {
            println!("num is {}", x)
        },
        None => {
            println!("num is odd")
        }
    }
}

pub fn run() {
    my_match_iflet(2);
    my_match_iflet(5);
}