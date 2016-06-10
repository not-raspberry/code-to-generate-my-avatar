pub mod hilbert;
pub mod image_gen;

#[cfg(test)]
mod tests;

extern crate image;

use std::env;
use std::fs::OpenOptions;
use std::process::exit;
use image::{ImageFormat};

fn main() {
    let mut args = env::args();
    let exec_name = args.next().unwrap();

    if let Some(file_name) = args.next() {
        let dest = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(file_name);
        match dest {
            Ok(mut file) => {
                let image = image_gen::hilbert_pixels();
                image.save(&mut file, ImageFormat::PNG).unwrap();
            }
            Err(err) => {
                println!("Failed to open a file: {}", err);
                exit(1);
            }
        }
    } else {
        println!("Invocation: $ {} <destination_filename>", exec_name);
    };
}
