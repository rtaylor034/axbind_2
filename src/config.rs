use crate:: {
    extract_value,
    TableResultOptional,
    TableGetError,
    TableHandle,
    TableResult,
    PathBuf,
    Path,
};

pub struct MasterConfig<'t> {
    pub map_directory: &'t String,
    pub function_directory: &'t String,
    pub tag_directory: &'t String,
    pub group_options: GroupOptions<'t>,
    pub layer_options: LayerOptions<'t>,
    pub meta_options: MetaOptions<'t>,
}

pub struct GroupOptions<'t> {
    pub axbind_file_format: Option<&'t String>,
}
pub struct LayerOptions<'t> {
    pub escape_char: Option<char>,
    pub key_format: Option<&'t String>,
}
pub struct MetaOptions<'t> {
    pub escape_char: Option<char>,
    pub wildcard_char: Option<char>,
    _p: core::marker::PhantomData<&'t str>,
}
