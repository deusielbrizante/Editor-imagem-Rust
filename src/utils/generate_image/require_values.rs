use crate::{
    enums::{type_generation::TypeGeneration, type_gradient::TypeGradient},
    structs::{
        crop::CropValues,
        generated::{GeneratedGradientImage, GeneratedImage},
    },
    utils::terminal::clear_screen,
};
use std::io::{stdin, stdout, Write};

pub fn require_values(phrase: &str) -> f32 {
    let mut image_adjusted: String = String::new();

    loop {
        print!("{}", phrase);
        stdout().flush().unwrap();

        stdin()
            .read_line(&mut image_adjusted)
            .expect("Falha ao ler o valor digitado!");

        match image_adjusted.trim().parse() {
            Ok(option) => break option,
            _ => println!("Digite apenas números!"),
        }

        image_adjusted.clear();
    }
}

pub fn require_other_values() -> GeneratedImage {
    let mut generated_values: GeneratedImage = GeneratedImage::new();

    let phrases: [&str; 5] = [
        "Digite a largura da imagem que deseja gerar: ",
        "Digite a altura da imagem que deseja gerar: ",
        "Digite o valor de 'vermelho' que deseja para a imagem: ",
        "Digite o valor de 'verde' que deseja para a imagem: ",
        "Digite o valor de 'azul' que deseja para a imagem: ",
    ];

    generated_values.width = loop {
        let value_temp: f32 = require_values(phrases[0]);

        if value_temp < 1.0 {
            println!("O valor da 'largura' não pode ser menor que 1!\n");
            continue;
        }

        if value_temp > u32::MAX as f32 {
            println!(
                "O valor da 'largura' não pode ser maior que {}!\n",
                u32::MAX
            );
            continue;
        }

        break value_temp as u32;
    };

    generated_values.height = loop {
        let value_temp: f32 = require_values(phrases[1]);

        if value_temp < 1.0 {
            println!("O valor da 'altura' não pode ser menor que 1!\n");
            continue;
        }

        if value_temp > u32::MAX as f32 {
            println!("O valor da 'altura' não pode ser maior que {}!\n", u32::MAX);
            continue;
        }

        break value_temp as u32;
    };

    generated_values.red = loop {
        let value_temp: f32 = require_values(phrases[2]);

        if value_temp > 255.0 || value_temp < 0.0 {
            println!("O valor de 'vermelho' deve ser entre 0 e 255!\n");
            continue;
        }

        break value_temp as u8;
    };

    generated_values.green = loop {
        let value_temp: f32 = require_values(phrases[3]);

        if value_temp < 0.0 || value_temp > 255.0 {
            println!("O valor de 'verde' deve ser entre 0 e 255!\n");
            continue;
        }

        break value_temp as u8;
    };

    generated_values.blue = loop {
        let value_temp: f32 = require_values(phrases[4]);

        if value_temp < 0.0 || value_temp > 255.0 {
            println!("O valor de 'azul' deve ser entre 0 e 255!\n");
            continue;
        }

        break value_temp as u8;
    };

    generated_values
}

pub fn require_fractal_values() -> GeneratedImage {
    let mut generated_values: GeneratedImage = GeneratedImage::new();

    let phrases: [&str; 2] = [
        "Digite a largura da imagem que deseja gerar: ",
        "Digite a altura da imagem que deseja gerar: ",
    ];

    generated_values.width = loop {
        let value_temp: f32 = require_values(phrases[0]);

        if value_temp < 1.0 {
            println!("O valor da 'largura' não pode ser menor que 1!\n");
            continue;
        }

        if value_temp > u32::MAX as f32 {
            println!(
                "O valor da 'largura' não pode ser maior que {}!\n",
                u32::MAX
            );
            continue;
        }

        break value_temp as u32;
    };

    generated_values.height = loop {
        let value_temp: f32 = require_values(phrases[1]);

        if value_temp < 1.0 {
            println!("O valor da 'altura' não pode ser menor que 1!\n");
            continue;
        }

        if value_temp > u32::MAX as f32 {
            println!("O valor da 'altura' não pode ser maior que {}!\n", u32::MAX);
            continue;
        }

        break value_temp as u32;
    };

    generated_values
}

pub fn require_gradient_values() -> (GeneratedGradientImage, TypeGradient) {
    let mut gradient_values: GeneratedGradientImage = GeneratedGradientImage::new();

    let phrases: [&str; 9] = [
        "Digite a largura do gradiente que deseja gerar: ",
        "Digite a altura do gradiente que deseja gerar: ",
        "Digite o valor inicial de 'vermelho' que deseja para o gradiente: ",
        "Digite o valor inicial de 'verde' que deseja para o gradiente: ",
        "Digite o valor inicial de 'azul' que deseja para o gradiente: ",
        "Digite o valor final de 'vermelho' que deseja para o gradiente: ",
        "Digite o valor final de 'verde' que deseja para o gradiente: ",
        "Digite o valor final de 'azul' que deseja para o gradiente: ",
        "Digite o tipo de gradiente que deseja gerar:\n\n1 - Horizontal\n2 - Vertical\n3 - Diagonal\n4 - Radial\n\nSelecione alguma das opções: ",
    ];

    gradient_values.width = loop {
        let value_temp: f32 = require_values(phrases[0]);

        if value_temp < 1.0 {
            println!("O valor da 'largura' não pode ser menor que 1!\n");
            continue;
        }

        if value_temp > u32::MAX as f32 {
            println!(
                "O valor da 'largura' não pode ser maior que {}!\n",
                u32::MAX
            );
            continue;
        }

        break value_temp as u32;
    };

    gradient_values.height = loop {
        let value_temp: f32 = require_values(phrases[1]);

        if value_temp < 1.0 {
            println!("O valor da 'altura' não pode ser menor que 1!\n");
            continue;
        }

        if value_temp > u32::MAX as f32 {
            println!("O valor da 'altura' não pode ser maior que {}!\n", u32::MAX);
            continue;
        }

        break value_temp as u32;
    };

    gradient_values.start_rgb.red = loop {
        let value_temp: f32 = require_values(phrases[2]);

        if value_temp > 255.0 || value_temp < 0.0 {
            println!("O valor de 'vermelho' deve ser entre 0 e 255!\n");
            continue;
        }

        break value_temp as u8;
    };

    gradient_values.start_rgb.green = loop {
        let value_temp: f32 = require_values(phrases[3]);

        if value_temp < 0.0 || value_temp > 255.0 {
            println!("O valor de 'verde' deve ser entre 0 e 255!\n");
            continue;
        }

        break value_temp as u8;
    };

    gradient_values.start_rgb.blue = loop {
        let value_temp: f32 = require_values(phrases[4]);

        if value_temp < 0.0 || value_temp > 255.0 {
            println!("O valor de 'azul' deve ser entre 0 e 255!\n");
            continue;
        }

        break value_temp as u8;
    };

    gradient_values.end_rgb.red = loop {
        let value_temp: f32 = require_values(phrases[5]);

        if value_temp > 255.0 || value_temp < 0.0 {
            println!("O valor de 'vermelho' deve ser entre 0 e 255!\n");
            continue;
        }

        break value_temp as u8;
    };

    gradient_values.end_rgb.green = loop {
        let value_temp: f32 = require_values(phrases[6]);

        if value_temp < 0.0 || value_temp > 255.0 {
            println!("O valor de 'verde' deve ser entre 0 e 255!\n");
            continue;
        }

        break value_temp as u8;
    };

    gradient_values.end_rgb.blue = loop {
        let value_temp: f32 = require_values(phrases[7]);

        if value_temp < 0.0 || value_temp > 255.0 {
            println!("O valor de 'azul' deve ser entre 0 e 255!\n");
            continue;
        }

        break value_temp as u8;
    };

    clear_screen();

    let type_gradient: TypeGradient = loop {
        let value_temp: f32 = require_values(phrases[8]);

        if value_temp < 1.0 || value_temp > 4.0 {
            println!("Digite uma das opções acima!\n");
            continue;
        }

        break match value_temp as u32 {
            1 => TypeGradient::Horizontal,
            2 => TypeGradient::Vertical,
            3 => TypeGradient::Diagonal,
            4 => TypeGradient::Radial,
            _ => TypeGradient::Horizontal,
        };
    };

    (gradient_values, type_gradient)
}

pub fn require_values_crop(width_image: u32, height_image: u32) -> CropValues {
    let mut values_crop: CropValues = CropValues::new();

    let phrases: [&str; 4] = [
        "\nDigite o local que será recortado a imagem no eixo 'x': ",
        "\nDigite o local que será recortado a imagem no eixo 'y': ",
        "\nDigite o tamanho da 'largura' que terá o corte: ",
        "\nDigite o tamanho da 'altura' que terá o corte: ",
    ];

    print!(
        "Largura da imagem: {}, Altura da imagem: {}",
        width_image, height_image
    );

    loop {
        values_crop.x = require_values(phrases[0]) as u32;

        if values_crop.x > width_image {
            println!("O valor de 'x' não pode ser maior que a largura da imagem!");
            continue;
        }

        break;
    }

    loop {
        values_crop.y = require_values(phrases[1]) as u32;

        if values_crop.y > height_image {
            println!("O valor de 'y' não pode ser maior que a altura da imagem!\n");
            continue;
        }

        break;
    }

    loop {
        values_crop.width = require_values(phrases[2]) as u32;

        if values_crop.width < 1 {
            println!("O valor da 'largura' não pode ser menor que 1!\n");
            continue;
        }

        break;
    }

    loop {
        values_crop.height = require_values(phrases[3]) as u32;

        if values_crop.height < 1 {
            println!("O valor da 'altura' não pode ser menor que 1!\n");
            continue;
        }

        break;
    }

    values_crop
}

pub fn require_values_rotation() -> f32 {
    print!("Escolha um dos valores abaixo para rotacionar a imagem:\n\n1 - 90°\n2 - 180°\n3 - 270°\n\n");

    loop {
        let value_rotation = require_values("Selecione alguma das opções: ");

        if value_rotation < 1.0 || value_rotation > 3.0 {
            println!("Digite uma das opções acima!\n");
            continue;
        }

        break value_rotation;
    }
}

pub fn require_values_generation_image() -> TypeGeneration {
    loop {
        let value_temp: f32 = require_values(
            "Escolha o tipo de imagem que deseja gerar:\n1 - Sólida\n2 - Ondas\n3 - Gradiente\n4 - Fractal\nSelecione alguma das opções: ",
        );

        if value_temp < 1.0 || value_temp > 4.0 {
            println!("Digite uma das opções acima!\n");
            continue;
        }

        clear_screen();

        break match value_temp as u32 {
            1 => TypeGeneration::Solid,
            2 => TypeGeneration::Waves,
            3 => TypeGeneration::Gradient,
            4 => TypeGeneration::Fractal,
            _ => TypeGeneration::Solid,
        };
    }
}
