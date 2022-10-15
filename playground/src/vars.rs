pub mod sub_a;
mod sub_b;

pub fn run() {
    println!("Here is vars module");
  
    let mut x = 3;
    println!("{}", x);

    x = 6;
    println!("{}", x);
}
