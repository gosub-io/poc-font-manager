use std::collections::HashSet;
use std::path::{Path, PathBuf};
use log::info;
use anyhow::Error;
use fontique::{Collection, CollectionOptions, SourceKind};
use crate::font_manager::font_info::{FontInfo, FontStyle};
use crate::font_manager::sources::{FontSource, FontSourceType};

#[allow(unused)]
pub struct FontiqueSource {
    collection: Collection,
    font_info: Vec<FontInfo>
}

impl FontSource for FontiqueSource {
    fn new() -> Self {
        let mut font_info = Vec::new();
        let mut coll = Collection::new(CollectionOptions::default());

        let mut seen_paths: HashSet<PathBuf> = HashSet::new();

        let names: Vec<String> = coll.family_names().map(|n| n.to_string()).collect();
        for name in names {
            // println!("Family: {}", name);
            if let Some(family) = coll.family_by_name(&name) {
                // println!("  - Family: {}", name);
                for font in family.fonts() {
                    // println!("    - Font: {:?} {:?}", font.source().kind, font.source().id);
                    // println!(" Weight: {}", font.weight());
                    let style = match font.style() {
                        fontique::Style::Normal => FontStyle::Normal,
                        fontique::Style::Oblique(_) => FontStyle::Oblique,
                        fontique::Style::Italic => FontStyle::Italic,
                    };

                    let stretch: f32 = font.stretch().ratio();
                    let weight: f32 = font.weight().value();

                    let path = match &font.source().kind {
                        SourceKind::Path(path) => {
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
                        source_type: FontSourceType::Fontique,
                    });
                }
            }
        }

        info!("Loaded {} fonts from fontique.", font_info.len());

        Self {
            collection: coll,
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

fn resolve_symlink(path: PathBuf) -> PathBuf {
    let mut resolved_path = path.clone();

    loop {
        match std::fs::read_link(&resolved_path) {
            Ok(target) => {
                resolved_path = if target.is_relative() {
                    path.parent().unwrap_or(Path::new("/")).join(target)
                } else {
                    target
                };
            },
            Err(_) => break,
        }
    }

    resolved_path
}