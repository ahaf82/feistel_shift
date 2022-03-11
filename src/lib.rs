use std::error::Error;
use std::io;

#[derive(Debug)] // so it's printable
struct Parameters {
    message: String,
    n_rounds: u32,
    shift_direction: ShiftValues,
    shift_distance: u32,
    key: String,
}

pub fn interactive_feistel() -> Result<(), Box<dyn Error>> {
    let params = get_input_values();
    println!("{:?}", params);
    let mut result = params.message.clone();
    for i_round in 0..params.n_rounds {
        result = feistel_round(
            &result,
            &params.shift_direction,
            params.shift_distance,
            &params.key,
        );
        println!("after round {}:          {}", i_round + 1, result);
    }
    result = swap_sides_of_binary(result);
    println!("after final swap:       {}", result);
    Ok(())
}

fn get_input_values() -> Parameters {
    let message = get_binary_string("Enter binary number with even number of bits: ");
    let n_rounds = get_integer("Enter number of rounds: ");
    let shift_direction = get_enum_function_input("Enter shift direction (1: <<, 2: >>): ");
    let shift_distance = get_integer("Enter shift distance: ");
    let half_length = message.len() / 2;
    let prompt = format!("Enter key (must be {half_length} bits long): ");
    let key = get_binary_string_of_length(&prompt, half_length);

    Parameters {
        message,
        n_rounds,
        shift_direction,
        shift_distance,
        key,
    }
}

/* function output single_run */
pub fn get_integer(prompt: &str) -> u32 {
    let mut input = String::new();
    loop {
        println!("{prompt}");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let trimmed = input.trim();
        match trimmed.parse::<u32>() {
            Ok(i) => break i,
            Err(..) => println!("Given number is not an integer."),
        };
    }
}

fn get_binary_string(prompt: &str) -> String {
    let mut input = String::new();
    loop {
        println!("{prompt}");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let trimmed = input.trim();
        if !trimmed.is_empty() && trimmed.chars().all(|x| '0' == x || x == '1') {
            break trimmed.to_string();
        } else {
            println!("Input must contain only 0s and 1s.")
        }
    }
}

fn get_binary_string_of_length(prompt: &str, expected_length: usize) -> String {
    loop {
        let input = get_binary_string(prompt);
        if input.len() == expected_length {
            break input;
        } else {
            println!("Input must have {expected_length} characters.")
        }
    }
}

fn get_enum_function_input(prompt: &str) -> ShiftValues {
    let mut input = String::new();

    loop {
        println!("{prompt}");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        if input.trim() == "1" {
            break ShiftValues::LeftShift;
        }
        if input.trim() == "2" {
            break ShiftValues::RightShift;
        }
    }
}

fn get_left_side(input: &str) -> String {
    let mut new_string = String::from("");
    for i in 0..=(input.len() - 1) / 2 {
        let push_string =
            isize::from_str_radix(&String::from(input.chars().nth(i).unwrap()), 2).unwrap();
        new_string.push_str(&format!("{}", push_string));
    }
    new_string
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
    let mut new_string = String::from("");
    for i in ((input.len() - 1) / 2) + 1..=input.len() - 1 {
        let push_string =
            isize::from_str_radix(&String::from(input.chars().nth(i).unwrap()), 2).unwrap();
        new_string.push_str(&format!("{}", push_string));
    }
    new_string
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
/// let left_shifted_value: String = feistel_shift::left_shift(&test_string, 2);
///
/// assert_eq!(left_shifted_value, "010010100");
/// ```
pub fn left_shift(value: &String, shift_count: u32) -> String {
    let intval = isize::from_str_radix(&value, 2).unwrap();
    let left_shifted_value = intval << shift_count;
    let mut left_shifted_binary = format!("{:b}", left_shifted_value).trim().to_string();
    if left_shifted_binary.len() > value.len() {
        for _ in 1..=left_shifted_binary.len() - value.len() {
            left_shifted_binary.remove(0);
        }
    } else {
        for _ in 1..=value.len() - left_shifted_binary.len() {
            left_shifted_binary = String::from("0") + &left_shifted_binary;
        }
    }
    println!("left shifted right_side:   {}", left_shifted_binary);
    left_shifted_binary
}

/// Returns the result of a string, shifted to the right by a given unsigned integer value
///
/// # Example
///
/// ```
/// let test_string = String::from("010100101");
/// let right_shifted_value: String = feistel_shift::right_shift(&test_string, 2);
///
/// assert_eq!(right_shifted_value, "000101001");
/// ```
pub fn right_shift(value: &String, shift_count: u32) -> String {
    let intval = isize::from_str_radix(&value, 2).unwrap();
    let right_shifted_value = intval >> shift_count;
    let mut right_shifted_binary = format!("{:b}", right_shifted_value).trim().to_string();
    for _ in 1..=(value.len() - right_shifted_binary.len()) {
        right_shifted_binary = String::from("0") + &right_shifted_binary;
    }
    println!("right shifted right_side:  {}", right_shifted_binary);
    right_shifted_binary
}

/// Returns the xor value of two binary strings
///
/// # Example
///
/// ```
/// let test_string1 = String::from("01011101");
/// let test_string2 = String::from("11110111");
/// let xored_value: String = feistel_shift::xor_binary_values(&test_string1, &test_string2);
///
/// assert_eq!(xored_value, "10101010");
/// ```
pub fn xor_binary_values(binary_value_1: &str, binary_value_2: &str) -> String {
    let mut xor_value = String::new();
    for i in 0..=binary_value_1.len() - 1 {
        let left_side =
            isize::from_str_radix(&String::from(binary_value_1.chars().nth(i).unwrap()), 2)
                .unwrap();
        let right_side =
            isize::from_str_radix(&String::from(binary_value_2.chars().nth(i).unwrap()), 2)
                .unwrap();
        xor_value.push_str(&format!("{:b}", left_side ^ right_side));
    }
    println!("key or left-side value:    {}", binary_value_2);
    println!("x-ored value:              {}", xor_value);
    xor_value
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
    let mut string_to_swap = binary_string.to_owned();
    for _ in 0..(&binary_string.len() / 2) {
        let last_char = string_to_swap[string_to_swap.len() - 1..string_to_swap.len()].to_owned();
        string_to_swap = format!(
            "{}{}",
            last_char.to_owned(),
            &string_to_swap[0..string_to_swap.len() - 1]
        );
    }
    string_to_swap.to_owned()
}

/// Returns a string after the process of one round of the feistel algorithm, where the input is a string,the
/// function (left or a right shift and the shift distance)
///
/// # Example
///
/// ```
/// let test_string = String::from("01011101");
/// let feistel_round_result: String = feistel_shift::feistel_round(&test_string, &feistel_shift::ShiftValues::LeftShift, 2, "1001");
///
/// assert_eq!(feistel_round_result, "11011000");
/// ```
pub fn feistel_round(
    message: &str,
    shift_direction: &ShiftValues,
    shift_distance: u32,
    key: &str,
) -> String {
    let mut left_side = get_left_side(message);
    println!("left side of string        {}", left_side);
    let right_side = get_right_side(message);
    println!("right side of string       {}", right_side);
    let mut string_to_swap: String;
    match shift_direction {
        ShiftValues::LeftShift => string_to_swap = left_shift(&right_side, shift_distance),
        ShiftValues::RightShift => string_to_swap = right_shift(&right_side, shift_distance),
    }
    string_to_swap = xor_binary_values(&string_to_swap, key);
    left_side = xor_binary_values(&left_side, &string_to_swap);
    let concatenated_strings = concat_strings(&left_side, &right_side);
    println!("concatenation of string {}", concatenated_strings);
    swap_sides_of_binary(concatenated_strings)
}

#[derive(Debug)]
pub enum ShiftValues {
    LeftShift,
    RightShift,
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
}
