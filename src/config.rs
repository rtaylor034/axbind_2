use crate:: {
    extract_value,
    TableResultOptional,
    TableGetError,
    TableHandle,
    PotentialValueHandle,
    TableResult,
    PathBuf,
    Path,
    Error,
    Context,
};
#[derive(Clone, Debug)]
pub struct MasterConfig<'t> {
    pub map_directory: &'t String,
    pub function_directory: &'t String,
    pub tag_directory: &'t String,
    pub group_options: GroupOptions<'t>,
    pub layer_options: LayerOptions<'t>,
    pub meta_options: MetaOptions<'t>,
}
#[derive(Clone, Debug)]
pub struct GroupOptions<'t> {
    pub axbind_file_format: Option<&'t String>,
}
#[derive(Clone, Debug)]
pub struct LayerOptions<'t> {
    pub escape_char: Option<char>,
    pub key_format: Option<&'t String>,
}
#[derive(Clone, Debug)]
pub struct MetaOptions<'t> {
    pub escape_char: Option<char>,
    pub wildcard_char: Option<char>,
    _p: core::marker::PhantomData<&'t str>,
}
#[derive(Error, Debug)]
pub enum ConfigError {
    #[error(transparent)]
    TableGet(#[from] TableGetError),
    #[error("{0}")]
    Misc(String),
}
impl<'st> MasterConfig<'st> {
    pub fn from_table<'t>(handle: TableHandle<'t>) -> Result<MasterConfig<'t>, ConfigError> {
        Ok(MasterConfig {
            map_directory: extract_value!(String, handle.get("map_directory"))?,
            function_directory: extract_value!(String, handle.get("function_directory"))?,
            tag_directory: extract_value!(String, handle.get("tag_directory"))?,
            group_options: GroupOptions::from_table_forced(handle.traverse(&["options", "group"])?)?,
            layer_options: LayerOptions::from_table_forced(handle.traverse(&["options", "layer"])?)?,
            meta_options: MetaOptions::from_table_forced(handle.traverse(&["options", "meta"])?)?,
        })
    }
}
impl<'st> GroupOptions<'st> {
    pub fn from_table_forced<'t>(handle: TableHandle<'t>) -> Result<GroupOptions<'t>, ConfigError> {
        Ok(GroupOptions {
            axbind_file_format: Some(extract_value!(String, handle.get("axbind_file_format"))?),
        })
    }
}
impl<'st> LayerOptions<'st> {
    pub fn from_table_forced<'t>(handle: TableHandle<'t>) -> Result<LayerOptions<'t>, ConfigError> {
        Ok(LayerOptions {
            escape_char: Some(extract_char(handle.get("escape_char"))?),
            key_format: Some(extract_value!(String, handle.get("key_format"))?),
        })
    }
}
impl<'st> MetaOptions<'st> {
    pub fn from_table_forced<'t>(handle: TableHandle<'t>) -> Result<MetaOptions<'t>, ConfigError> {
        Ok(MetaOptions {
            escape_char: Some(extract_char(handle.get("escape_char"))?),
            wildcard_char: Some(extract_char(handle.get("wildcard_char"))?),
            _p: core::marker::PhantomData,
        })
    }
}
fn extract_char(handle: PotentialValueHandle) -> Result<char, ConfigError> {
    let raw = extract_value!(String, handle.clone())?.as_str();
    match raw.len() == 1 {
        true => Ok(raw.chars().next().unwrap()),
        false => Err(ConfigError::Misc(format!(
            "value for '{}' must be exactly 1 character",
            handle.context
        ))),
    }
}
