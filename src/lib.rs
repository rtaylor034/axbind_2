use toml_context::*;
use std::path::{Path, PathBuf};
use thiserror::Error;
pub mod args;
pub mod config;
pub mod registry;
pub type RefMapping<'t> = std::collections::HashMap<&'t String, &'t String>;
