use regex::Regex;

fn operation(regex: Regex, operator: char, mut expression: String) -> String {
    loop {
        let caps = regex.captures(&expression);

        if caps.is_none() {
            break;
        }

        let caps = caps.unwrap();
        let cap_expression = caps.get(0).unwrap().as_str();
        let left_num: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let right_num: i32 = caps.get(2).unwrap().as_str().parse().unwrap();
        let result: i32 = match operator {
            '/' => left_num / right_num,
            '*' => left_num * right_num,
            '+' => left_num + right_num,
            '-' => left_num - right_num,
            _ => 0,
        };
        expression = expression.replace(cap_expression, &result.to_string());
    }

    expression
}

fn main() {
    println!("Hola mundo!");

    // Regex
    let regex_div = Regex::new(r"(\d+)\s?/\s?(\d+)").unwrap();
    let regex_mul = Regex::new(r"(\d+)\s?\*\s?(\d+)").unwrap();
    let regex_sum = Regex::new(r"(\d+)\s?\+\s?(\d+)").unwrap();
    let regex_subs = Regex::new(r"(\d+)\s?\-\s?(\d+)").unwrap();

    // User data
    println!("Por favor introduce tu operación:");
    let mut user_expression = String::new();
    std::io::stdin().read_line(&mut user_expression).unwrap();

    // Operations
    user_expression = operation(regex_div, '/', user_expression);
    user_expression = operation(regex_mul, '*', user_expression);
    user_expression = operation(regex_sum, '+', user_expression);
    user_expression = operation(regex_subs, '-', user_expression);

    // Results
    println!("Resultado: {}", user_expression);
}
