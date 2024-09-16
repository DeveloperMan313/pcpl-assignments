use colored::Colorize;
use std::env;
use std::io;
use std::num::ParseFloatError;

fn input_f32(retry_msg: &str) -> f32 {
    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_n) => {
                input = input.trim().to_owned();
            }
            Err(_error) => {
                println!("{}", retry_msg);
                continue;
            }
        }
        match input.parse::<f32>() {
            Ok(f) => {
                return f;
            }
            Err(_error) => {
                println!("{}", retry_msg);
                continue;
            }
        }
    }
}

fn parse_f32(str: &String, f: &mut f32) -> Result<f32, ParseFloatError> {
    match str.parse::<f32>() {
        Ok(result) => {
            *f = result;
            return Ok(result);
        }
        Err(err) => Err(err),
    }
}

fn print_roots(roots: &[f32]) {
    let roots_joined: String = roots
        .into_iter()
        .map(|f| f.to_string().green().to_string())
        .collect::<Vec<String>>()
        .join(", ");
    match roots.len() {
        0 => println!("{}", "No roots".red()),
        1 => println!("One root:\n{}", roots_joined),
        2 => println!("Two roots:\n{}", roots_joined),
        3 => println!("Three roots:\n{}", roots_joined),
        4 => println!("Four roots:\n{}", roots_joined),
        _ => panic!("Too many roots!"),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 1 && args.len() != 4 {
        println!("Must be called with 0 or 3 args");
        return;
    }
    let (mut a, mut b, mut c) = (0.0, 0.0, 0.0);
    if args.len() == 1 {
        let retry_msg = "Wrong format. Please input again:";
        println!("Input coefficient A:");
        a = input_f32(&retry_msg);
        println!("Input coefficient B:");
        b = input_f32(&retry_msg);
        println!("Input coefficient C:");
        c = input_f32(&retry_msg);
    } else {
        if parse_f32(&args[1], &mut a).is_err() {
            println!("Wrong format of A");
            return;
        }
        if parse_f32(&args[2], &mut b).is_err() {
            println!("Wrong format of B");
            return;
        }
        if parse_f32(&args[3], &mut c).is_err() {
            println!("Wrong format of C");
            return;
        }
    }
    let descriptor = b * b - 4.0 * a * c;
    let mut squares = Vec::<f32>::new();
    let mut roots = Vec::<f32>::new();
    if descriptor == 0.0 {
        squares.push(-b / 2.0 / a);
    } else if descriptor > 0.0 {
        let desc_sqrt = f32::sqrt(descriptor);
        squares.push((-b - desc_sqrt) / 2.0 / a);
        squares.push((-b + desc_sqrt) / 2.0 / a);
    }
    for square in squares {
        if square == 0.0 {
            roots.push(0.0);
        } else if square > 0.0 {
            roots.push(-f32::sqrt(square));
            roots.push(f32::sqrt(square));
        }
    }
    roots.sort_by(|a, b| a.partial_cmp(b).unwrap());
    print_roots(&roots);
}
