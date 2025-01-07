use std::path::PathBuf;
use crate::font_manager::sources::FontSourceType;

#[derive(Clone, Debug)]
pub enum FontStyle {
    Normal,
    Italic,
    Oblique,
}

impl std::fmt::Display for FontStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            FontStyle::Normal => write!(f, "Normal"),
            FontStyle::Italic => write!(f, "Italic"),
            FontStyle::Oblique => write!(f, "Oblique"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct FontInfo {
    /// Family name of the font (e.g. "Arial")
    pub family: String,
    /// Style of the font
    pub style: FontStyle,
    /// Weight (400 normal, 700 bold)
    pub weight: f32,
    /// Stretch (1.0 normal, < 1.0 condensed)
    pub stretch: f32,
    /// Font is monospaced
    pub monospaced: bool,
    /// Path to the font file
    pub path: Option<PathBuf>,
    /// Index of the face in the font-file
    pub index: Option<i32>,
    /// Source type of the font
    pub source_type: FontSourceType,
}