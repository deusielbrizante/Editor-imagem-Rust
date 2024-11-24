use crate::utils::terminal::{clear_screen, display_menu, wait_enter};
use image::{
    DynamicImage, ImageError,
    ImageError::{Decoding, Encoding, IoError, Limits, Parameter, Unsupported},
    ImageFormat,
};
use std::io::{stdin, stdout, Write};

pub fn get_image() -> String {
    clear_screen();

    loop {
        let mut infile: String = String::new();

        print!("Mova a foto que deseja adicionar o efeito: ");
        stdout().flush().unwrap();

        infile.clear();
        stdin().read_line(&mut infile).unwrap();
        clear_screen();

        infile = infile
            .trim()
            .replace("\"", "")
            .replace("\'", "")
            .replace("\\", "/")
            .to_string();

        match ImageFormat::from_path(&infile) {
            Ok(_) => break infile,
            Err(_) => {
                println!("Arquivo inválido! Envie um arquivo de imagem!");
            }
        }
    }
}

pub fn get_image_name() -> String {
    let mut img_name: String = String::new();

    loop {
        clear_screen();

        img_name.clear();
        print!("Digite o nome que deseja dar para sua foto: ");
        stdout().flush().unwrap();

        stdin().read_line(&mut img_name).unwrap();

        let phrase: String = format!(
            "Deseja continuar com o nome \"{}\" para o arquivo ?",
            img_name.trim()
        );
        match display_menu(&phrase, &["Sim"], false) {
            1 => break,
            _ => continue,
        }
    }

    img_name.trim().to_string()
}

pub fn format_image() -> (String, ImageFormat) {
    let format_option_menu: [&str; 4] = ["PNG", "JPEG", "JPG", "ICO"];

    loop {
        match display_menu(
            "Qual tipo de formato você quer na sua imagem ?",
            &format_option_menu,
            true,
        ) {
            1 => break (String::from("png"), ImageFormat::Png),
            2 => break (String::from("jpeg"), ImageFormat::Jpeg),
            3 => break (String::from("jpg"), ImageFormat::Jpeg),
            4 => break (String::from("ico"), ImageFormat::Ico),
            _ => {
                clear_screen();
                println!("Escolha uma opção válida!");
                wait_enter();
            }
        };
    }
}

pub fn save_image(img: &DynamicImage, outfile: String, format: ImageFormat) {
    clear_screen();

    match img.save_with_format(outfile, format) {
        Ok(_) => {
            println!("Imagem salva com sucesso !!!");
            wait_enter();
        }
        Err(_) => {
            println!("Erro ao salvar a imagem...");
            wait_enter();
        }
    }
}

pub fn process_image(infile: &&str) -> Result<DynamicImage, ImageError> {
    match image::open(infile) {
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
