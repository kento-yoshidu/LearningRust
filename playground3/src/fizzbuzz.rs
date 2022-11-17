fn fizzbuzz_while(end: u8) {
    let mut x = 1;

    while x <= end {
        if x % 3 == 0 && x % 5 == 0 {
            println!("FizzBuzz")
        } else if x % 3 == 0 {
            println!("Fizz");
        } else if x % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", x);
        }
        x += 1;
    }
}

fn fizzbuzz_match(end: u8) {
    let r = 1..=end;

    for i in r {
        match i % 15 {
            0 => println!("FizzBuzz"),
            3 | 6 | 9 | 12 => println!("Fizz"),
            5 | 10 => println!("Buzz"),
            _ => println!("{}", i)
        }

    }
}

fn fizzbuzz_match_tuple(end: u8) {
    let r = 1..=end;

    for i in r {
        match (i % 3, i % 5) {
            (0, 0) => println!("FizzBuzz"),
            (0, _) => println!("Fizz"),
            (_, 0) => println!("Buzz"),
            _ => println!("{}", i)
        }
    }
}

pub fn run() {
    fizzbuzz_match_tuple(30);
}