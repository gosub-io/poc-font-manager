use crate::font_manager::font_info::FontInfo;
use std::any::Any;
use std::path::{Path, PathBuf};

#[cfg(feature = "source_fontkit")]
pub mod fontkit;
#[cfg(feature = "source_fontique")]
pub mod fontique;
#[cfg(feature = "source_parley")]
pub mod parley;
#[cfg(feature = "source_pango")]
pub mod pango;

pub trait FontSource: AsAny {
    fn new() -> Self where Self: Sized;
    fn available_fonts(&self) -> &[FontInfo];
}

pub trait AsAny {
    #[allow(unused)]
    fn as_any(&self) -> &dyn Any;
}

impl<T: FontSource + Any> AsAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Copy, Hash, Eq, PartialEq, Debug)]
pub enum FontSourceType {
    #[cfg(feature = "source_fontkit")]
    Fontkit,
    #[cfg(feature = "source_fontique")]
    Fontique,
    #[cfg(feature = "source_parley")]
    Parley,
    #[cfg(feature = "source_pango")]
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