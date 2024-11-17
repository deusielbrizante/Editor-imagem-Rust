use crate::{
    enums::type_execution::TypeExecution,
    utils::{execute_adjusts::execute_adjusts, process_image::process_image},
};
use image::DynamicImage;

pub fn blur(infile: String) -> (bool, DynamicImage) {
    let mut img: DynamicImage = DynamicImage::new_rgb8(1, 1);
    let mut not_continue: bool = true;

    match process_image(&infile.trim()) {
        Ok(validate_img) => img = validate_img,
        Err(_) => not_continue = false,
    };

    if !not_continue {
        return (not_continue, img);
    }

    (not_continue, execute_adjusts(img, &TypeExecution::Blur))
}
