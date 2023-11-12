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
    OptWrite,
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
#[derive(Clone, Debug, Default, OptWrite)]
pub struct GroupOptions<'t> {
    pub axbind_file_format: Option<&'t String>,
}
#[derive(Clone, Debug, Default, OptWrite)]
pub struct LayerOptions<'t> {
    pub escape_char: Option<char>,
    pub key_format: Option<&'t String>,
}
#[derive(Clone, Debug, Default, OptWrite)]
pub struct MetaOptions<'t> {
    pub escape_char: Option<char>,
    pub wildcard_char: Option<char>,
    _p: core::marker::PhantomData<&'t str>,
}
impl<'st> MasterConfig<'st> {
    pub fn from_table<'t>(handle: TableHandle<'t>) -> Result<MasterConfig<'t>, Error> {
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
    pub fn from_table_forced<'t>(handle: TableHandle<'t>) -> Result<GroupOptions<'t>, Error> {
        Ok(GroupOptions {
            axbind_file_format: Some(extract_value!(String, handle.get("axbind_file_format"))?),
        })
    }
    pub fn from_table<'t>(handle: TableHandle<'t>) -> Result<GroupOptions<'t>, Error> {
        Ok(GroupOptions {
            axbind_file_format: extract_value!(String, handle.get("axbind_file_format")).optional()?,
        })
    }
}
impl<'st> LayerOptions<'st> {
    pub fn from_table_forced<'t>(handle: TableHandle<'t>) -> Result<LayerOptions<'t>, Error> {
        Ok(LayerOptions {
            escape_char: Some(extract_char(handle.get("escape_char"))?),
            key_format: Some(extract_value!(String, handle.get("key_format"))?),
        })
    }
    pub fn from_table<'t>(handle: TableHandle<'t>) -> Result<LayerOptions<'t>, Error> {
        Ok(LayerOptions {
            escape_char: extract_char_optional(handle.get("escape_char"))?,
            key_format: extract_value!(String, handle.get("key_format")).optional()?,
        })
    }
}
impl<'st> MetaOptions<'st> {
    pub fn from_table_forced<'t>(handle: TableHandle<'t>) -> Result<MetaOptions<'t>, Error> {
        Ok(MetaOptions {
            escape_char: Some(extract_char(handle.get("escape_char"))?),
            wildcard_char: Some(extract_char(handle.get("wildcard_char"))?),
            _p: core::marker::PhantomData,
        })
    }
    pub fn from_table<'t>(handle: TableHandle<'t>) -> Result<MetaOptions<'t>, Error> {
        Ok(MetaOptions {
            escape_char: extract_char_optional(handle.get("escape_char"))?,
            wildcard_char: extract_char_optional(handle.get("wildcard_char"))?,
            _p: core::marker::PhantomData,
        })
    }
}
fn extract_char(handle: PotentialValueHandle) -> Result<char, Error> {
    let raw = extract_value!(String, handle.clone())?.as_str();
    match raw.len() == 1 {
        true => Ok(raw.chars().next().unwrap()),
        false => Err(anyhow::anyhow!("value for key '{}' must be exactly 1 character", handle.context)),
    }
}
fn extract_char_optional(handle: PotentialValueHandle) -> Result<Option<char>, Error> {
    let raw = match extract_value!(String, handle.clone()).optional()? {
        Some(v) => v.as_str(),
        None => return Ok(None),
    };
    match raw.len() == 1 {
        true => Ok(Some(raw.chars().next().unwrap())),
        false => Err(anyhow::anyhow!("value for key '{}' must be exactly 1 character", handle.context)),
    }
}
