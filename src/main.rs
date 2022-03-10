use std::process;

use feistel_shift::interactive_feistel;

fn main() {
    // get_input_values();
    if let Err(e) = interactive_feistel() {
        eprintln!("Application error: {}", e);

        process::exit(1);
    };
}
