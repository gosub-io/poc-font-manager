use std::collections::HashMap;
use anyhow::anyhow;
use log::error;
use crate::font_manager::font_info::{FontInfo, FontStyle};
use crate::font_manager::sources::{FontSource, FontSourceType};
use crate::font_manager::sources::fontique::FontiqueSource;
use crate::font_manager::sources::fontkit::FontKitSource;
use crate::font_manager::sources::pango::PangoSource;
use crate::font_manager::sources::parley::ParleySource;

#[allow(dead_code)]
pub const LOG_TARGET: &str = "font-manager";

pub struct FontManager {
    sources: HashMap<FontSourceType, Box<dyn FontSource>>,
}

impl FontManager {
    pub fn new() -> Self {
        let mut sources : HashMap<FontSourceType, Box<dyn FontSource>> = HashMap::new();

        sources.insert(FontSourceType::Fontkit, Box::new(FontKitSource::new()));
        sources.insert(FontSourceType::Fontique, Box::new(FontiqueSource::new()));
        sources.insert(FontSourceType::Parley, Box::new(ParleySource::new()));
        sources.insert(FontSourceType::Pango, Box::new(PangoSource::new()));

        Self {
            sources,
        }
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
        match self.sources.get(&source_type) {
            Some(source) => source.find(families, style).ok(),
            None => {
                error!(target: LOG_TARGET, "Unknown font source: {:?}", source_type);
                None
            }
        }
    }
}

impl FontManager {
    pub fn fontique_load_font(&self, _font_info: &FontInfo) -> Result<fontique::FontInfo, anyhow::Error> {
        let Some(_source) = self.sources.get(&FontSourceType::Fontique) else {
            return Err(anyhow!("Fontique source not found"))
        };

        let _a = 1;

        Err(anyhow!("Not implemented"))
    }

    pub fn fontkit_load_font(&self, _font_info: &FontInfo) -> Result<freetype::Face, anyhow::Error> {
        let Some(_source) = self.sources.get(&FontSourceType::Fontkit) else {
            return Err(anyhow!("Fontique source not found"))
        };

        Err(anyhow!("Not implemented"))
    }

    pub fn parley_load_font(&self, font_info: &FontInfo) -> Result<parley::FontStack, anyhow::Error> {
        let Some(source) = self.sources.get(&FontSourceType::Parley) else {
            return Err(anyhow!("Parley source not found"))
        };

        let ps = source.as_any().downcast_ref::<ParleySource>()
            .ok_or_else(|| anyhow!("Failed to downcast ParleySource"))?;

        ps.context().load_font(font_info)
    }

    pub fn pango_load_font(&self, _font_info: &FontInfo) -> Result<pangocairo::Font, anyhow::Error> {
        let Some(_source) = self.sources.get(&FontSourceType::Pango) else {
            return Err(anyhow!("Pango source not found"))
        };

        Err(anyhow!("Not implemented"))
    }

    pub fn parley_context(&self) -> Result<&parley::FontContext, anyhow::Error> {
        let Some(source) = self.sources.get(&FontSourceType::Parley) else {
            return Err(anyhow!("Parley source not found"))
        };

        let ps = source.as_any().downcast_ref::<ParleySource>()
            .ok_or_else(|| anyhow!("Failed to downcast ParleySource"))?;

        Ok(ps.context())
    }

    // pub fn pango_context(&self) {
    //     let Some(_source) = self.sources.get(&FontSourceType::Pango) else {
    //         return Err(anyhow!("Pango source not found"))
    //     };
    //
    //     _source.context();
    //
    // }
    //
    // pub fn pango_stack(&self) {
    //     let Some(_source) = self.sources.get(&FontSourceType::Pango) else {
    //         return Err(anyhow!("Pango source not found"))
    //     };
    //
    //     _source.context()
    // }
}
