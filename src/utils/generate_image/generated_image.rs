use crate::{
    enums::{
        type_generation::{TypeGeneration, TypeGenerationData},
        type_gradient::TypeGradient,
    },
    structs::generated::GeneratedImage,
    utils::generate_image::generate_gradients::{
        gradient_diagonal, gradient_horizontal, gradient_radial, gradient_vertical,
    },
};
use image::{DynamicImage, RgbImage};
use rand::Rng;

pub fn generate_image(
    image_values: TypeGenerationData,
    type_generation: TypeGeneration,
    type_gradient: TypeGradient,
) -> DynamicImage {
    let img_buffer: RgbImage = match image_values {
        TypeGenerationData::Solid(image_values) => {
            let mut buffer = RgbImage::new(image_values.width, image_values.height);

            match type_generation {
                TypeGeneration::Solid => generate_solid(&mut buffer, image_values),
                TypeGeneration::Waves => generate_waves(&mut buffer, image_values),
                _ => RgbImage::new(image_values.width, image_values.height),
            }
        }
        TypeGenerationData::Gradient(gradient_values) => {
            let mut buffer = RgbImage::new(gradient_values.width, gradient_values.height);

            match type_gradient {
                TypeGradient::Horizontal => gradient_horizontal(&mut buffer, gradient_values),
                TypeGradient::Vertical => gradient_vertical(&mut buffer, gradient_values),
                TypeGradient::Diagonal => gradient_diagonal(&mut buffer, gradient_values),
                TypeGradient::Radial => gradient_radial(&mut buffer, gradient_values),
            }
        }
    };

    DynamicImage::ImageRgb8(img_buffer)
}

fn generate_solid(img_buffer: &mut RgbImage, image_values: GeneratedImage) -> RgbImage {
    *img_buffer = RgbImage::new(image_values.width, image_values.height);

    for pixel in img_buffer.pixels_mut() {
        *pixel = image::Rgb([image_values.red, image_values.green, image_values.blue]);
    }

    img_buffer.clone()
}

fn generate_waves(img_buffer: &mut RgbImage, image_values: GeneratedImage) -> RgbImage {
    *img_buffer = RgbImage::new(image_values.width, image_values.height);

    let mut rng = rand::thread_rng();
    let red_phase = rng.gen_range(0.0..std::f32::consts::PI);
    let green_phase = rng.gen_range(0.0..std::f32::consts::PI);
    let blue_phase = rng.gen_range(0.0..std::f32::consts::PI);

    let red_direction_x = rng.gen_range(0.05..0.5);
    let red_direction_y = rng.gen_range(0.05..0.5);

    let green_direction_x = rng.gen_range(0.05..0.5);
    let green_direction_y = rng.gen_range(0.05..0.5);

    let blue_direction_x = rng.gen_range(0.05..0.5);
    let blue_direction_y = rng.gen_range(0.05..0.5);

    for (x, y, pixel) in img_buffer.enumerate_pixels_mut() {
        let red = ((x as f32 * red_direction_x + y as f32 * red_direction_y + red_phase).sin()
            * 127.0
            + 128.0) as u8;
        let green = ((x as f32 * green_direction_x + y as f32 * green_direction_y + green_phase)
            .cos()
            * 127.0
            + 128.0) as u8;
        let blue = ((x as f32 * blue_direction_x + y as f32 * blue_direction_y + blue_phase).sin()
            * 127.0
            + 128.0) as u8;

        *pixel = image::Rgb([
            if image_values.red != 0 {
                red % image_values.red
            } else {
                0
            },
            if image_values.green != 0 {
                green % image_values.green
            } else {
                0
            },
            if image_values.blue != 0 {
                blue % image_values.blue
            } else {
                0
            },
        ]);
    }

    img_buffer.clone()
}