use crate::font_manager::font_info::FontInfo;

#[allow(dead_code)]
struct Layouter {
}

impl Layouter {
    #[allow(dead_code)]
    pub fn new() -> Layouter {
        Layouter {}
    }

    #[allow(dead_code)]
    pub fn generate_layout(&self, _info: &FontInfo, _text: &str, _size: f32, _width: f32) {
        println!("Layouting...");
    }
}