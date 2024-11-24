use crate::{
    enums::type_execution::TypeExecution,
    utils::generate_image::performs_creation::{
        perform_blur_brightness, perform_crop, perform_invert, perform_rotate,
    },
};
use image::DynamicImage;

pub fn execute_adjusts(img: &mut DynamicImage, type_execution: &TypeExecution) {
    match type_execution {
        TypeExecution::Blur | TypeExecution::Brighten => {
            perform_blur_brightness(type_execution, img);
        }
        TypeExecution::Crop => perform_crop(img),
        TypeExecution::Rotate => perform_rotate(img),
        TypeExecution::Invert => perform_invert(img),
        TypeExecution::Grayscale => {
            *img = img.grayscale();
        }
    };
}
