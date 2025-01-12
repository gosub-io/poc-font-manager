use crate::font_manager::font_info::{FontInfo, FontStyle};

pub mod fontkit;
pub mod fontique;
pub mod parley;
pub mod pango;

pub trait FontSource {
    fn new() -> Self where Self: Sized;
    fn available_fonts(&self) -> &[FontInfo];
    fn find(&self, family: &[&str], style: FontStyle) -> Result<FontInfo, anyhow::Error>;
}

#[derive(Clone, Copy, Hash, Eq, PartialEq, Debug)]
pub enum FontSourceType {
    Fontkit,
    Fontique,
    Parley,
    Pango,
}