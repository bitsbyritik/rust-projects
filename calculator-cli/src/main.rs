 use std::{f64, io};

fn main() {
    println!("Simple Calculator");
    println!("Availabe operations are: +, -, *, /, % ");
    println!("Enter your expressions (e.g. 8 + 3)");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let tokens: Vec<&str> = input.trim().split_whitespace().collect();

    if tokens.len() < 3 {
        println!("Invalid input");
        return;
    }

    match evaluate_expression(tokens) {
        Ok(result) => println!("{:2}", result),
        Err(err) => println!("{}", err),
    }
}

fn evaluate_expression(token: Vec<&str>) -> Result<f64, String> {
    let mut values: Vec<f64> = Vec::new();
    let mut ops: Vec<&str> = Vec::new();

    let mut i = 0;
    let mut current = token[i].parse::<f64>().map_err(|_| "Invalid number")?;
    i += 1;

    while i < token.len() {
        let op = token[i];
        if i + 1 >= token.len() {
            return Err("Inavlid expression".into());
        }
        let next_num = token[i + 1].parse().map_err(|_| "Invalid number")?;

        match op {
            "*" => current *= next_num,
            "/" => {
                if next_num == 0.0 {
                    return Err("Division by zero".into());
                }
                current /= next_num
            }
            "%" => {
                if next_num == 0.0 {
                    return Err("Modulo by zero".into());
                }
                current = current.rem_euclid(next_num)
            }
            "+" | "-" => {
                values.push(current);
                ops.push(op);
                current = next_num
            }
            _ => return Err("Invalid operations".into()),
        }
        i += 2;
    }

    values.push(current);

    let mut result = values[0];
    for (j, &op) in ops.iter().enumerate() {
        match op {
            "+" => result += values[j + 1],
            "-" => result -= values[j + 1],
            _ => {}
        }
    }

    Ok(result)
}