use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use anyhow::anyhow;
use font_kit::family_name::FamilyName;
use font_kit::handle::Handle;
use font_kit::properties::Properties;
use font_kit::source::SystemSource;
use cairo::freetype::{Face, Library};
use log::{error, info};

#[allow(dead_code)]
const LOG_TARGET: &str = "font-manager";

#[derive(Clone, Debug)]
pub enum FontStyle {
    Normal,
    Italic,
    Oblique,
}

impl Display for FontStyle {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
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
    pub path: PathBuf,
    /// Index of the face in the font-file
    pub index: Option<i32>,
}

#[allow(unused)]
pub struct FontManager {
    source: SystemSource,
    ft_library: Library,
    /// Vec of all font-info structures found
    font_info: Vec<FontInfo>,
    /// List of all font handles
    handles: Vec<Handle>,
    /// Cache of font faces that are loaded through freetype
    face_cache: Arc<Mutex<HashMap<String, Face>>>,
}

impl FontManager {
    pub fn new() -> Self {
        let library = Library::init().expect("unable to init freetype library");

        let source = SystemSource::new();
        let handles = source.all_fonts().unwrap();

        let mut font_info = Vec::new();
        for handle in &handles {
            match Self::handle_to_info(handle) {
                Ok(info) => font_info.push(info),
                Err(e) => {
                    error!(target: LOG_TARGET, "unable to load font: {}", e);
                }
            }
        }

        FontManager {
            source,
            ft_library: library,
            font_info: font_info,
            handles,
            face_cache: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Returns all available fonts
    pub fn available_fonts(&self) -> Vec<FontInfo> {
        let mut fonts = self.font_info.clone();
        fonts.sort_by_key(|fi| fi.family.clone());

        fonts
    }

    /// Returns the first font that matches the given family and style
    pub fn find(&self, families: Vec<&str>, style: FontStyle) -> Option<FontInfo> {
        let mut f = Vec::new();
        for family in families {
            let family = family.replace('\'', "");
            let family = family.trim();
            f.push(match family {
                "serif" => FamilyName::Serif,
                "sans-serif" => FamilyName::SansSerif,
                "monospace" => FamilyName::Monospace,
                "cursive" => FamilyName::Cursive,
                "fantasy" => FamilyName::Fantasy,
                _ => FamilyName::Title(family.to_string()),
            });
        }

        let mut properties = Properties::default();
        match style {
            FontStyle::Italic => properties.style(font_kit::properties::Style::Italic),
            FontStyle::Oblique => properties.style(font_kit::properties::Style::Oblique),
            FontStyle::Normal => properties.style(font_kit::properties::Style::Normal),
        };

        match self.source.select_best_match(&f, &properties) {
            Ok(handle) => match Self::handle_to_info(&handle) {
                Ok(info) => Some(info),
                Err(_) => None,
            }
            Err(_) => None,
        }
    }

    /// Load the font face for the given font info
    pub fn load(&self, font_info: &FontInfo) -> Result<Face, anyhow::Error> {
        let cache_key = format!("{}:{}", font_info.family, font_info.style);
        if let Some(font_face) = self.face_cache.lock().unwrap().get(&cache_key) {
            info!(target: LOG_TARGET, "Font loaded from cache: {}", cache_key);
            return Ok(font_face.clone());
        }

        let face = match self.ft_library.new_face(&font_info.path, font_info.index.unwrap_or(0) as isize) {
            Ok(face) => face,
            Err(e) => {
                error!(target: LOG_TARGET, "unable to load font: {}", e);
                return Err(anyhow!("unable to load font"))
            }
        };

        info!(target: LOG_TARGET,
            "Font loaded: {} (number of glyphs: {})",
            face.family_name().unwrap_or("Unknown".parse()?),
            face.num_glyphs()
        );

        info!(target: LOG_TARGET, "Caching font face: {}", cache_key);
        self.face_cache.lock().unwrap().insert(cache_key.clone(), face.clone());
        Ok(face)
    }

    /// Converts a font handle to a gosub font info structure
    fn handle_to_info(handle: &Handle) -> Result<FontInfo, anyhow::Error> {
        let font = handle.load().unwrap();

        let family = font.family_name();
        let props = font.properties();

        let style = match props.style {
            font_kit::properties::Style::Normal => FontStyle::Normal,
            font_kit::properties::Style::Italic => FontStyle::Italic,
            font_kit::properties::Style::Oblique => FontStyle::Oblique,
        };

        let Handle::Path {
            ref path,
            font_index,
        } = handle else {
            error!(target: LOG_TARGET, "Expected a path handle. Got: {:?}", handle);
            return Err(anyhow!("Expected a path handle"));
        };

        Ok(FontInfo {
            family,
            style,
            weight: props.weight.0,
            stretch: props.stretch.0,
            monospaced: font.is_monospace(),
            path: path.clone(),
            index: Some(*font_index as i32),
        })
    }
}
