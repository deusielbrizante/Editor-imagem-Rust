use crate::{
    enums::{
        type_execution::TypeExecution,
        type_generation::{TypeGeneration, TypeGenerationData},
        type_gradient::TypeGradient,
    },
    structs::{
        crop::CropValues,
        generated::{GeneratedGradientImage, GeneratedImage},
    },
    utils::generate_image::{
        generated_image::generate_image,
        require_values::{
            require_fractal_values, require_gradient_values, require_other_values, require_values,
            require_values_crop, require_values_generation_image, require_values_rotation,
        },
    },
};
use image::DynamicImage;

pub fn perform_blur_brightness(type_execution: &TypeExecution, img: &mut DynamicImage) {
    let phrases: [&str; 2] = [
        "Digite a intensidade de blur você quer na imagem: ",
        "Digite a intensidade de brilho você quer na imagem: ",
    ];

    *img = match type_execution {
        TypeExecution::Blur => img.blur(require_values(phrases[0])),
        TypeExecution::Brighten => img.brighten(require_values(phrases[1]).round() as i32),
        _ => img.clone(),
    };
}

pub fn perform_crop(img: &mut DynamicImage) {
    let values_crop: CropValues = require_values_crop(img.width(), img.height());

    *img = img.crop(
        values_crop.x,
        values_crop.y,
        values_crop.width,
        values_crop.height,
    );
}

pub fn perform_rotate(img: &mut DynamicImage) {
    *img = match require_values_rotation() as u32 {
        1 => img.rotate90(),
        2 => img.rotate180(),
        3 => img.rotate270(),
        _ => img.clone(),
    };
}

pub fn perform_invert(img: &mut DynamicImage) {
    let mut img_inverted: DynamicImage = img.clone();
    img_inverted.invert();
    *img = img_inverted;
}

pub fn perform_generate_image() -> DynamicImage {
    let type_generated: TypeGeneration = require_values_generation_image();

    match type_generated {
        TypeGeneration::Gradient => {
            let (gradient_values, type_gradient): (GeneratedGradientImage, TypeGradient) =
                require_gradient_values();

            generate_image(
                TypeGenerationData::Gradient(gradient_values),
                type_generated,
                type_gradient,
            )
        }
        TypeGeneration::Fractal => {
            let generated_values: GeneratedImage = require_fractal_values();

            generate_image(
                TypeGenerationData::Defaults(generated_values),
                type_generated,
                TypeGradient::Horizontal,
            )
        }
        TypeGeneration::Solid | TypeGeneration::Waves => {
            let generated_values: GeneratedImage = require_other_values();

            generate_image(
                TypeGenerationData::Defaults(generated_values),
                type_generated,
                TypeGradient::Horizontal,
            )
        }
    }
}
