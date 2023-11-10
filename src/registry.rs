use crate:: {
    TableHandle,
    Path,
};
use std::collections::HashMap;
pub trait LazyVerify { 
    type Identity<'s>;
    fn construct_unverified<'t>(handle: TableHandle<'t>) -> Self::Identity<'t>;
    fn verify() -> Result<(), RegistryError>;
    fn is_verified() -> bool;
}
pub struct Registry<T: LazyVerify> {
    registry: HashMap<String, T>,
}
pub struct BindMap<'t> {
    verified: bool,
    handle: TableHandle<'t>,
}
pub struct BindFunction<'t> {
    verified: bool,
    handle: TableHandle<'t>,
}
pub enum RegistryError {
}
impl<T: LazyVerify> Registry<T> {
    pub fn from_directory<P: AsRef<Path>>(path: P) -> Result<Registry<T>, RegistryError> {
        todo!();
    }
    pub fn get(key: &str) -> Result<T, RegistryError> {
        todo!();
    }
}
impl<'st> LazyVerify for BindMap<'st> {
    type Identity<'s> = BindMap<'st>;
    fn construct_unverified<'t>(handle: TableHandle<'t>) -> Self::Identity<'t> {
        todo!()
    }
    fn verify() -> Result<(), RegistryError> {
        todo!()
    }

    fn is_verified() -> bool {
        todo!()
    }
}
