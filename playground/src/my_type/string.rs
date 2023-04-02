pub fn run() {
    let string = String::from("Hello World");
    println!("1: string {}", string);

    println!("2: &string[0..2] {}", &string[0..2]);
}
