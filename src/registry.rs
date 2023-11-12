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
//Honestly overcomplex for what its worth (just for lazy validation)
//Its not even that expensive to validate a map/function
//I dont even need too, I just like hashmaps better than treemaps becuase O oF OnE!1
use anyhow::{Context, Error};
use std::collections::HashMap;
pub trait RegistryItem { 
    const ITEM_TYPE: &'static str;
    type Identity<'s>: RegistryItem;
    fn construct_unverified<'t>(handle: TableHandle<'t>) -> Self::Identity<'t>;
    fn verify(&mut self) -> Result<&mut Self, Error>;
    fn is_verified(&self) -> bool;
}
#[derive(Debug)]
pub struct Registry<T: RegistryItem> {
    registry: HashMap<String, T>,
}
#[derive(Debug)]
pub struct BindMap<'t> {
    verified: bool,
    handle: TableHandle<'t>,
    pub bindings: RefMapping<'t>,
}
#[derive(Debug)]
pub struct BindFunction<'t> {
    verified: bool,
    handle: TableHandle<'t>,
    shell: &'t str,
    command: &'t str,
}
impl<T: RegistryItem> Registry<T> {
    pub fn from_handles<'t, I: IntoIterator<Item = TableHandle<'t>>>(handles: I) -> Registry<T::Identity<'t>> {
        let name_key = format!("axbind_{}", T::ITEM_TYPE);
        let registry: HashMap<String, T::Identity<'t>> = HashMap::from_iter(
            handles.into_iter()
            .filter_map(|handle| 
                warn!(extract_value!(String, handle.get(name_key.as_str()))
                    .with_context(|| format!("No '{}' key found in file '{}'. (file skipped)", name_key, handle.context))).ok()
                    .map(|name| (name.to_owned(), T::construct_unverified(handle)))));
        Registry {
            registry,
        }
    }
    //awkward tbh
    pub fn verify_get(&mut self, key: &str) -> Result<Option<&T>, Error> {
        match self.registry.get_mut(key) {
            Some(v) => Ok(Some(v)),
            None => Ok(None)
        }
    }
}
impl<'st> RegistryItem for BindMap<'st> {
    const ITEM_TYPE: &'static str = "map";
    type Identity<'s> = BindMap<'s>;
    fn construct_unverified<'t>(handle: TableHandle<'t>) -> Self::Identity<'t> {
        BindMap {
            verified: false,
            handle,
            bindings: RefMapping::new()
        }
    }
    fn verify(&mut self) -> Result<&mut Self, Error> {
        if self.is_verified() { return Ok(self) }
        let bind_table = extract_value!(Table, self.handle.get("bindings"))?;
        self.bindings = RefMapping::with_capacity(bind_table.table.len());
        for (key, bind_handle) in bind_table {
            self.bindings.insert(key, extract_value!(String, bind_handle)?);
        }
        Ok(self)
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
    fn verify(&mut self) -> Result<&mut Self, Error> {
        if self.is_verified() { return Ok(self) }
        self.shell = extract_value!(String, self.handle.get("shell"))?;
        self.command = extract_value!(String, self.handle.get("command"))?;
        Ok(self)
    }

    fn is_verified(&self) -> bool { self.verified }
}