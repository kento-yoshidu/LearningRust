use std::io::stdin;

struct Memory {
    slots: Vec<(String, f64)>,
}

fn main() {
    let mut memory = Memory {
        slots: vec![],
    };

    let mut prev_memory = 0.0;

    for line in stdin().lines() {
        let line = line.unwrap();

        if line.is_empty() {
            println!("Bye.");

            break;
        }

        let tokens: Vec<&str> = line.split(char::is_whitespace).collect();

        let is_memory = tokens[0].starts_with("mem");

        if is_memory && tokens[0].ends_with('+') {
            add_and_print_memory(&mut memory, tokens[0], prev_memory);
            continue;
        } else if is_memory && tokens[0].ends_with('-') {
            add_and_print_memory(&mut memory, tokens[0], -prev_memory);
            continue;
        }

        let left = eval_token(tokens[0], &memory);
        let right = eval_token(tokens[2], &memory);

        let result = eval_expression(left, tokens[1], right);

        print_output(result);

        prev_memory = result;
    }
}

fn print_output(value: f64) {
    println!(" => {value}");
}

fn eval_token(token: &str, memory: &Memory) -> f64 {
    if token.starts_with("mem") {
        let slot_name = &token[3..];

        for slot in &memory.slots {
            if slot.0 == slot_name {
                return slot.1;
            }
        }

        0.0
    } else {
        token.parse().unwrap()
    }
}

fn eval_expression(left: f64, operator: &str, right: f64) -> f64 {
    match operator {
        "+" => left + right,
        "-" => left - right,
        "*" => left * right,
        "/" => left / right,
        _ => unreachable!(),
    }
}

fn add_and_print_memory(memory: &mut Memory, token: &str, prev_result: f64) {
    let slot_name = &token[3..token.len()-1];

    for slot in memory.slots.iter_mut() {
        if slot.0 == slot_name {
            slot.1 += prev_result;
            print_output(slot.1);
            return;
        }
    }

    memory.slots.push((slot_name.to_string(), prev_result));
    print_output(prev_result);;
}
