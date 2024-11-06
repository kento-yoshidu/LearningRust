use rand::Rng;

fn main() {
    let mut count = 0;

    while count < 3 {
        let mode = rand::thread_rng().gen_range(1..=2);

        match mode {
            1 => loop {
                let op1 = rand::thread_rng().gen_range(0..100);
                let op2 = rand::thread_rng().gen_range(0..100);

                println!("{} + {} = ??", op1, op2);

                let mut ans_input = String::new();

                std::io::stdin().read_line(&mut ans_input).unwrap();

                let ans_input = ans_input.trim().parse::<i32>().unwrap();

                if dbg!(ans_input == op1 + op2) {
                    println!("correct");
                    count += 1;
                    break;
                } else {
                    println!("incorrect");
                }
            },
            2 => loop {
                let op1 = rand::thread_rng().gen_range(0..100);
                let op2 = rand::thread_rng().gen_range(0..100);

                println!("{} - {} = ??", op1, op2);

                let mut ans_input = String::new();

                std::io::stdin().read_line(&mut ans_input).unwrap();

                let ans_input = ans_input.trim().parse::<i32>().unwrap();

                if dbg!(ans_input == op1 - op2) {
                    println!("correct");
                    count += 1;
                    break;
                } else {
                    println!("incorrect")
                }
            },
            _ => unreachable!(),
        }
    }

    println!("Bye");
}
