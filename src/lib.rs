use toml_context::*;

use anyhow::Error;
use optwrite::OptWrite;
pub mod args;
pub mod config;
pub mod registry;
pub mod tagfiles;
pub type RefMapping<'t> = std::collections::HashMap<&'t String, &'t String>;
pub type OwnedMapping = std::collections::HashMap<String, String>;

#[macro_export]
macro_rules! display_err {
    ($tag:expr, $error:expr) => {
        eprintln!("{}", $tag);
        let mut spaces = String::new();
        let mut cause_chain = $error.chain().into_iter();
        eprintln!(" └ {} ", cause_chain.next().unwrap());
        for cause in cause_chain {
            spaces.push_str("  ");
            eprintln!(" {}└ {}", spaces, cause);
        }
    }
}
#[macro_export]
macro_rules! warn {
    ($result:expr) => {{
        match $result {
            Ok(v) => Ok(v),
            Err(e) => {
                $crate::display_err!("[WARNING]", e);
                Err(e)
            }
        }
    }};
}
#[macro_export]
macro_rules! warn_continue {
    ($result:expr) => {{
        match warn!($result) {
            Ok(v) => v,
            Err(_) => continue,
        }
    }};
}
pub fn escaped_manip<'s, F>(text: &'s str, escape: char, manip: F) -> String
where
    F: Fn(&'s str) -> String,
{
    let mut o = String::with_capacity(text.len());
    let mut current_chunk = text;
    while let Some((mut left, right)) = current_chunk.split_once(escape) {
        o.push_str(manip(left).as_str());
        //'left' is reused to store the escaped character to add it back, un-manipulated
        (left, current_chunk) = right.split_at(1);
        o.push_str(left);
    }
    o.push_str(manip(current_chunk).as_str());
    o
}
