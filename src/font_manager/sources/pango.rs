use anyhow::Error;
use log::info;
use pangocairo::pango::{Context, FontMap};
use pangocairo::prelude::{FontFaceExt, FontFamilyExt, FontMapExt};
use crate::font_manager::font_info::{FontInfo, FontStyle};
use crate::font_manager::sources::{FontSource, FontSourceType};

#[allow(unused)]
pub struct PangoSource {
    font_map: FontMap,
    context: Context,
    font_info: Vec<FontInfo>
}

impl FontSource for PangoSource {
    fn new() -> Self {
        let font_map = pangocairo::FontMap::new();
        let context = font_map.create_context();

        let mut font_info = vec![];
        for family in context.list_families() {

            for face in family.list_faces() {
                let desc = face.describe();

                let style = match desc.style() {
                    pangocairo::pango::Style::Normal => FontStyle::Normal,
                    pangocairo::pango::Style::Oblique => FontStyle::Oblique,
                    pangocairo::pango::Style::Italic => FontStyle::Italic,
                    _ => FontStyle::Normal,
                };

                // Conversion taken from: https://developer.mozilla.org/en-US/docs/Web/CSS/font-stretch
                let stretch = match desc.stretch() {
                    pangocairo::pango::Stretch::UltraCondensed => 0.5,
                    pangocairo::pango::Stretch::ExtraCondensed => 0.625,
                    pangocairo::pango::Stretch::Condensed => 0.75,
                    pangocairo::pango::Stretch::SemiCondensed => 87.5,
                    pangocairo::pango::Stretch::Normal => 1.0,
                    pangocairo::pango::Stretch::SemiExpanded => 1.125,
                    pangocairo::pango::Stretch::Expanded => 1.25,
                    pangocairo::pango::Stretch::ExtraExpanded => 1.5,
                    pangocairo::pango::Stretch::UltraExpanded => 2.0,
                    _ => 1.0,
                };

                let weight = match desc.weight() {
                    pangocairo::pango::Weight::Thin => 100,
                    pangocairo::pango::Weight::Ultralight => 200,
                    pangocairo::pango::Weight::Semilight => 250,
                    pangocairo::pango::Weight::Light => 300,
                    pangocairo::pango::Weight::Book => 350,
                    pangocairo::pango::Weight::Normal => 400,
                    pangocairo::pango::Weight::Medium => 500,
                    pangocairo::pango::Weight::Semibold => 600,
                    pangocairo::pango::Weight::Bold => 700,
                    pangocairo::pango::Weight::Ultrabold => 800,
                    pangocairo::pango::Weight::Heavy => 900,
                    _ => 400,
                };

                font_info.push(FontInfo {
                    family: family.name().to_string(),
                    style,
                    weight: weight as f32,
                    stretch,
                    monospaced: family.is_monospace(),
                    source_type: FontSourceType::Pango,
                    path: None,
                    index: None,
                })
            }
        }

        info!("Loaded {} fonts from pango.", font_info.len());

        Self {
            font_map,
            context,
            font_info,
        }
    }

    fn available_fonts(&self) -> &[FontInfo] {
        &self.font_info
    }

    fn find(&self, _family: &[&str], _style: FontStyle) -> Result<FontInfo, Error> {
        todo!()
    }
}