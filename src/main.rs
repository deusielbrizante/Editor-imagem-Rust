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

mod utils;
mod options;

use std::io::{stdin, stdout, Write};
use std::process::exit;
use image::{DynamicImage, ImageFormat};
use utils::terminal::{display_menu, clear_screen};
use utils::save_image::save_img;
use utils::extract_path::extract_path;
use crate::utils::file_name::file_name;
use crate::utils::format_image::format_image;
use crate::utils::get_image::get_image;

fn main() {
    // 1. First, you need to implement some basic command-line argument handling,
    // so you can make your program do different things.  Here's a little bit
    // to get you started doing manual parsing.
    //
    // Challenge: If you're feeling really ambitious, you could delete this code
    // and use the "clap" library instead: https://docs.rs/clap/2.32.0/clap/
    loop {
        let mut infile: String = get_image();

        let items: [&str; 8] = [
            "Blur",
            "Brighten",
            "Crop",
            "Rotate",
            "Invert",
            "Grayscale",
            "Generate",
            "Fractal"
        ];

        let option_menu_selected: u8 = display_menu("Home", &items, true);

        clear_screen();
        let (mut success, mut return_img) = (false, DynamicImage::new_rgb8(1, 1));
        match option_menu_selected {
            1 => {
                (success, return_img) = options::blur(infile.clone());
            }
            2 => println!("2"),
            3 => println!("3"),
            4 => println!("4"),
            5 => println!("5"),
            6 => println!("6"),
            7 => println!("7"),
            8 => println!("8"),
            _ => exit(0),
        }

        let format_image: (String, ImageFormat) = format_image();
        let file_name: String = file_name();

        let option_outfile_menu: [&str; 2] = [
            "Sim",
            "Outro"
        ];

        let mut outfile: String = String::new();
        let mut format_name_image: String = String::new();

        if success {
            match display_menu("Sua foto foi alterada com sucesso! Deseja salvá-la no mesmo diretório ?", &option_outfile_menu, true) {
                1 => {
                    outfile = extract_path(&infile);
                }
                2 => {
                    println!("Digite abaixo o diretório que deseje salvar a imagem!");
                    stdout().flush().unwrap();

                    stdin().read_line(&mut outfile).unwrap();

                    outfile = extract_path(&outfile);
                }
                _ => return
            }
        } else {
            println!("Erro ao converter sua imagem!");
        }

        format_name_image = String::from(format!("{}/{}.{}", outfile, file_name, format_image.0));
        save_img(&return_img, format_name_image, format_image.1);

        let yes: [&str; 1] = [
            "Sim"
        ];

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
fn print_usage_and_exit() {
    println!("USAGE (when in doubt, use a .png extension on your filenames)");
    println!("blur INFILE OUTFILE");
    println!("fractal OUTFILE");
    // **OPTION**
    // Print useful information about what subcommands and arguments you can use
    // println!("...");
    std::process::exit(-1);
}

fn brighten(infile: String, outfile: String) {
    // See blur() for an example of how to open / save an image.

    // .brighten() takes one argument, an i32.  Positive numbers brighten the
    // image. Negative numbers darken it.  It returns a new image.

    // Challenge: parse the brightness amount from the command-line and pass it
    // through to this function.
}

fn crop(infile: String, outfile: String) {
    // See blur() for an example of how to open an image.

    // .crop() takes four arguments: x: u32, y: u32, width: u32, height: u32
    // You may hard-code them, if you like.  It returns a new image.

    // Challenge: parse the four values from the command-line and pass them
    // through to this function.

    // See blur() for an example of how to save the image.
}

fn rotate(infile: String, outfile: String) {
    // See blur() for an example of how to open an image.

    // There are 3 rotate functions to choose from (all clockwise):
    //   .rotate90()
    //   .rotate180()
    //   .rotate270()
    // All three methods return a new image.  Pick one and use it!

    // Challenge: parse the rotation amount from the command-line, pass it
    // through to this function to select which method to call.

    // See blur() for an example of how to save the image.
}

fn invert(infile: String, outfile: String) {
    // See blur() for an example of how to open an image.

    // .invert() takes no arguments and converts the image in-place, so you
    // will use the same image to save out to a different file.

    // See blur() for an example of how to save the image.
}

fn grayscale(infile: String, outfile: String) {
    // See blur() for an example of how to open an image.

    // .grayscale() takes no arguments. It returns a new image.

    // See blur() for an example of how to save the image.
}

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