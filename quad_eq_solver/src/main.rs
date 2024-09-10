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
    if descriptor < 0.0 {
        println!("{}", "No roots".red());
    } else if descriptor == 0.0 {
        println!("One root:\n{}", (-b / 2.0 / a).to_string().green());
    } else {
        let desc_sqrt = f32::sqrt(descriptor);
        println!(
            "Two roots:\n{}, {}",
            ((-b - desc_sqrt) / 2.0 / a).to_string().green(),
            ((-b + desc_sqrt) / 2.0 / a).to_string().green(),
        );
    }
}
