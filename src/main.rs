// FINAL PROJECT
//
// Create an image processing application.  Exactly what it does and how it does
// it is up to you, though I've stubbed a good amount of suggestions for you.
// Look for comments labeled **OPTION** below.
//
// Two image files are included in the project root for your convenience: dyson.png and pens.png
// Feel free to use them or provide (or generate) your own images.
//
// Don't forget to have fun and play around with the code!
//
// Documentation for the image library is here: https://docs.rs/image/0.21.0/image/
//
// NOTE 1: Image processing is very CPU-intensive.  Your program will run *noticeably* faster if you
// run it with the `--release` flag.
//
//     cargo run --release [ARG1 [ARG2]]
//
// For example:
//
//     cargo run --release blur image.png blurred.png
//
// NOTE 2: This is how you parse a number from a string (or crash with a
// message). It works with any integer or float type.
//
//     let positive_number: u32 = some_string.parse().expect("Failed to parse a number");

mod enums;
mod structs;
mod utils;

use image::{DynamicImage, ImageFormat};
use utils::select_option::select_generate_option;
use std::process::exit;

use crate::{
    enums::type_execution::TypeExecution,
    utils::{
        image::{format_image, get_image, get_image_name, save_image},
        path::get_path_image,
        select_option::select_simple_option,
        terminal::{clear_screen, display_menu},
    },
};

fn main() {
    // 1. First, you need to implement some basic command-line argument handling,
    // so you can make your program do different things.  Here's a little bit
    // to get you started doing manual parsing.
    //
    // Challenge: If you're feeling really ambitious, you could delete this code
    // and use the "clap" library instead: https://docs.rs/clap/2.32.0/clap/
    loop {
        let mut infile: String = String::new();

        let items: [&str; 7] = [
            "Blur",
            "Brighten",
            "Crop",
            "Rotate",
            "Invert",
            "Grayscale",
            "Generate Image",
        ];

        let option_menu_selected: u8 = display_menu("Home", &items, true);

        if option_menu_selected != 7 {
            infile = get_image();
        }

        clear_screen();

        let (success, return_img): (bool, DynamicImage) = match option_menu_selected {
            1 => select_simple_option(infile.clone(), TypeExecution::Blur),
            2 => select_simple_option(infile.clone(), TypeExecution::Brighten),
            3 => select_simple_option(infile.clone(), TypeExecution::Crop),
            4 => select_simple_option(infile.clone(), TypeExecution::Rotate),
            5 => select_simple_option(infile.clone(), TypeExecution::Invert),
            6 => select_simple_option(infile.clone(), TypeExecution::Grayscale),
            7 => select_generate_option(),
            _ => exit(0),
        };

        let format_image: (String, ImageFormat) = format_image();
        let file_name: String = get_image_name();

        if success {
            let outfile: String = get_path_image(&mut infile, option_menu_selected);

            let format_name_image: String =
                String::from(format!("{}/{}.{}", outfile, file_name, format_image.0));
            save_image(&return_img, format_name_image, format_image.1);
        } else {
            println!("Erro ao converter sua imagem!");
        }

        let yes: [&str; 1] = ["Sim"];

        match display_menu("Deseja voltar para o menu inicial ?", &yes, true) {
            1 => {
                continue;
            }
            _ => exit(0),
        }
    }

    // let mut args: Vec<String> = std::env::args().skip(1).collect();
    // if args.is_empty() {
    //     print_usage_and_exit();
    // }
    // let subcommand = args.remove(0);
    // match subcommand.as_str() {
    //     // EXAMPLE FOR CONVERSION OPERATIONS
    //     "blur" => {
    //         if args.len() != 2 {
    //             print_usage_and_exit();
    //         }
    //         let infile = args.remove(0);
    //         let outfile = args.remove(0);
    //         // **OPTION**
    //         // Improve the blur implementation -- see the blur() function below
    //         let blur_value: f32 = 2.0;
    //         blur(infile, outfile, blur_value);
    //     }

    // **OPTION**
    // Brighten -- see the brighten() function below

    // **OPTION**
    // Crop -- see the crop() function below

    // **OPTION**
    // Rotate -- see the rotate() function below

    // **OPTION**
    // Invert -- see the invert() function below

    // **OPTION**
    // Grayscale -- see the grayscale() function below

    // A VERY DIFFERENT EXAMPLE...a really fun one. :-)
    // "fractal" => {
    //     if args.len() != 1 {
    //         print_usage_and_exit();
    //     }
    //     let outfile = args.remove(0);
    //     fractal(outfile);
    // }

    // **OPTION**
    // Generate -- see the generate() function below -- this should be sort of like "fractal()"!

    // For everything else...
    //     _ => {
    //         print_usage_and_exit();
    //     }
    // }
}
/*

fn generate(outfile: String) {
    // Create an ImageBuffer -- see fractal() for an example
    // Iterate over the coordinates and pixels of the image -- see fractal() for an example

    // Set the image to some solid color. -- see fractal() for an example

    // Challenge: parse some color data from the command-line, pass it through
    // to this function to use for the solid color.

    // Challenge 2: Generate something more interesting!

    // See blur() for an example of how to save the image
}

// This code was adapted from https://github.com/PistonDevelopers/image
fn fractal(outfile: String) {
    let width = 800;
    let height = 800;

    let mut imgbuf = image::ImageBuffer::new(width, height);

    let scale_x = 3.0 / width as f32;
    let scale_y = 3.0 / height as f32;

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        // Use red and blue to be a pretty gradient background
        let red = (0.3 * x as f32) as u8;
        let blue = (0.3 * y as f32) as u8;

        // Use green as the fractal foreground (here is the fractal math part)
        let cx = y as f32 * scale_x - 1.5;
        let cy = x as f32 * scale_y - 1.5;

        let c = num_complex::Complex::new(-0.4, 0.6);
        let mut z = num_complex::Complex::new(cx, cy);

        let mut green = 0;
        while green < 255 && z.norm() <= 2.0 {
            z = z * z + c;
            green += 1;
        }

        // Actually set the pixel. red, green, and blue are u8 values!
        *pixel = image::Rgb([red, green, blue]);
    }

    imgbuf.save(outfile).unwrap();
}

// **SUPER CHALLENGE FOR LATER** - Let's face it, you don't have time for this during class.
//
// Make all of the subcommands stackable!
//
// For example, if you run:
//
//   cargo run infile.png outfile.png blur 2.5 invert rotate 180 brighten 10
//
// ...then your program would:
// - read infile.png
// - apply a blur of 2.5
// - invert the colors
// - rotate the image 180 degrees clockwise
// - brighten the image by 10
// - and write the result to outfile.png
//
// Good luck!
*/
