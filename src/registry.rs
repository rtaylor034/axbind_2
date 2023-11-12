use crate:: {
    TableRoot,
    TableHandle,
    Path,
    PathBuf,
    RefMapping,
    extract_value,
    warn_continue,
    warn
};
use anyhow::{Context, Error};
use std::collections::HashMap;
pub trait RegistryItem { 
    const ITEM_TYPE: &'static str;
    type Identity<'s>: RegistryItem;
    fn construct_unverified<'t>(handle: TableHandle<'t>) -> Self::Identity<'t>;
    fn verify() -> Result<(), Error>;
    fn is_verified() -> bool;
}
pub struct Registry<T: RegistryItem> {
    registry: HashMap<String, T>,
    roots: Vec<TableRoot>,
}
pub struct BindMap<'t> {
    verified: bool,
    handle: TableHandle<'t>,
    pub bindings: RefMapping<'t>,
}
pub struct BindFunction<'t> {
    verified: bool,
    handle: TableHandle<'t>,
    shell: &'t str,
    command: &'t str,
}
impl<T: RegistryItem> Registry<T> {
    pub fn from_handles<'t, I: IntoIterator<Item = TableHandle<'t>>>(handles: I) -> Result<Registry<T::Identity<'t>>, Error> {
        todo!()
    }
    pub fn get(key: &str) -> Result<T, Error> {
        todo!();
    }
}
impl<'st> RegistryItem for BindMap<'st> {
    const ITEM_TYPE: &'static str = "map";
    type Identity<'s> = BindMap<'st>;
    fn construct_unverified<'t>(handle: TableHandle<'t>) -> Self::Identity<'t> {
        todo!()
    }
    fn verify() -> Result<(), Error> {
        todo!()
    }

    fn is_verified() -> bool {
        todo!()
    }
}
impl<'st> RegistryItem for BindFunction<'st> {
    const ITEM_TYPE: &'static str = "function";
    type Identity<'s> = BindFunction<'st>;
    fn construct_unverified<'t>(handle: TableHandle<'t>) -> Self::Identity<'t> {
        todo!()
    }
    fn verify() -> Result<(), Error> {
        todo!()
    }

    fn is_verified() -> bool {
        todo!()
    }
}
