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
    fn verify(&mut self) -> Result<(), Error>;
    fn is_verified(&self) -> bool;
}
pub struct Registry<'t, T: RegistryItem> {
    registry: HashMap<&'t String, T>,
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
impl<'st, T: RegistryItem> Registry<'st, T> {
    pub fn from_handles<'t, I: IntoIterator<Item = TableHandle<'t>>>(handles: I) -> Registry<'t, T::Identity<'t>> {
        let name_key = format!("axbind_{}", T::ITEM_TYPE);
        let registry: HashMap<&String, T::Identity<'t>> = HashMap::from_iter(
            handles.into_iter()
            .filter_map(|handle| 
                warn!(extract_value!(String, handle.get(name_key.as_str()))
                    .with_context(|| format!("No '{}' key found in file '{}'. (file skipped)", name_key, handle.context))).ok()
                    .map(|name| (name, T::construct_unverified(handle)))));
        Registry {
            registry,
        }
    }
    pub fn get(key: &str) -> Result<&T::Identity<'_>, Error> {
        todo!();
    }
}
impl<'st> RegistryItem for BindMap<'st> {
    const ITEM_TYPE: &'static str = "map";
    type Identity<'s> = BindMap<'s>;
    fn construct_unverified<'t>(handle: TableHandle<'t>) -> Self::Identity<'t> {
        BindMap {
            verified: false,
            handle: handle.clone(),
            bindings: RefMapping::with_capacity(handle.table.len())
        }
    }
    fn verify(&mut self) -> Result<(), Error> {
        todo!()
    }

    fn is_verified(&self) -> bool { self.verified }
}
impl<'st> RegistryItem for BindFunction<'st> {
    const ITEM_TYPE: &'static str = "function";
    type Identity<'s> = BindFunction<'s>;
    fn construct_unverified<'t>(handle: TableHandle<'t>) -> Self::Identity<'t> {
        BindFunction {
            verified: false,
            handle,
            shell: "",
            command: "",
        }
    }
    fn verify(&mut self) -> Result<(), Error> {
        todo!()
    }

    fn is_verified(&self) -> bool { self.verified }
}
