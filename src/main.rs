use std::io;
fn main() {
    let stdin = io::stdin();
    loop {
        println!("Please enter your RPN Expression");
        let mut line = String::new();
        stdin.read_line( &mut line).expect("Error reading line");
        let line = line.trim();
        match eval(line.to_string()) {
            Ok(t) => {
                println!("The evaluation of your expression is: {}", t);
            }
            Err(e) => {
                println!("This error resulted: {} ", e)
            }
        }
    }
}

fn eval(expression: String) -> Result<i32, String> {
    let mut stack: Vec<i32> = Vec::new();
    for token in expression.split_whitespace() {
        match token.parse::<i32>() {
            Ok(t) => {
                stack.push(t);
            }
            Err(_) => {
                if stack.len() < 2 {
                    return Err("Stack does not contain 2 elemnts".to_string());
                }
                let num2 = stack.pop().expect("Stack may not contain atleast 2 elements");
                let num1 = stack.pop().expect("Stack may not contain atleast 2 elements");
                if "+-*/".contains(token) {
                    match token {
                        "+" => {
                            stack.push(num1+num2); 
                        }
                        "-" => {
                            stack.push(num1-num2);
                        }
                        "*" => {
                            stack.push(num1*num2);
                        }
                        
                        "/" => {
                            if num2 == 0 {
                                return Err("Divison by 0".to_string());
                            }
                            stack.push(num1/num2);
                        }
                        _ => {
                            return Err("Unrecognized operator".to_string());
                        }
                    }
                }
            }
        }
    }
    if stack.len() != 1 {
        return Err("Stack does not contain exactly 1 element".to_string());
    }
    Ok(stack.pop().unwrap())
}