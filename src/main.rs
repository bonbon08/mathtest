fn calculate(term: &str) -> i64 {
    let posschars = vec!['+', '/', '*', '-'];
    let mut lastwasminus = false;
    let possnums = vec!["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
    let mut momnum = 0;
    let mut nums = Vec::new();
    let mut signs = Vec::new();
    for (_i, c) in term.chars().enumerate() {
        if posschars.contains(&c) {
            signs.push(c);
            nums.push(momnum);
            momnum = 0;
        } else if possnums.contains(&c.to_string().as_str()) {
            if lastwasminus {
                momnum = -(c.to_digit(10).unwrap() as i64);
                lastwasminus = false;
            } else {
                momnum = momnum * 10;
                momnum = momnum + c.to_digit(10).unwrap() as i64;
            }
        }
    }
    nums.push(momnum);

    let mut result = nums[0];
    for i in 1..nums.len() {
        match signs[i - 1] {
            '+' => result += nums[i],
            '-' => result -= nums[i],
            '*' => result *= nums[i],
            '/' => result /= nums[i],
            _ => {}
        }
    }

    result
}

fn main() {
    println!("{}", calculate("1+1-2"));
}
