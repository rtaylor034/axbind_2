use toml_context::*;
use std::path::{Path, PathBuf};
use anyhow::Error;
pub mod args;
pub mod config;
pub mod registry;
pub mod tagfiles;
pub type RefMapping<'t> = std::collections::HashMap<&'t String, &'t String>;

#[macro_export]
macro_rules! warn {
    ($result:expr) => {
        {
            match $result {
                Ok(v) => Ok(v),
                Err(e) => {
                    eprintln!("[WARN]");
                    let mut spaces = String::new();
                    for (i, cause) in e.chain().enumerate() {
                        spaces.push(' ');
                        eprintln!(" ({}){}> {}", i, spaces, cause);
                    }
                    Err(e)
                },
            }
        }
    }
}
#[macro_export]
macro_rules! warn_continue {
    ($result:expr) => {
        {
            match warn!($result) {
                Ok(v) => v,
                Err(_) => continue,
            }
        }
    }
}
