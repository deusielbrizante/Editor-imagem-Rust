use crate::{
    enums::type_execution::TypeExecution,
    utils::{execute_adjusts::execute_adjusts, process_image::process_image},
};
use image::DynamicImage;

pub fn brighten(infile: String) -> (bool, DynamicImage) {
    // See blur() for an example of how to open / save an image.

    // .brighten() takes one argument, an i32.  Positive numbers brighten the
    // image. Negative numbers darken it.  It returns a new image.

    // Challenge: parse the brightness amount from the command-line and pass it
    // through to this function.
    let mut img = DynamicImage::new_rgb8(1, 1);
    let mut not_continue: bool = true;

    match process_image(&infile.trim()) {
        Ok(validate_img) => img = validate_img,
        Err(_) => not_continue = false,
    }

    if !not_continue {
        return (not_continue, img);
    }

    (not_continue, execute_adjusts(img, &TypeExecution::Brighten))
}
