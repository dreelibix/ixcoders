use std::env;

#[derive(Debug)]
enum OperandParseError {
    EmptyInput,
    ParseError(std::num::ParseIntError)
}

fn main() {
    // Initialize a variable with the greeting message.
    let greeting_message: String = get_greeting_message();
    // Print the message to console.
    println!("{}", greeting_message);

    // Get all command line arguments and skip executable path.
    let args: Vec<String> = env::args().skip(1).collect();
    // Print all the command line arguments.
    args.iter().for_each(|i| println!("{}", i));
    // Execute the command and print the result.
    match handle_command(&args[0], &args[1..]) {
        Ok(result) => println!("The result is {}.", result.unwrap()),
        Err(e) => println!("{}", e),
    };
}

/// Function to get the greeting message.
///
/// This is a zero-argument function that provides a default greeting
/// for the application. The returned string is a compile-time constant.
///
/// # Examples
///
/// ```
/// let msg = get_greeting_message();
/// assert_eq!(msg, "Hi bro. Welcome to the IXCodeRS Project - a dev sandbox for Rust.");
/// ```
///
/// # Returns
/// - `String`: Immutable string containing the welcome message
fn get_greeting_message() -> String {
    String::from("Hi bro. Welcome to the IXCodeRS Project - a dev sandbox for Rust.")
}

fn handle_command(command: &str, options: &[String]) -> Result<Option<i32>, String> {
    match command {
        "calc" => handle_calc(&options[0], &options[1..]), 
        _ => Err("Unsupported command.".into()),
    }
}

fn handle_calc(operator: &str, operands: &[String]) -> Result<Option<i32>, String> {
    match convert_operands(operands) {
        Ok(int_operands) => {
            match operator {
                "add" => Ok(add(int_operands)),
                "sub" => Ok(sub(int_operands)),
                _ => Err("Unsupported operator.".into()),
            }
        },
        Err(OperandParseError::EmptyInput) => Err("Input operands must not be empty.".into()),
        Err(OperandParseError::ParseError(_)) => Err("Failed to parse operands as number.".into()),
    }
}

fn convert_operands(operands: &[String]) -> Result<Vec<i32>, OperandParseError> {
    if operands.is_empty() {
        return Err(OperandParseError::EmptyInput);
    }
    operands.iter().map(|o| o.parse().map_err(OperandParseError::ParseError)).collect()
}

fn add(operands: Vec<i32>) -> Option<i32> {
    Some(operands.iter().sum())
}
fn sub(operands: Vec<i32>) -> Option<i32> {
    let mut operands = operands.iter();
    let first_operand = *operands.next()?;
    Some(operands.fold(first_operand, |acc, &x| acc - x))
}