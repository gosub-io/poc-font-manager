use crate::font_manager::font_info::{FontInfo, FontStyle};
use std::any::Any;
use std::path::{Path, PathBuf};

pub mod fontkit;
pub mod fontique;
pub mod parley;
pub mod pango;

pub trait FontSource: AsAny {
    fn new() -> Self where Self: Sized;
    fn available_fonts(&self) -> &[FontInfo];
    fn find(&self, family: &[&str], style: FontStyle) -> Result<FontInfo, anyhow::Error>;
}

pub trait AsAny {
    fn as_any(&self) -> &dyn Any;
}

impl<T: FontSource + Any> AsAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Copy, Hash, Eq, PartialEq, Debug)]
pub enum FontSourceType {
    Fontkit,
    Fontique,
    Parley,
    Pango,
}

/// Resolves a symlinked path
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