pub struct CropValues {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

impl CropValues {
    pub fn new() -> Self {
        CropValues {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        }
    }
}
