use crate::font_manager::font_info::{FontInfo, FontStyle};
use crate::font_manager::sources::{FontSource, FontSourceType};
#[cfg(feature = "source_fontkit")]
use crate::font_manager::sources::fontkit::FontKitSource;
#[cfg(feature = "source_pango")]
use crate::font_manager::sources::pango::PangoSource;
#[cfg(feature = "source_parley")]
use crate::font_manager::sources::parley::ParleySource;

#[allow(dead_code)]
pub const LOG_TARGET: &str = "font-manager";

pub struct FontManager {
    #[cfg(feature = "source_fontkit")]
    fontkit: FontKitSource,
    #[cfg(feature = "source_pango")]
    pango: PangoSource,
    #[cfg(feature = "source_parley")]
    parley: ParleySource,
}

impl FontManager {
    pub fn new() -> Self {
        Self {
            #[cfg(feature = "source_fontkit")]
            fontkit: FontKitSource::new(),
            #[cfg(feature = "source_parley")]
            parley: ParleySource::new(),
            #[cfg(feature = "source_pango")]
            pango: PangoSource::new(),
        }
    }

    // Returns all compiled in font sources
    pub fn sources(&self) -> Vec<FontSourceType> {
        let mut v = vec![];

        #[cfg(feature = "source_fontkit")]
        v.push(FontSourceType::Fontkit);
        #[cfg(feature = "source_pango")]
        v.push(FontSourceType::Pango);
        #[cfg(feature = "source_parley")]
        v.push(FontSourceType::Parley);

        v
    }

    /// Returns all available fonts for given source-type
    pub fn available_fonts(&self, source_type: FontSourceType) -> Vec<FontInfo> {
        let mut fonts = match source_type {
            #[cfg(feature = "source_fontkit")]
            FontSourceType::Fontkit => self.fontkit.available_fonts().to_vec(),
            #[cfg(feature = "source_pango")]
            FontSourceType::Pango => self.pango.available_fonts().to_vec(),
            #[cfg(feature = "source_parley")]
            FontSourceType::Parley => self.parley.available_fonts().to_vec(),
        };

        fonts.sort_by_key(|fi| fi.family.clone());
        fonts
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
    #[cfg(feature = "source_fontkit")]
    pub fn find_fontkit(&self) -> &FontKitSource {
        &self.fontkit
    }

    #[cfg(feature = "source_pango")]
    pub fn find_pango(&self) -> &PangoSource {
        &self.pango
    }

    #[cfg(feature = "source_parley")]
    pub fn find_parley(&self) -> &ParleySource {
        &self.parley
    }

    // #[cfg(feature = "source_fontkit")]
    // pub fn fontkit_load_freetype_font(&self, font_info: &FontInfo) -> Result<freetype::Face, anyhow::Error> {
    //     let Some(source) = self.sources.get(&FontSourceType::Fontkit) else {
    //         return Err(anyhow!("FontKit source not found"))
    //     };
    //
    //     let ps = source.as_any().downcast_ref::<FontKitSource>()
    //         .ok_or_else(|| anyhow!("Failed to downcast FontKitSource"))?;
    //
    //     ps.load_freetype_font(font_info)
    // }
    //
    // #[cfg(feature = "source_pango")]
    // pub fn pango_load_font(&self, font_info: &FontInfo) -> Result<pangocairo::pango::Font, anyhow::Error> {
    //     let Some(source) = self.sources.get(&FontSourceType::Pango) else {
    //         return Err(anyhow!("Pango source not found"))
    //     };
    //
    //     let ps = source.as_any().downcast_ref::<PangoSource>()
    //         .ok_or_else(|| anyhow!("Failed to downcast PangoSource"))?;
    //
    //     ps.load_font(font_info)
    // }
    //
    // #[cfg(feature = "source_pango")]
    // pub fn pango_get_description(&self, font_info: &FontInfo, size: f64) -> Result<pangocairo::pango::FontDescription, anyhow::Error> {
    //     let Some(source) = self.sources.get(&FontSourceType::Pango) else {
    //         return Err(anyhow!("Pango source not found"))
    //     };
    //
    //     let ps = source.as_any().downcast_ref::<PangoSource>()
    //         .ok_or_else(|| anyhow!("Failed to downcast PangoSource"))?;
    //
    //     Ok(ps.get_description(font_info, size))
    // }
    //
    // #[cfg(feature = "source_parley")]
    // pub fn parley_context(&self) -> Result<Arc<RefCell<parley::FontContext>>, anyhow::Error> {
    //     let Some(source) = self.sources.get(&FontSourceType::Parley) else {
    //         return Err(anyhow!("Parley source not found"))
    //     };
    //
    //     let ps = source.as_any().downcast_ref::<ParleySource>()
    //         .ok_or_else(|| anyhow!("Failed to downcast ParleySource"))?;
    //
    //     Ok(ps.context())
    // }
    //
    // #[cfg(feature = "source_parley")]
    // pub fn parley_get_font_stack(&self, font_info: &FontInfo) -> Result<parley::FontStack, anyhow::Error> {
    //     let Some(source) = self.sources.get(&FontSourceType::Parley) else {
    //         return Err(anyhow!("Parley source not found"))
    //     };
    //
    //     let ps = source.as_any().downcast_ref::<ParleySource>()
    //         .ok_or_else(|| anyhow!("Failed to downcast ParleySource"))?;
    //
    //     Ok(ps.get_font_stack(font_info.family.clone()))
    // }
}