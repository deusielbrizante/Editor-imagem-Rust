use crate::utils::terminal::wait_enter;
use image::{
    DynamicImage, ImageError,
    ImageError::{Decoding, Encoding, IoError, Limits, Parameter, Unsupported},
};

pub fn process_image(infile: &&str) -> Result<DynamicImage, ImageError> {
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
