use std::io;

fn main() {
    println!("Welcome to the basic calculator!\nPlease enter a mathematical expression in one line that doesn't contain exponents, but basic mathematical operations (+ - * /).\nMultiple operations are allowed.");

    let mut input = String::new();
    
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input = input.trim();

    //Maybe implement some detection for bad expressions

    // let (first, second) = paranthesis_locator(input);

    // println!("First: {first}, second: {second}");

    // // let first = 0;

    // let (first_chunk, second_chunk) = evaluate_and_replace(input, &first, &second);

    // println!("First chunk: {first_chunk}, second chunk: {second_chunk}");
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


}

fn evaluate (expression: &str) -> (String) {
    let bytes = expression.as_bytes();

    let first_num = 0;
    let second_num = 0;
    let mut first_num_string = "";
    let mut second_num_string = "";
    let split_index = 0;
    let mut operand = ' ';

    //Use match to find operand when enumerating. When found, then convert string to number. Maybe use a stack
    for (i, &item) in bytes.iter().enumerate() {
        match item {
            b'+' => {
                operand = '+';
                
            }
        }
        
    }

    expression.to_string()
}