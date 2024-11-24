use crate::structs::generated::{GeneratedGradientImage, GeneratedImage};

pub enum TypeGeneration {
    Solid,
    Waves,
    Gradient,
    Fractal,
}

pub enum TypeGenerationData {
    Gradient(GeneratedGradientImage),
    Defaults(GeneratedImage),
}
