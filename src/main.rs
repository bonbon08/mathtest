fn calculate(term: &str) -> i64 {
    let mut num_stack = Vec::new(); 
    let mut op_stack = Vec::new();  
    let mut num = 0;
    let mut last_was_digit = false;

    let term = term.chars().collect::<Vec<_>>(); 
    let mut i = 0;

    while i < term.len() {
        let c = term[i];

        if c.is_digit(10) {
            num = num * 10 + c.to_digit(10).unwrap() as i64;
            last_was_digit = true;
        } else if c == '(' {
            op_stack.push(c);
        } else if c == ')' {
            while *op_stack.last().unwrap() != '(' {
                let op = op_stack.pop().unwrap();
                let b = num_stack.pop().unwrap();
                num = apply_operator(op, b, num);
            }
            op_stack.pop();
        } else if c == '+' || c == '-' || c == '*' || c == '/' {
            if last_was_digit {
                num_stack.push(num);
                num = 0;
            }

            while !op_stack.is_empty() && precedence(*op_stack.last().unwrap()) >= precedence(c) {
                let op = op_stack.pop().unwrap();
                let b = num_stack.pop().unwrap();
                num = apply_operator(op, b, num);
            }

            op_stack.push(c);
            last_was_digit = false;
        }

        i += 1;
    }

    if last_was_digit {
        num_stack.push(num);
    }

    while !op_stack.is_empty() {
        let op = op_stack.pop().unwrap();
        let b = num_stack.pop().unwrap();
        num = apply_operator(op, b, num);
    }

    num
}

fn precedence(op: char) -> i32 {
    match op {
        '+' | '-' => 1,
        '*' | '/' => 2,
        _ => 0,
    }
}

fn apply_operator(op: char, a: i64, b: i64) -> i64 {
    match op {
        '+' => a + b,
        '-' => a - b,
        '*' => a * b,
        '/' => a / b,
        _ => 0,
    }
}

fn main() {
    println!("{}", calculate("1+1-2"));
    println!("{}", calculate("2+3*2"));
    println!("{}", calculate("(2+3)*2")); 
    println!("{}", calculate("2*(3+5)")); 
    println!("{}", calculate("10/(2+3)"));
}
