pub mod hilbert;

#[cfg(test)]
mod tests;

extern crate image;

use std::env;
use image::{GenericImage, Pixel};

fn main() {
    let mut args = env::args();
    let name_arg = args.nth(1);

    if let Some(name) = name_arg {
        print_pixels(&name)
    } else {
        for i in 1..4 {
            println!("Order: {}", i);
            hilbert::parse_rule(&hilbert::RULE_A, i);
            println!("");
        }
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
