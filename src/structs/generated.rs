pub struct GeneratedImage {
    pub width: u32,
    pub height: u32,
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl GeneratedImage {
    pub fn new() -> GeneratedImage {
        GeneratedImage {
            width: 0,
            height: 0,
            red: 0,
            green: 0,
            blue: 0,
        }
    }
}

pub struct GeneratedGradientImage {
    pub width: u32,
    pub height: u32,
    pub start_rgb: RGB,
    pub end_rgb: RGB,
}

impl GeneratedGradientImage {
    // pub fn new(width: u32, height: u32, r: u8, g: u8, b: u8) -> GeneratedSolidImage {
    //     GeneratedSolidImage {
    //         width,
    //         height,
    //         red: r,
    //         green: g,
    //         blue: b,
    //     }
    // }

    pub fn new() -> GeneratedGradientImage {
        GeneratedGradientImage {
            width: 0,
            height: 0,
            start_rgb: RGB::new(),
            end_rgb: RGB::new(),
        }
    }
}

pub struct RGB {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl RGB {
    pub fn new() -> RGB {
        RGB {
            red: 0,
            green: 0,
            blue: 0,
        }
    }
}
