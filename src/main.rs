use std::process;

use feistel_shift::get_input_values;

fn main() {
    // get_input_values();
    if let Err(e) = get_input_values() {
        eprintln!("Application error: {}", e);

        process::exit(1);
    };
}
