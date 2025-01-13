use crate::font_manager::manager::LOG_TARGET;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use anyhow::{anyhow, Error};
use font_kit::handle::Handle;
use font_kit::source::SystemSource;
use freetype::{Face, Library};
use log::{error, info};
use crate::font_manager::font_info::{FontInfo, FontStyle};
use crate::font_manager::sources::{FontSource, FontSourceType};

#[allow(unused)]
pub struct FontKitSource {
    source: SystemSource,
    ft_library: Library,
    /// Vec of all font-info structures found
    font_info: Vec<FontInfo>,
    /// List of all font handles
    handles: Vec<Handle>,
    /// Cache of font faces that are loaded through freetype
    face_cache: Arc<Mutex<HashMap<String, Face>>>,
}

impl FontSource for FontKitSource {
    fn new() -> Self {
        let library = Library::init().expect("unable to init freetype library");

        let source = SystemSource::new();
        let handles = source.all_fonts().unwrap();

        let mut seen_paths: HashSet<PathBuf> = HashSet::new();

        let mut font_info = Vec::new();
        for handle in &handles {
            match handle_to_info(&mut seen_paths, handle) {
                Ok(info) => font_info.push(info),
                Err(_) => {}
            }
        }

        info!("Loaded {} fonts from fontkit.", font_info.len());

        Self {
            source,
            ft_library: library,
            font_info,
            handles,
            face_cache: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    fn available_fonts(&self) -> &[FontInfo] {
        &self.font_info
    }

    fn find(&self, _family: &[&str], _style: FontStyle) -> Result<FontInfo, Error> {
        todo!()
    }
}

/// Converts a font handle to a gosub font info structure
fn handle_to_info(seen_paths: &mut HashSet<PathBuf>, handle: &Handle) -> Result<FontInfo, anyhow::Error> {
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

    // Check if the path is symlinked
    let resolved_path = resolve_symlink(path.to_path_buf());
    if seen_paths.contains(&resolved_path) {
        return Err(anyhow!("Path already seen"));
    }
    seen_paths.insert(resolved_path.clone());

    Ok(FontInfo {
        source_type: FontSourceType::Fontkit,
        family,
        style,
        weight: props.weight.0,
        stretch: props.stretch.0,
        monospaced: font.is_monospace(),
        path: Some(resolved_path.clone()),
        index: Some(*font_index as i32),
    })
}
