use std::io;

fn main() {
    println!("Welcome to the basic calculator!\nPlease enter a mathematical expression in one line that doesn't contain exponents, but basic mathematical operations (+ - * /).");
    println!("Multiple operations, parantheses, and negative numbers are allowed.");

    let mut input = String::new();
    
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input = input.trim().split_whitespace().collect::<String>();

    let result = evaluate_whole(input.to_string());

    println!("Result: {result}");
}

fn evaluate_whole (mut expression: String) -> String {
    
    loop{
        let (beginning_index, end_index) = paranthesis_locator(&expression);

        if beginning_index == 0 && end_index == 0 {
            let final_result = evaluate(expression);
            return final_result;
        }
        
        expression = evaluate_and_replace(&expression, beginning_index, end_index);
    }
}

fn paranthesis_locator (expression: &str) -> (usize, usize) {
    let bytes = expression.as_bytes();

    let mut beginning_index = 0;
    let mut end_index = 0;

    for (i, &item) in bytes.iter().enumerate() {
        if item == b'(' {
            beginning_index = i;
        }

        if item == b')' {
            end_index = i;
            break;
        }
    }

    (beginning_index, end_index)

}

fn evaluate_and_replace (expression: &str, beginning_index: usize, end_index: usize) -> String {
    let mut second_chunk = String::new();
    
    let first_chunk = expression[..beginning_index].to_string();
    if end_index != expression.len()-1 {
        second_chunk = expression[end_index+1..].to_string();
    }

    let sub_expression = &expression[beginning_index+1..end_index];
    let expression_result = evaluate(sub_expression.to_string());

    let mut result = String::new();
    result.push_str(&first_chunk);
    result.push_str(&expression_result);
    result.push_str(&second_chunk);

    result

}

fn evaluate (mut expression: String) -> String {


    'outer: loop {
        // If expression is already 1 number with no operands, return it.
        if expression.parse::<f32>().is_ok() {
            return expression.to_string();
        }

        let mut first_num_string = String::new();
        let mut second_num_string = String::new();
        let mut third_num_string = String::new();
        let mut second_index = 0;
        let mut third_index = 0;
        let mut fourth_index = 0;
        let mut fifth_index = 0;
        let mut first_operand = b' ';
        let mut second_operand = b' ';

        let length = expression.len();


        let bytes = expression.as_bytes();
        // It's possible to cut down on the time to evaluate if the first operand is mult or division. 
        'inner: for (i, &item) in bytes.iter().enumerate() {

            //if item == b'+' || item == b'-' || item == b'*' || item == b'/' {
            //if let b'+' | b'-' | b'*' | b'/' = item {
            if [b'+', b'-', b'*', b'/'].contains(&item) {

                // Makes sure that negative symbol of numbers is ignored
                if item == b'-' && i > 0 && [b'+', b'-', b'*', b'/'].contains(&bytes[i-1]) {
                    continue 'inner;
                }

                if first_num_string != "" {
                    if second_num_string == ""{
                        second_num_string = expression[second_index..i].to_string();
                        second_operand = item;
                        third_index = i;
                        fourth_index = i+1;
                    }
                    else {
                        third_num_string = expression[fourth_index..i].to_string();
                        fifth_index = i;
                        break 'inner;
                    }
                }
                else {
                    first_operand = item;
                    first_num_string = expression[..i].to_string();
                    second_index = i+1;
                }
            } 
        }

        if second_operand == b' ' {
            second_num_string = expression[second_index..length].to_string();
            let result = basic_evaluate(first_num_string, second_num_string, first_operand);
            expression = result;
            continue 'outer;
        }
        else {
            if third_num_string == "" {
                third_num_string = expression[fourth_index..length].to_string();
                fifth_index = length;
            }
            
            if second_operand == b'*' || second_operand == b'/' {
                let result = basic_evaluate(second_num_string, third_num_string, second_operand);
                let mut before_result = expression[..second_index].to_string();
                let after_result = expression[fifth_index..].to_string();
                before_result.push_str(&result);
                before_result.push_str(&after_result);
                expression = before_result;
                continue 'outer;
            }
            else {
                let mut result = basic_evaluate(first_num_string, second_num_string, first_operand);
                let rest_of_expression = expression[third_index..].to_string();
                result.push_str(&rest_of_expression);
                expression = result;
                continue 'outer;
            }
            
        }
    }

}


fn basic_evaluate (first_num_string: String, second_num_string: String, operand: u8) -> String {
    let first_num = first_num_string.parse::<f32>().unwrap();
    let second_num = second_num_string.parse::<f32>().unwrap();
    let mut result: f32 = 0.0;
    match operand {
        b'+' => {result = first_num + second_num;}
        b'-' => {result = first_num - second_num;}
        b'*' => {result = first_num * second_num;}
        b'/' => {result = first_num / second_num;}
        _ => {println!("Uh oh! Something went wrong in match");}
    }
    result.to_string()
}