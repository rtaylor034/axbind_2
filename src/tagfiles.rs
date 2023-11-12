use crate::{
    TableHandle,
    extract_value,
    TableResultOptional,
    config,
};
const ENTRYPOINT_PATH: &'static str = "main.toml";
pub struct TagSpecification<'t> {
    pub group_paths: Vec<&'t String>,
}
pub struct TagGroup<'t> {
    pub files: Vec<&'t String>,
    pub layers: Vec<TagLayer<'t>>,
    pub options: config::GroupOptions<'t>,
}
pub struct TagLayer<'t> {
    pub map: &'t String,
    pub remaps: Vec<&'t String>,
    pub functions: Vec<&'t String>,
    pub options: config::LayerOptions<'t>,
}
