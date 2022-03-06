use std::error::Error;
use std::io;
use std::process;

pub fn get_input_values() -> Result<(), Box<dyn Error>> {
	// get binary input value
	let mut binary_input_value = get_binary_string_parameter(
		String::from("Gib eine Binärzahl ein, mit einer geraden Anzahl an Zeichen ein: "),
		0,
	);
	// rounds
	let rounds =
		get_single_input_integer_parameter(String::from("Anzahl Verschlüsselungsrunden: "));
	// function
	let shift_function = get_enum_function_input(String::from(
		"Shift-Funktion (1 für Left Shift, 2 für Right Shift): ",
	));
	// shift_count
	let shift_count = get_single_input_integer_parameter(String::from("Anzahl Shift-Schritte: "));
	// key_values rounds depending on rounds
	let half_length: u32 = binary_input_value.len().try_into().unwrap();
	let key_value = get_binary_string_parameter(String::from(
		"Schlüsselwert (Binärwert in halber Zeichen-Länge wie zu codierender Eingabewert notwendig): ",
	), half_length/2);

	println!(
		"Eingabewert: {}, Rundenanzahl: {}, Shiftfunktion: {:?}, Shiftweite: {}, Schlüsselwert: {}",
		binary_input_value, rounds, shift_function, shift_count, key_value
	);

	println!("key_val:                   {}", key_value);

	// let left_side = get_left_side(&binary_input_value);
	// println!("left side of string {}", left_side);
	// let right_side = get_right_side(&binary_input_value);
	// println!("right side of string {}", right_side);
	// let concatenated_strings = concat_strings(&left_side, &right_side);
	// println!("concatenation of string {}", concatenated_strings);

	for n in 0..rounds {
		binary_input_value =
			single_encryption_round(binary_input_value, &shift_function, shift_count, &key_value);
		println!("after {} round:          {}", n + 1, binary_input_value);
	}

	binary_input_value = swap_sides_of_binary(binary_input_value);
	println!("after final swap:       {}", binary_input_value);
	Ok(())
}

/* function output single_run */
pub fn get_single_input_integer_parameter(input_message: String) -> u32 {
	let mut input = String::new();

	loop {
		input = String::from("");
		let mut is_allowed_input = true;
		println!("{}", &input_message);
		io::stdin()
			.read_line(&mut input)
			.expect("Failed to read line");

		if input.trim() == "abcd" {
			process::exit(1)
		}

		is_allowed_input = check_if_input_is_integer(&input);

		if input.trim() != "" && is_allowed_input == true {
			// println!("Bitte gib einen Wert ein");
			break;
		}

		println!("Das war keine Integerzahl, bitte nochmal!");
	}

	let output_value: u32 = input.trim().parse().unwrap();

	output_value
}

pub fn check_if_input_is_integer(input: &String) -> bool {
	let mut integer_input = true;
	for c in input.trim().chars() {
		if c != '0'
			&& c != '1' && c != '2'
			&& c != '3' && c != '4'
			&& c != '5' && c != '6'
			&& c != '7' && c != '8'
			&& c != '9'
		{
			integer_input = false
		}
	}
	integer_input
}

pub fn get_binary_string_parameter(input_message: String, needed_length: u32) -> String {
	let mut input = String::new();

	loop {
		input = String::from("");
		println!("{}", &input_message);
		io::stdin()
			.read_line(&mut input)
			.expect("Failed to read line");

		if input.trim() == "abcd" {
			process::exit(1)
		}

		if input_is_valid(&input, needed_length) {
			println!("Bitte gib einen Wert ein");
			break;
		}
	}

	let output_value: String = input.trim().to_string();

	output_value
}

pub fn input_is_valid(input: &str, needed_length: u32) -> bool {
	if input.trim() != ""
		&& check_if_input_is_binary(input)
		&& check_if_input_has_allowed_length(input, needed_length)
	{
		return true;
	}
	false
}

pub fn check_if_input_is_binary(input: &str) -> bool {
	let mut binary_input = true;
	for c in input.trim().chars() {
		if c != '0' && c != '1' {
			binary_input = false
		}
	}
	if binary_input == false {
		println!("Das war keine Binärzahl, bitte nochmal!");
	}
	binary_input
}

pub fn check_if_input_has_allowed_length(input: &str, needed_length: u32) -> bool {
	if needed_length == 0 && input.trim().len() % 2 == 1 {
		println!(
			"Bitte gib eine {}-stellige Anzahl an Zeichen ein!",
			needed_length
		);
		return false;
	}
	if needed_length != 0 && input.trim().len() != needed_length.try_into().unwrap() {
		println!(
			"Bitte gib benötigte Anzahl an Zeichen {} ein!",
			needed_length
		);
		println!("Input: {}", input);
		return false;
	}
	true
}

pub fn get_single_input_string_parameter(input_message: String) -> String {
	let mut input = String::new();
	let mut output_value: String = String::from("");

	loop {
		println!("{}", &input_message);

		io::stdin()
			.read_line(&mut input)
			.expect("Failed to read line");

		output_value = input.clone();

		if input.trim() == "abcd" {
			process::exit(1);
		}
		if input.trim() != "" {
			break;
		}
	}

	output_value
}

pub fn get_enum_function_input(input_message: String) -> ShiftValues {
	let mut input = String::new();
	let mut output_value: String = String::from("");

	loop {
		println!("{}", &input_message);

		input = String::from("");

		io::stdin()
			.read_line(&mut input)
			.expect("Failed to read line");

		if input.trim() == "abcd" {
			process::exit(1);
		}
		if input.trim() == "1" {
			break ShiftValues::LeftShift;
		}
		if input.trim() == "2" {
			break ShiftValues::RightShift;
		}
	}
}

pub fn get_left_side(input: &str) -> String {
	let mut new_string = String::from("");
	for i in 0..=(input.len() - 1) / 2 {
		let push_string =
			isize::from_str_radix(&String::from(input.chars().nth(i).unwrap()), 2).unwrap();
		new_string.push_str(&format!("{}", push_string));
	}
	new_string
}

pub fn get_right_side(input: &str) -> String {
	let mut new_string = String::from("");
	for i in ((input.len() - 1) / 2) + 1..=input.len() - 1 {
		let push_string =
			isize::from_str_radix(&String::from(input.chars().nth(i).unwrap()), 2).unwrap();
		new_string.push_str(&format!("{}", push_string));
	}
	new_string
}

pub fn concat_strings(input_first: &str, input_second: &str) -> String {
	let mut new_string = String::from("");
	new_string.push_str(input_first);
	new_string.push_str(input_second);
	new_string
}

// function left_shift(value, shift_count)
pub fn left_shift(value: &String, shift_count: u32) -> String {
	let mut real_shift = 0;
	let intval = isize::from_str_radix(&value, 2).unwrap();
	let left_shifted_value = intval << shift_count;
	let mut left_shifted_binary = format!("{:b}", left_shifted_value).trim().to_string();
	if left_shifted_binary.len() > value.len() {
		for n in 1..=left_shifted_binary.len() - value.len() {
			left_shifted_binary.remove(0);
		}
	} else {
		for n in 1..=value.len() - left_shifted_binary.len() {
			left_shifted_binary = String::from("0") + &left_shifted_binary;
		}
	}
	println!("left shifted right_side:   {}", left_shifted_binary);
	left_shifted_binary
}

// function right_shift(value, shift_count)
pub fn right_shift(value: &String, shift_count: u32) -> String {
	let intval = isize::from_str_radix(&value, 2).unwrap();
	let right_shifted_value = intval >> shift_count;
	let mut right_shifted_binary = format!("{:b}", right_shifted_value).trim().to_string();
	for n in 1..=(value.len() - right_shifted_binary.len()) {
		right_shifted_binary = String::from("0") + &right_shifted_binary;
	}
	println!("right shifted right_side:  {}", right_shifted_binary);
	right_shifted_binary
}

// function x_or(value_one, value_two)
pub fn xor_binary_values(binary_value_1: &String, binary_value_2: &String) -> String {
	let mut i = 0;
	let mut xor_value = String::from("");
	for i in 0..=binary_value_1.len() - 1 {
		let left_side =
			isize::from_str_radix(&String::from(binary_value_1.chars().nth(i).unwrap()), 2)
				.unwrap();
		let right_side =
			isize::from_str_radix(&String::from(binary_value_2.chars().nth(i).unwrap()), 2)
				.unwrap();
		// println!("left side {}", left_side);
		// println!("rigth side {}", right_side);
		xor_value.push_str(&format!("{:b}", left_side ^ right_side));
	}
	// println!("shifted value:        {}", binary_value_1);
	println!("key or left-side value:    {}", binary_value_2);
	println!("x-ored value:              {}", xor_value);
	xor_value
}

// function swap_sides(value)
pub fn swap_sides_of_binary(binary_string: String) -> String {
	let mut string_to_swap = binary_string.to_owned();
	for i in 0..(&binary_string.len() / 2) {
		let last_char = string_to_swap[string_to_swap.len() - 1..string_to_swap.len()].to_owned();
		string_to_swap = format!(
			"{}{}",
			last_char.to_owned(),
			&string_to_swap[0..string_to_swap.len() - 1]
		);
	}
	string_to_swap.to_owned()
}

// single encryption round
pub fn single_encryption_round(
	binary_input_value: String,
	shift_direction: &ShiftValues,
	shift_count: u32,
	key_value: &String,
) -> String {
	let mut left_side = get_left_side(&binary_input_value);
	println!("left side of string        {}", left_side);
	let right_side = get_right_side(&binary_input_value);
	println!("right side of string       {}", right_side);
	let mut string_to_swap = String::from("");
	match shift_direction {
		ShiftValues::LeftShift => string_to_swap = left_shift(&right_side, shift_count),
		ShiftValues::RightShift => string_to_swap = right_shift(&right_side, shift_count),
	}
	string_to_swap = xor_binary_values(&string_to_swap, &key_value);
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
