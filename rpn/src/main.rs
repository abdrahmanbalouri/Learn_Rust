use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Error");
        return;
    }

    let c = &args[1];
    let mut stack = Vec::new();

    for i in c.split_whitespace() {
        if let Ok(a) = i.parse::<i32>() {
            stack.push(a);
        } else {
            if stack.len() < 2 {
                println!("Error");
                return;
            }
            let b = stack.pop().unwrap();
            let a = stack.pop().unwrap();

            let h = match i {
                "+" => a + b,
                "-" => a - b,
                "*" => a * b,
                "/" => {
                    if b == 0 {
                        println!("Error");
                        return;
                    }
                    a / b
                }
                "%" => {
                    if b == 0 {
                        println!("Error");
                        return;
                    }
                    a % b
                }
                _ => {
                    println!("Error");
                    return;
                }
            };
            stack.push(h);
        }
    }
    if stack.len() == 1 {
        println!("{}", stack[0]);
        return;
    }
    println!("Error");
    return;
}
