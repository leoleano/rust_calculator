use std::io;

fn main() {
    println!("Welcome to the basic calculator!\nPlease enter a mathematical expression in one line that doesn't contain exponents, but basic mathematical operations (+ - * /).\nMultiple operations are allowed.");

    let mut input = String::new();
    
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input = input.trim();

    let expression = String::from("24+52*100+4");
    let mut result = evaluate_one(&expression);
    result = evaluate_one(&result);
    result = evaluate_one(&result);
    println!("Result: {result}");
}

fn paranthesis_replace (expression: &str) -> &str {
    loop{
        let (beginning_index, end_index) = paranthesis_locator(expression);

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

fn evaluate_and_replace (expression: &str, beginning_index: &usize, end_index: &usize) -> (String) {
    let first_chunk = &expression[..*beginning_index];
    let second_chunk = &expression[*end_index+1..];

    //Add checks here for if beginning and end are 0
    let sub_expression = &expression[*beginning_index+1..*end_index];

    expression.to_string()

}

fn evaluate_one (expression: &str) -> String {

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
    for (i, &item) in bytes.iter().enumerate() {
        // f 5 + s 3 t + fo 2 fi * 5
        if item == b'+' || item == b'-' || item == b'*' || item == b'/' {
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
                    break;
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
        let result = evaluate(first_num_string, second_num_string, first_operand);
        return result;
    }
    else {
        if third_num_string == "" {
            third_num_string = expression[fourth_index..length].to_string();
            fifth_index = length;
        }
        
        if second_operand == b'*' || second_operand == b'/' {
            let result = evaluate(second_num_string, third_num_string, second_operand);
            let mut before_result = expression[..second_index].to_string();
            let after_result = expression[fifth_index..].to_string();
            before_result.push_str(&result);
            before_result.push_str(&after_result);
            return before_result;
        }
        else {
            let mut result = evaluate(first_num_string, second_num_string, first_operand);
            let rest_of_expression = expression[third_index..].to_string();
            result.push_str(&rest_of_expression);
            return result;
        }
        
    }

}


fn evaluate (first_num_string: String, second_num_string: String, operand: u8) -> String {
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