use std::io::stdin;

fn main() {
    for line in stdin().lines() {
        let line = line.unwrap();

        if line.is_empty() {
            println!("Bye.");

            break;
        }

        let tokens: Vec<&str> = line.split(char::is_whitespace).collect();

        let left = tokens[0].parse::<f64>().unwrap();
        let right = tokens[2].parse::<f64>().unwrap();

        let result = match tokens[1] {
            "+" => left + right,
            "-" => left - right,
            "*" => left * right,
            "/" => left / right,
            "%" => left % right,
            _ => unreachable!(),
        };

        println!("{:?}", result);
    }
}
