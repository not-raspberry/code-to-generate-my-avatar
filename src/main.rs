pub mod hilbert;

#[cfg(test)]
mod tests;

extern crate image;

use std::env;
use image::{GenericImage, Pixel};

fn main() {
    let mut args = env::args();
    let exec_name = args.next().unwrap();
    let name_arg = args.next();

    if let Some(name) = name_arg {
        print_pixels(&name)
    } else {
        println!("Invocation: $ {} <filename>", exec_name);
    };
}

fn print_pixels(name: &str) {
    let img = image::open(name).unwrap();
    let (w, h) = img.dimensions();
    println!("Image dimensions: {} x {}", w, h);

    let mut line = 0;
    for (_, y, pixel) in img.pixels() {
        if line != y {
            println!("");
            line += 1;
        }
        let rgb = pixel.to_rgb();
        let sum: u32 = rgb.data.iter().fold(0u32, |a, &b| a + b as u32);
        let avg = sum / 3;  // Desaturate :)
        print!("{: >4} ", avg);
    }
    println!("");
}
