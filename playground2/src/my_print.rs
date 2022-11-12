pub fn run() {
    let s = String::from("hello");

    my_print(&s);
    my_print(&s);
}

fn my_print<T: std::fmt::Display>(msg: &T) {
    println!("{}", msg)
}
