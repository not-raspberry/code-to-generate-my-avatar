pub mod hilbert;

#[cfg(test)]
mod tests;

extern crate image;

use std::env;

fn main() {
    let mut args = env::args();
    let exec_name = args.next().unwrap();
    let name_arg = args.next();

    if let Some(name) = name_arg {
        hilbert::hilbert_pixels(name);
    } else {
        println!("Invocation: $ {} <filename>", exec_name);
    };
}
