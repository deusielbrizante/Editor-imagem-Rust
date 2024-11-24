use crate::{
    enums::{
        type_generation::{TypeGeneration, TypeGenerationData},
        type_gradient::TypeGradient,
    },
    structs::generated::{GeneratedGradientImage, GeneratedImage, RGB},
    utils::generate_image::generate_gradients::{
        gradient_diagonal, gradient_horizontal, gradient_radial, gradient_vertical,
    },
};
use image::{DynamicImage, RgbImage};
use num_complex::Complex;
use rand::{rngs::ThreadRng, Rng};

pub fn generate_image(
    image_values: TypeGenerationData,
    type_generation: TypeGeneration,
    type_gradient: TypeGradient,
) -> DynamicImage {
    let img_buffer: RgbImage = match image_values {
        TypeGenerationData::Defaults(image_values) => match type_generation {
            TypeGeneration::Solid => generate_solid(image_values),
            TypeGeneration::Waves => generate_waves(image_values),
            TypeGeneration::Fractal => generate_fractal(image_values),
            _ => RgbImage::new(image_values.width, image_values.height),
        },
        TypeGenerationData::Gradient(gradient_values) => match type_gradient {
            TypeGradient::Horizontal => gradient_horizontal(gradient_values),
            TypeGradient::Vertical => gradient_vertical(gradient_values),
            TypeGradient::Diagonal => gradient_diagonal(gradient_values),
            TypeGradient::Radial => gradient_radial(gradient_values),
        },
    };

    DynamicImage::ImageRgb8(img_buffer)
}

fn generate_solid(image_values: GeneratedImage) -> RgbImage {
    let mut img_buffer = RgbImage::new(image_values.width, image_values.height);

    for pixel in img_buffer.pixels_mut() {
        *pixel = image::Rgb([image_values.red, image_values.green, image_values.blue]);
    }

    img_buffer
}

fn generate_waves(image_values: GeneratedImage) -> RgbImage {
    let mut img_buffer: RgbImage = RgbImage::new(image_values.width, image_values.height);

    let mut rng: ThreadRng = rand::thread_rng();
    let red_phase: f32 = rng.gen_range(0.0..std::f32::consts::PI);
    let green_phase: f32 = rng.gen_range(0.0..std::f32::consts::PI);
    let blue_phase: f32 = rng.gen_range(0.0..std::f32::consts::PI);

    let red_direction_x: f32 = rng.gen_range(0.05..0.5);
    let red_direction_y: f32 = rng.gen_range(0.05..0.5);

    let green_direction_x: f32 = rng.gen_range(0.05..0.5);
    let green_direction_y: f32 = rng.gen_range(0.05..0.5);

    let blue_direction_x: f32 = rng.gen_range(0.05..0.5);
    let blue_direction_y: f32 = rng.gen_range(0.05..0.5);

    for (x, y, pixel) in img_buffer.enumerate_pixels_mut() {
        let red: u8 = ((x as f32 * red_direction_x + y as f32 * red_direction_y + red_phase).sin()
            * 127.0
            + 128.0) as u8;
        let green: u8 =
            ((x as f32 * green_direction_x + y as f32 * green_direction_y + green_phase).cos()
                * 127.0
                + 128.0) as u8;
        let blue: u8 =
            ((x as f32 * blue_direction_x + y as f32 * blue_direction_y + blue_phase).sin() * 127.0
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

    img_buffer
}

fn generate_fractal(image_values: GeneratedImage) -> RgbImage {
    let mut img_buffer: RgbImage =
        create_random_background(image_values.width, image_values.height);

    let mut rng: ThreadRng = rand::thread_rng();

    let red_base: u8 = rng.gen_range(80..=255);
    let green_base: u8 = rng.gen_range(80..=255);
    let blue_base: u8 = rng.gen_range(80..=255);

    let scale_base: f32 = rng.gen_range(0.001..=2.0);

    let max_dimension: f32 = image_values.width.max(image_values.height) as f32;

    let scale_x: f32 = scale_base / max_dimension as f32;
    let scale_y: f32 = scale_base / max_dimension as f32;

    let blend_factor: f32 = rng.gen_range(0.1..1.0);

    let offset_x: f32 = rng.gen_range(-0.2..=0.2);
    let offset_y: f32 = rng.gen_range(-0.2..=0.2);

    for (x, y, pixel) in img_buffer.enumerate_pixels_mut() {
        let cx: f32 = y as f32 * scale_x - (scale_base / 2.0) + offset_x;
        let cy: f32 = x as f32 * scale_y - (scale_base / 2.0) + offset_y;

        let c: Complex<f32> = num_complex::Complex::new(-0.4, 0.6);
        let mut z: Complex<f32> = num_complex::Complex::new(cx, cy);

        let mut fractal: u32 = 0;
        while fractal < 255 && z.norm() <= 2.0 {
            z = z * z + c;
            fractal += 1;
        }

        let red: u8 = (red_base as f32 * (fractal as f32 / 255.0)) as u8;
        let green: u8 = (green_base as f32 * (fractal as f32 / 255.0)) as u8;
        let blue: u8 = (blue_base as f32 * (fractal as f32 / 255.0)) as u8;

        let bg_color: [u8; 3] = pixel.0;

        let mixed_red: u8 =
            (red as f32 * blend_factor + bg_color[0] as f32 * (1.0 - blend_factor)) as u8;
        let mixed_green: u8 =
            (green as f32 * blend_factor + bg_color[1] as f32 * (1.0 - blend_factor)) as u8;
        let mixed_blue: u8 =
            (blue as f32 * blend_factor + bg_color[2] as f32 * (1.0 - blend_factor)) as u8;

        *pixel = image::Rgb([mixed_red, mixed_green, mixed_blue]);
    }

    img_buffer
}

fn create_random_background(width: u32, height: u32) -> RgbImage {
    let random_gradient_functions: Vec<fn(GeneratedGradientImage) -> RgbImage> = vec![
        gradient_horizontal,
        gradient_vertical,
        gradient_diagonal,
        gradient_radial,
    ];

    let random_default_functions: Vec<fn(GeneratedImage) -> RgbImage> =
        vec![generate_solid, generate_waves];

    let mut rng: ThreadRng = rand::thread_rng();

    if rng.gen_range(0..=1) as u8 == 0 {
        let random_function: fn(GeneratedImage) -> RgbImage =
            random_default_functions[rng.gen_range(0..random_default_functions.len())];

        random_function(GeneratedImage {
            width,
            height,
            red: rng.gen_range(0..=255),
            green: rng.gen_range(0..=255),
            blue: rng.gen_range(0..=255),
        })
    } else {
        let random_function: fn(GeneratedGradientImage) -> RgbImage =
            random_gradient_functions[rng.gen_range(0..random_gradient_functions.len())];

        random_function(GeneratedGradientImage {
            width,
            height,
            start_rgb: RGB {
                red: rng.gen_range(0..=255),
                green: rng.gen_range(0..=255),
                blue: rng.gen_range(0..=255),
            },
            end_rgb: RGB {
                red: rng.gen_range(0..=255),
                green: rng.gen_range(0..=255),
                blue: rng.gen_range(0..=255),
            },
        })
    }
}
