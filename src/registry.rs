use crate::{extract_value, warn, RefMapping, TableHandle};
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
    pub fn from_handles<'t, I: IntoIterator<Item = TableHandle<'t>>>(
        handles: I,
    ) -> Registry<T::Identity<'t>> {
        let name_key = format!("axbind_{}", T::ITEM_TYPE);
        let registry: HashMap<String, T::Identity<'t>> =
            HashMap::from_iter(handles.into_iter().filter_map(|handle| {
                warn!(
                    extract_value!(String, handle.get(name_key.as_str())).with_context(|| format!(
                        "No '{}' key found in file '{}'. (file skipped)",
                        name_key, handle.context
                    ))
                )
                .ok()
                .map(|name| (name.to_owned(), T::construct_unverified(handle)))
            }));
        Registry { registry }
    }
    //awkward tbh
    pub fn verify_get<S: AsRef<str>>(&mut self, key: S) -> Result<Option<&T>, Error> {
        //this get_mut throws an error if the 'registry' HashMap has key type &String??
        //forced to make keys owned Strings
        //this didnt happen in v1?
        match self.registry.get_mut(key.as_ref()) {
            Some(v) => Ok(Some(v.verify().with_context(|| {
                format!("Error interpreting {} '{}'", T::ITEM_TYPE, key.as_ref())
            })?)),
            None => Ok(None),
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
            bindings: RefMapping::new(),
        }
    }
    fn verify(&mut self) -> Result<&mut Self, Error> {
        if self.is_verified() {
            return Ok(self);
        }
        let bind_table = extract_value!(Table, self.handle.get("bindings"))?;
        self.bindings = RefMapping::with_capacity(bind_table.table.len());
        for (key, bind_handle) in bind_table {
            self.bindings
                .insert(key, extract_value!(String, bind_handle)?);
        }
        Ok(self)
    }

    fn is_verified(&self) -> bool {
        self.verified
    }
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
        if self.is_verified() {
            return Ok(self);
        }
        self.shell = extract_value!(String, self.handle.get("shell"))?;
        self.command = extract_value!(String, self.handle.get("command"))?;
        Ok(self)
    }

    fn is_verified(&self) -> bool {
        self.verified
    }
}
impl BindFunction<'_> {
    pub fn apply(
        &self,
        key: &str,
        meta_options: &crate::config::MetaOptions,
    ) -> Result<String, Error> {
        use crate::escaped_manip;
        use std::process::Command;
        let command = escaped_manip(self.command, meta_options.escape_char.unwrap(), |text| {
            text.replace(meta_options.wildcard_char.unwrap(), key)
        });
        Ok(std::str::from_utf8(
            Command::new(self.shell)
                //awkward assumption of '-c'; dont want to make user specify this for every function(?)
                .arg("-c")
                .arg(&command)
                .output()?
                .stdout
                .as_slice(),
        )?
        .to_owned())
    }
}
