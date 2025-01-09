use crate::font_manager::FontInfo;

struct Layouter {
}

impl Layouter {
    pub fn new() -> Layouter {
        Layouter {}
    }

    pub fn generate_layout(&self, info: &FontInfo, text: &str, size: f32, width: f32) {
        println!("Layouting...");
    }
}