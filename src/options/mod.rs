mod blur;

use crate::options::blur::execute_blur;
use crate::utils::terminal::wait_enter;
use image::ImageError::{Decoding, Encoding, IoError, Limits, Parameter, Unsupported};
use image::{DynamicImage, ImageError};

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

    let img_blur: DynamicImage = execute_blur(img);

    (not_continue, img_blur)
}

fn process_image(infile: &&str) -> Result<DynamicImage, ImageError> {
    let img: Result<DynamicImage, ImageError> = image::open(infile);
    match img {
        Ok(img) => Ok(img),
        Err(err) => match err {
            IoError(io_err) => {
                println!("Erro na entrada ou saída {:?}", io_err);
                wait_enter();
                Err(IoError(io_err))
            }
            Decoding(decode_err) => {
                println!("Erro ao decodificar esse tipo de arquivo! {:?}", decode_err);
                wait_enter();
                Err(Decoding(decode_err))
            }
            Encoding(encode_err) => {
                println!("Erro ao codificar esse tipo de arquivo! {:?}", encode_err);
                wait_enter();
                Err(Encoding(encode_err))
            }
            Limits(limit_err) => {
                println!(
                    "A conclusão da operação exige mais recursos do que o permitido! {:?}",
                    limit_err
                );
                wait_enter();
                Err(Limits(limit_err))
            }
            Parameter(param_err) => {
                println!("Erro no formato do arquivo! {:?}", param_err);
                wait_enter();
                Err(Parameter(param_err))
            }
            Unsupported(unsupported_err) => {
                println!(
                    "Erro ao tentar executar a ação. Reveja a ordem das ações pedida! {:?}",
                    unsupported_err
                );
                wait_enter();
                Err(Unsupported(unsupported_err))
            }
        },
    }
}
