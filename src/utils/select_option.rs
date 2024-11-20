use crate::{
    enums::type_execution::TypeExecution,
    utils::{
        execute_adjusts::execute_adjusts,
        generate_image::performs_creation::perform_generate_image, image::process_image,
    },
};
use image::DynamicImage;

pub fn select_simple_option(infile: String, type_execution: TypeExecution) -> (bool, DynamicImage) {
    let mut img: DynamicImage = DynamicImage::new_rgb8(1, 1);
    let mut not_continue: bool = true;

    match process_image(&infile.trim()) {
        Ok(validate_img) => img = validate_img,
        Err(_) => not_continue = false,
    };

    if !not_continue {
        return (not_continue, img);
    }

    (not_continue, execute_adjusts(img, &type_execution))
}

pub fn select_generate_option() -> (bool, DynamicImage) {
    (true, perform_generate_image())
}
