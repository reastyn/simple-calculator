use std::io;
use std::iter::Peekable;
use std::process;
use std::slice::Iter;

#[derive(Debug)]
enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}
#[derive(Debug)]
enum Operation {
    Binary(Operator, Box<Operation>, Box<Operation>),
    Number(f64),
}

#[derive(Debug)]
enum Lexer {
    Number(f64),
    OperationChar(char),
    End,
}

fn lex(user_input: &str) -> Result<Vec<Lexer>, String> {
    let mut lexer_vec: Vec<Lexer> = Vec::new();
    let mut number = String::new();
    let mut last_space = false;

    for char in user_input.chars() {
        match char {
            '0'..='9' | '.' => {
                if last_space && number.len() > 0 {
                    return Err("Unexpected space".to_string());
                }
                number.push(char)
            }
            ' ' => {}
            '+' | '-' | '*' | '/' => {
                let cast_num = number.parse::<f64>().unwrap();
                lexer_vec.push(Lexer::Number(cast_num));
                number.clear();
                lexer_vec.push(Lexer::OperationChar(char));
            }
            '\n' => {
                let number = number.parse::<f64>().unwrap();
                lexer_vec.push(Lexer::Number(number));
                lexer_vec.push(Lexer::End);
            }
            _ => return Err(format!("Unsupported character '{}'", char)),
        }
        last_space = char == ' ';
    }

    Ok(lexer_vec)
}

fn parse_precendence_2(lexer_vec: &mut Peekable<Iter<Lexer>>) -> Result<Operation, String> {
    let next = lexer_vec.next();

    match next {
        Some(Lexer::Number(num)) => Ok(Operation::Number(*num)),
        None => Err("Unexpected input".to_string()),
        _ => Err("Unexpected operation".to_string()),
    }
}

fn parse_precendence_1(lexer_vec: &mut Peekable<Iter<Lexer>>) -> Result<Operation, String> {
    let mut parsed = parse_precendence_2(lexer_vec)?;
    loop {
        match lexer_vec.peek().unwrap() {
            Lexer::OperationChar(op) if *op == '*' || *op == '/' => {
                lexer_vec.next();
                let right = parse_precendence_2(lexer_vec)?;
                match op {
                    '*' => {
                        parsed =
                            Operation::Binary(Operator::Multiply, Box::new(parsed), Box::new(right))
                    }
                    '/' => {
                        parsed =
                            Operation::Binary(Operator::Divide, Box::new(parsed), Box::new(right))
                    }
                    _ => return Err("Unexpected operation".to_string()),
                }
            }
            _ => break,
        }
    }

    Ok(parsed)
}

fn parse_precendence_0(lexer_vec: &mut Peekable<Iter<Lexer>>) -> Result<Operation, String> {
    let mut parsed = parse_precendence_1(lexer_vec)?;
    loop {
        match lexer_vec.peek().unwrap() {
            Lexer::OperationChar(op) if *op == '+' || *op == '-' => {
                lexer_vec.next();
                let right = parse_precendence_1(lexer_vec)?;
                match op {
                    '+' => {
                        parsed = Operation::Binary(Operator::Add, Box::new(parsed), Box::new(right))
                    }
                    '-' => {
                        parsed =
                            Operation::Binary(Operator::Subtract, Box::new(parsed), Box::new(right))
                    }
                    _ => return Err("Unexpected operation".to_string()),
                }
            }
            _ => break,
        }
    }

    Ok(parsed)
}

fn parse(lexer_vec: &mut Peekable<Iter<Lexer>>) -> Result<Operation, String> {
    let parsed = parse_precendence_0(lexer_vec);
    match lexer_vec.peek() {
        Some(Lexer::End) => parsed,
        _ => return Err("Unexpected input".to_string()),
    }
}

fn execute(operation: Operation) -> Result<f64, String> {
    match operation {
        Operation::Number(num) => Ok(num),
        Operation::Binary(op, left, right) => {
            let left = execute(*left)?;
            let right = execute(*right)?;
            match op {
                Operator::Add => Ok(left + right),
                Operator::Subtract => Ok(left - right),
                Operator::Multiply => Ok(left * right),
                Operator::Divide => {
                    if right == 0.0 {
                        return Err("Division by zero".to_string());
                    }
                    Ok(left / right)
                }
            }
        }
    }
}
// Implementation of simple grammar
// A -> E END
// E -> T + T | T - T
// T -> F * F | F / F
// F -> NUMBER
fn main() -> io::Result<()> {
    let mut user_input = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut user_input)?;
    let lexer_vec = lex(&user_input).unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        process::exit(1)
    });
    let mut lexer_iter = lexer_vec.iter().peekable();
    println!("Lexed vector: {:?}", lexer_vec);

    let parsed = parse(&mut lexer_iter).unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        process::exit(1)
    });
    println!("Parsed Operation struct: {:?}", parsed);

    let res = execute(parsed).unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        process::exit(1)
    });
    println!("Result: {}", res);

    Ok(())
}
