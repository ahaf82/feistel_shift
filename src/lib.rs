use std::error::Error;
use std::io;

//#[derive(Debug)] // so it's printable
pub struct Parameters {
    message: String,
    n_rounds: u32,
    shift_function: fn(u32, u32) -> Option<u32>,
    shift_distance: u32,
    key: String,
}

pub fn interactive_feistel() -> Result<(), Box<dyn Error>> {
    let params = get_input_values();
    // println!("{:?}", params); // TODO for function member
    feistel(params)
}

pub fn feistel(params: Parameters) -> Result<(), Box<dyn Error>> {
    let mut result = params.message.clone();
    for i_round in 0..params.n_rounds {
        result = feistel_round_numeric(
            &result,
            |x| (params.shift_function)(x, params.shift_distance),
            &params.key,
        );
        println!("{:<32}{}", format!("after round {}", i_round + 1), result);
    }
    result = swap_sides_of_binary(result);
    println!("{:<32}{}", "final swap:", result);
    Ok(())
}

fn get_input_values() -> Parameters {
    let message = get_binary_string("Enter binary number with even number of bits: ");
    let n_rounds = get_integer("Enter number of rounds: ");
    let shift_function = get_shift_function("Enter shift direction\n[1] left\n[2] right): ");
    let shift_distance = get_integer("Enter shift distance: ");
    let half_length = message.len() / 2;
    let prompt = format!("Enter key (must be {half_length} bits long): ");
    let key = get_binary_string_of_length(&prompt, half_length);

    Parameters {
        message,
        n_rounds,
        shift_function,
        shift_distance,
        key,
    }
}

fn get_user_input(prompt: &str) -> String {
    let mut input = String::new();
    println!("{prompt}");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

fn get_integer(prompt: &str) -> u32 {
    loop {
        let answer = get_user_input(prompt);
        match answer.parse::<u32>() {
            Ok(i) => break i,
            Err(..) => println!("Given number is not an integer."),
        };
    }
}

fn get_binary_string(prompt: &str) -> String {
    loop {
        let answer = get_user_input(prompt);
        if answer.is_empty() {
            println!("Input must not be empty.")
        } else if !answer.chars().all(|x| '0' == x || x == '1') {
            println!("Input must contain only 0s and 1s.")
        } else if answer.len() % 2 == 1 {
            println!("Input must be of even length.")
        } else {
            break answer;
        }
    }
}

fn get_binary_string_of_length(prompt: &str, expected_length: usize) -> String {
    loop {
        let answer = get_binary_string(prompt);
        if answer.len() == expected_length {
            break answer;
        } else {
            println!("Input must have {expected_length} characters.")
        }
    }
}

fn get_shift_direction(prompt: &str) -> Direction {
    loop {
        let answer = get_user_input(prompt);
        match answer.as_str() {
            "1" => break Direction::Left,
            "2" => break Direction::Right,
            _ => println!("Wrong input."),
        }
    }
}

fn get_shift_function(prompt: &str) -> fn(u32, u32) -> Option<u32> {
    loop {
        let answer = get_user_input(prompt);
        match answer.as_str() {
            "1" => break u32::checked_shl,
            "2" => break u32::checked_shr,
            _ => println!("Wrong input."),
        }
    }
}

fn feistel_round(
    message: &str,
    shift_direction: Direction,
    shift_distance: usize,
    key: &str,
) -> String {
    let half_length = message.len() / 2;
    let (left, right) = message.split_at(half_length);
    println!("{:<32?}", left);
    println!("{:<32?}", right);
    let mut string_to_swap: String;
    match shift_direction {
        Direction::Left => string_to_swap = shift_left(right, shift_distance),
        Direction::Right => string_to_swap = shift_right(right, shift_distance),
    }
    string_to_swap = xor(&string_to_swap, key);
    let new_right = xor(left, &string_to_swap);
    println!("{:<32}{}{}", "before swap:", new_right, right);
    right.to_owned() + &new_right
}

fn feistel_round_numeric<F>(message: &str, function: F, key: &str) -> String
where
    F: Fn(u32) -> Option<u32>,
{
    let half_length = message.len() / 2;
    let (left, right) = message.split_at(half_length);
    let numeric_left = u32::from_str_radix(left, 2).unwrap();
    let numeric_right = u32::from_str_radix(right, 2).unwrap();
    let numeric_key = u32::from_str_radix(key, 2).unwrap();
    let transformed_right = function(numeric_right).unwrap() ^ numeric_key;
    let transformed_left = numeric_left ^ transformed_right;

    let transformed_right_string =
        format!("{:032b}", transformed_right)[32 - half_length..].to_string();
    let transformed_left_string =
        format!("{:032b}", transformed_left)[32 - half_length..].to_string();

    println!("{:<32}{}", "F(right) XOR key:", transformed_right_string);
    println!(
        "{:<32}{}",
        "left XOR F(right) XOR key:", transformed_left_string
    );

    println!("{:<32}{}{}", "before swap:", transformed_left_string, right);

    right.to_owned() + &transformed_left_string
}

fn get_left_side(input: &str) -> String {
    input[..(input.len() / 2)].to_owned()
}

/// Gets the second half of a string
///
/// # Example
///
/// ```
/// let test_string = String::from("01000101");
/// let right_side_value: String = feistel_shift::get_right_side(&test_string);
///
/// assert_eq!(right_side_value, "0101");
/// ```
pub fn get_right_side(input: &str) -> String {
    input[(input.len() / 2)..].to_owned()
}

/// Returns the concatenation of two strings
///
/// # Example
///
/// ```
/// let test_string1 = String::from("01011101");
/// let test_string2 = String::from("11110111");
/// let concatenated_string: String = feistel_shift::concat_strings(&test_string1, &test_string2);
///
/// assert_eq!(concatenated_string, "0101110111110111");
/// ```
pub fn concat_strings(input_first: &str, input_second: &str) -> String {
    let mut new_string = String::from("");
    new_string.push_str(input_first);
    new_string.push_str(input_second);
    new_string
}

/// Returns the result of a string, shifted to the left by a given unsigned integer value
///
/// # Example
///
/// ```
/// let test_string = String::from("010100101");
/// let left_shifted_value: String = feistel_shift::shift_left(&test_string, 2);
///
/// assert_eq!(left_shifted_value, "010010100");
/// ```
pub fn shift_left(value: &str, shift_count: usize) -> String {
    if shift_count < value.len() {
        let padding = "0".repeat(shift_count as usize);
        value[shift_count..].to_string() + &padding
    } else {
        "0".repeat(value.len())
    }
}

/// Returns the result of a string, shifted to the right by a given unsigned integer value
///
/// # Example
///
/// ```
/// let test_string = String::from("010100101");
/// let right_shifted_value: String = feistel_shift::shift_right(&test_string, 2);
///
/// assert_eq!(right_shifted_value, "000101001");
/// ```
pub fn shift_right(value: &str, shift_count: usize) -> String {
    if shift_count < value.len() {
        let padding = "0".repeat(shift_count as usize);
        padding + &value[0..(value.len() - shift_count)]
    } else {
        "0".repeat(value.len())
    }
}

/// Returns the xor value of two binary strings
///
/// # Example
///
/// ```
/// let left = String::from("01011101");
/// let right = String::from("11110111");
/// let res: String = feistel_shift::xor(&left, &right);
///
/// assert_eq!(res, "10101010");
/// ```
pub fn xor(left: &str, right: &str) -> String {
    let mut result = String::with_capacity(left.len());
    for pair in left.chars().zip(right.chars()) {
        let (a, b) = pair;
        println!("{a} {b}");
        let bit = a != b; // mimics xor for each bit
        result.push(char::from_digit(bit as u32, 2).unwrap());
        println!("{result}");
    }
    result
}

/// Swaps the first and the second half of a given string
///
/// # Example
///
/// ```
/// let test_string = String::from("01011101");
/// let swap_sides_value: String = feistel_shift::swap_sides_of_binary(test_string);
///
/// assert_eq!(swap_sides_value, "11010101");
/// ```
pub fn swap_sides_of_binary(binary_string: String) -> String {
    let half_length = binary_string.len() / 2;
    let (left, right) = binary_string.split_at(half_length);
    right.to_string() + left
}

#[derive(Debug)]
pub enum Direction {
    Left,
    Right,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_sides_test() {
        let test_string = String::from("0101110111110111");
        let left_side_of_string: String = get_left_side(&test_string);
        let right_side_of_string: String = get_right_side(&test_string);
        assert_eq!(left_side_of_string, "01011101");
        assert_eq!(right_side_of_string, "11110111");
    }

    #[test]
    fn feistel_round_test() {
        let test_string = String::from("01011101");
        let feistel_round_result: String =
            feistel_round_numeric(&test_string, |x| Some(x << 2), "1001");
        assert_eq!(feistel_round_result, "11011000");
    }
}
