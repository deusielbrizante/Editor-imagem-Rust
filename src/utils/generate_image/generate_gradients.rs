use crate::structs::generated::GeneratedGradientImage;
use image::RgbImage;
use rayon::iter::{ParallelBridge, ParallelIterator};

pub fn gradient_horizontal(image_values: GeneratedGradientImage) -> RgbImage {
    let mut img_buffer = RgbImage::new(image_values.width, image_values.height);

    img_buffer
        .enumerate_pixels_mut()
        .par_bridge()
        .for_each(|(x, _y, pixel)| {
            let t: f32 = x as f32 / (image_values.width - 1) as f32;
            let red: u8 = ((1.0 - t) * image_values.start_rgb.red as f32
                + t * image_values.end_rgb.red as f32) as u8;
            let green: u8 = ((1.0 - t) * image_values.start_rgb.green as f32
                + t * image_values.end_rgb.green as f32) as u8;
            let blue: u8 = ((1.0 - t) * image_values.start_rgb.blue as f32
                + t * image_values.end_rgb.blue as f32) as u8;
            *pixel = image::Rgb([red, green, blue]);
        });

    img_buffer
}

pub fn gradient_vertical(image_values: GeneratedGradientImage) -> RgbImage {
    let mut img_buffer = RgbImage::new(image_values.width, image_values.height);

    img_buffer
        .enumerate_pixels_mut()
        .par_bridge()
        .for_each(|(_x, y, pixel)| {
            let t: f32 = y as f32 / (image_values.height - 1) as f32;
            let red: u8 = ((1.0 - t) * image_values.start_rgb.red as f32
                + t * image_values.end_rgb.red as f32) as u8;
            let green: u8 = ((1.0 - t) * image_values.start_rgb.green as f32
                + t * image_values.end_rgb.green as f32) as u8;
            let blue: u8 = ((1.0 - t) * image_values.start_rgb.blue as f32
                + t * image_values.end_rgb.blue as f32) as u8;
            *pixel = image::Rgb([red, green, blue]);
        });

    img_buffer
}

pub fn gradient_diagonal(image_values: GeneratedGradientImage) -> RgbImage {
    let mut img_buffer = RgbImage::new(image_values.width, image_values.height);

    img_buffer
        .enumerate_pixels_mut()
        .par_bridge()
        .for_each(|(x, y, pixel)| {
            let t: f32 = (x + y) as f32 / (image_values.width + image_values.height) as f32;
            let red: u8 = ((1.0 - t) * image_values.start_rgb.red as f32
                + t * image_values.end_rgb.red as f32) as u8;
            let green: u8 = ((1.0 - t) * image_values.start_rgb.green as f32
                + t * image_values.end_rgb.green as f32) as u8;
            let blue: u8 = ((1.0 - t) * image_values.start_rgb.blue as f32
                + t * image_values.end_rgb.blue as f32) as u8;
            *pixel = image::Rgb([red, green, blue]);
        });

    img_buffer
}

pub fn gradient_radial(image_values: GeneratedGradientImage) -> RgbImage {
    let mut img_buffer = RgbImage::new(image_values.width, image_values.height);

    let center_x: f32 = image_values.width as f32 / 2.0;
    let center_y: f32 = image_values.height as f32 / 2.0;
    let max_distance: f32 = (center_x.powi(2) + center_y.powi(2)).sqrt();

    img_buffer
        .enumerate_pixels_mut()
        .par_bridge()
        .for_each(|(x, y, pixel)| {
            let dx: f32 = x as f32 - center_x;
            let dy: f32 = y as f32 - center_y;
            let distance: f32 = (dx.powi(2) + dy.powi(2)).sqrt() / max_distance;

            let red: u8 = ((1.0 - distance) * image_values.start_rgb.red as f32
                + distance * image_values.end_rgb.red as f32) as u8;
            let green: u8 = ((1.0 - distance) * image_values.start_rgb.green as f32
                + distance * image_values.end_rgb.green as f32) as u8;
            let blue: u8 = ((1.0 - distance) * image_values.start_rgb.blue as f32
                + distance * image_values.end_rgb.blue as f32) as u8;
            *pixel = image::Rgb([red, green, blue]);
        });

    img_buffer
}
