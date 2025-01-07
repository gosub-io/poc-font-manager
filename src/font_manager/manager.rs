use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Arc;
use anyhow::anyhow;
use log::error;
use crate::font_manager::font_info::{FontInfo, FontStyle};
use crate::font_manager::sources::{FontSource, FontSourceType};
#[cfg(feature = "source_fontique")]
use crate::font_manager::sources::fontique::FontiqueSource;
#[cfg(feature = "source_fontkit")]
use crate::font_manager::sources::fontkit::FontKitSource;
#[cfg(feature = "source_pango")]
use crate::font_manager::sources::pango::PangoSource;
#[cfg(feature = "source_parley")]
use crate::font_manager::sources::parley::ParleySource;

#[allow(dead_code)]
pub const LOG_TARGET: &str = "font-manager";

pub struct FontManager {
    sources: HashMap<FontSourceType, Box<dyn FontSource>>,
}

impl FontManager {
    pub fn new() -> Self {
        let mut sources : HashMap<FontSourceType, Box<dyn FontSource>> = HashMap::new();

        #[cfg(feature = "source_fontkit")]
        sources.insert(FontSourceType::Fontkit, Box::new(FontKitSource::new()));
        #[cfg(feature = "source_fontique")]
        sources.insert(FontSourceType::Fontique, Box::new(FontiqueSource::new()));
        #[cfg(feature = "source_parley")]
        sources.insert(FontSourceType::Parley, Box::new(ParleySource::new()));
        #[cfg(feature = "source_pango")]
        sources.insert(FontSourceType::Pango, Box::new(PangoSource::new()));

        Self {
            sources,
        }
    }

    // Returns all compiled in font sources
    pub fn sources(&self) -> Vec<FontSourceType> {
        let mut v = vec![];

        for key in self.sources.keys() {
            v.push(*key);
        }

        v
    }

    /// Returns all available fonts for given source-type
    pub fn available_fonts(&self, source_type: FontSourceType) -> Vec<FontInfo> {
        match self.sources.get(&source_type) {
            Some(source) => {
                let mut fonts = source.available_fonts().to_vec();
                fonts.sort_by_key(|fi| fi.family.clone());
                fonts
            },
            None => {
                error!(target: LOG_TARGET, "Unknown font source: {:?}", source_type);
                vec![]
            }
        }
    }

    pub fn find(&self, source_type: FontSourceType, families: &[&str], style: FontStyle) -> Option<FontInfo> {
        for &fam in families {
            for fi in self.available_fonts(source_type) {
                if fi.family.eq_ignore_ascii_case(fam) && fi.style == style {
                    return Some(fi.clone());
                }
            }
        }

        None
    }
}

impl FontManager {
    #[cfg(feature = "source_fontique")]
    pub fn fontique_load_font(&self, _font_info: &FontInfo) -> Result<fontique::FontInfo, anyhow::Error> {
        let Some(_source) = self.sources.get(&FontSourceType::Fontique) else {
            return Err(anyhow!("Fontique source not found"))
        };

        let _a = 1;

        Err(anyhow!("Not implemented"))
    }

    #[cfg(feature = "source_fontkit")]
    pub fn fontkit_load_freetype_font(&self, font_info: &FontInfo) -> Result<freetype::Face, anyhow::Error> {
        let Some(source) = self.sources.get(&FontSourceType::Fontkit) else {
            return Err(anyhow!("FontKit source not found"))
        };

        let ps = source.as_any().downcast_ref::<FontKitSource>()
            .ok_or_else(|| anyhow!("Failed to downcast FontKitSource"))?;

        ps.load_freetype_font(font_info)
    }

    #[cfg(feature = "source_pango")]
    pub fn pango_load_font(&self, font_info: &FontInfo) -> Result<pangocairo::pango::Font, anyhow::Error> {
        let Some(source) = self.sources.get(&FontSourceType::Pango) else {
            return Err(anyhow!("Pango source not found"))
        };

        let ps = source.as_any().downcast_ref::<PangoSource>()
            .ok_or_else(|| anyhow!("Failed to downcast PangoSource"))?;

        ps.load_font(font_info)
    }

    #[cfg(feature = "source_pango")]
    pub fn pango_get_description(&self, font_info: &FontInfo, size: f64) -> Result<pangocairo::pango::FontDescription, anyhow::Error> {
        let Some(source) = self.sources.get(&FontSourceType::Pango) else {
            return Err(anyhow!("Pango source not found"))
        };

        let ps = source.as_any().downcast_ref::<PangoSource>()
            .ok_or_else(|| anyhow!("Failed to downcast PangoSource"))?;

        Ok(ps.get_description(font_info, size))
    }

    #[cfg(feature = "source_parley")]
    pub fn parley_context(&self) -> Result<Arc<RefCell<parley::FontContext>>, anyhow::Error> {
        let Some(source) = self.sources.get(&FontSourceType::Parley) else {
            return Err(anyhow!("Parley source not found"))
        };

        let ps = source.as_any().downcast_ref::<ParleySource>()
            .ok_or_else(|| anyhow!("Failed to downcast ParleySource"))?;

        Ok(ps.context())
    }

    #[cfg(feature = "source_parley")]
    pub fn parley_get_font_stack(&self, font_info: &FontInfo) -> Result<parley::FontStack, anyhow::Error> {
        let Some(source) = self.sources.get(&FontSourceType::Parley) else {
            return Err(anyhow!("Parley source not found"))
        };

        let ps = source.as_any().downcast_ref::<ParleySource>()
            .ok_or_else(|| anyhow!("Failed to downcast ParleySource"))?;

        Ok(ps.get_font_stack(font_info.family.clone()))
    }
}