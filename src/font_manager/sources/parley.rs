use std::borrow::Cow;
use std::collections::HashSet;
use std::path::PathBuf;
use log::info;
use std::cell::RefCell;
use std::sync::Arc;
use crate::font_manager::font_info::{FontInfo, FontStyle};
use crate::font_manager::sources::{resolve_symlink, FontSource, FontSourceType};

// #[allow(unused)]
pub struct ParleySource {
    context: Arc<RefCell<parley::FontContext>>,
    font_info: Vec<FontInfo>
}

impl FontSource for ParleySource {
    fn new() -> Self {
        let mut context = parley::FontContext::new();
        let mut font_info = Vec::new();
        let coll = &mut context.collection;

        let mut seen_paths: HashSet<PathBuf> = HashSet::new();

        let names: Vec<String> = coll.family_names().map(|n| n.to_string()).collect();
        for name in names {
            if let Some(family) = coll.family_by_name(&name) {
                for font in family.fonts() {
                    let style = match font.style() {
                        parley::FontStyle::Normal => FontStyle::Normal,
                        parley::FontStyle::Oblique(_) => FontStyle::Oblique,
                        parley::FontStyle::Italic => FontStyle::Italic,
                    };

                    let stretch: f32 = font.stretch().ratio();
                    let weight: f32 = font.weight().value();

                    let path = match &font.source().kind {
                        parley::fontique::SourceKind::Path(path) => {
                            // Check if the path is symlinked
                            let resolved_path = resolve_symlink(path.to_path_buf());
                            if seen_paths.contains(&resolved_path) {
                                continue;
                            }
                            seen_paths.insert(resolved_path.clone());
                            Some(resolved_path)
                        },
                        _ => None,
                    };

                    font_info.push(FontInfo {
                        family: name.to_string(),
                        style,
                        weight,
                        stretch,
                        monospaced: false,  // We just don't know
                        path,
                        index: None,
                        source_type: FontSourceType::Parley,
                    });


                }
            }
        }

        info!("Loaded {} fonts from parley.", font_info.len());

        Self {
            context: Arc::new(RefCell::new(context)),
            font_info,
        }
    }

    fn available_fonts(&self) -> &[FontInfo] {
        &self.font_info
    }
}

impl ParleySource {
    pub fn context(&self) -> Arc<RefCell<parley::FontContext>> {
        self.context.clone()
    }

    pub fn get_font_stack(&self, family: String) -> parley::FontStack {
        parley::FontStack::Single(
            parley::style::FontFamily::Named(Cow::Owned(family))
        )
    }
}