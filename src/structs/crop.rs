pub struct CropValues {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

impl CropValues {
    // pub fn new_values(x: u32, y: u32, width: u32, height: u32) -> Self {
    //     CropValues {
    //         x,
    //         y,
    //         width,
    //         height,
    //     }
    // }

    pub fn new() -> Self {
        CropValues {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        }
    }
}
