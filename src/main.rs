use std::io::Write;
extern crate queues;
use queues::*;

#[derive(Clone, Debug)]
#[derive(PartialEq)]
enum Tokens {
    Plus,
    Minus,
    Divide,
    Mutipy,
    LParenthesis,
    RParenthesis,
    Digit(f32),
}

fn main() {
    println!("Calculator");
    print!("enter the math: ");
    std::io::stdout().flush().unwrap();
    let mut problem = String::new();
    std::io::stdin().read_line(&mut problem).unwrap();
    problem = problem.replace(' ', "");

    let mut postfix_problem = postfix(problem.trim().to_owned());
    println!("----------------");
    println!(
        "{} = {}",
        problem.trim(),
        math(&mut postfix_problem)
    )
}

fn math(problem: &mut Queue<Tokens>) -> f32 {
    let mut stack: Vec<f32> = Vec::new();
    loop {
        let x = match problem.remove() {
            Ok(x) => x,
            Err(_) => break,
        };
        match x {
            Tokens::Plus => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(a + b)
            }
            Tokens::Minus => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(a - b)
            }
            Tokens::Mutipy => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(a * b)
            }
            Tokens::Divide => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(a / b)
            }
            Tokens::Digit(x) => stack.push(x),
            _ => println!("{:?}", problem )
        }
    }
    stack[0]
}

fn postfix(problem: String) -> Queue<Tokens> {
    let mut output_queue: Queue<Tokens> = Queue::new();
    let mut operator_stack: Vec<Tokens> = Vec::new();
    let mut digit: String = String::new();
    for (idx, i) in problem.chars().enumerate() {
        if i.is_digit(10) || i == '.' {
            digit.push(i);

            if idx == problem.len() -1 {
                output_queue
                .add(Tokens::Digit(digit.parse::<f32>().unwrap()))
                .unwrap();
                for i in operator_stack.iter() {
                    if operator_stack.last() != Some(&Tokens::LParenthesis){
                        output_queue.add(i.to_owned()).unwrap();
                    }
                    
                }
            }
        } else if i == '('{
            operator_stack.push(Tokens::LParenthesis);

        }else if i == ')'{
            let mut count = 0;
            for i in operator_stack.iter(){
                match i {
                    Tokens::LParenthesis => {break},
                    _ => {count += 1}
                }
            }
            // if operator_stack.len() == count {panic!("mismatched parentheses")}
            println!("{:?}", operator_stack);
            for _ in 0..count{
                output_queue.add(operator_stack.pop().unwrap()).unwrap();
            }
            operator_stack.pop().unwrap();

        } else {
            let tokenized_i: Tokens;
            let token_precedence: i32;
            match i {
                '+' => {
                    tokenized_i = Tokens::Plus;
                    token_precedence = 1;
                }
                '-' => {
                    tokenized_i = Tokens::Minus;
                    token_precedence = 1;
                }
                '*' => {
                    tokenized_i = Tokens::Mutipy;
                    token_precedence = 2;
                }
                '/' => {
                    tokenized_i = Tokens::Divide;
                    token_precedence = 2;
                }
                _ => panic!(),
            }
            output_queue
                .add(Tokens::Digit(digit.parse::<f32>().unwrap()))
                .unwrap();
            digit.clear();
            if !operator_stack.is_empty() {
                let on_top_os: i32;
                match operator_stack[0] {
                    Tokens::Plus => on_top_os = 1,
                    Tokens::Minus => on_top_os = 1,
                    Tokens::Mutipy => on_top_os = 2,
                    Tokens::Divide => on_top_os = 2,
                    _ => panic!(),
                }

                if on_top_os >= token_precedence {
                    output_queue.add(operator_stack.pop().unwrap()).unwrap();
                }
            }
            operator_stack.push(tokenized_i)
        }
    }
    output_queue
}
