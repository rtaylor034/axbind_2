use crate::{
    TableHandle,
    extract_value,
    TableResultOptional,
    PotentialValueHandle,
    TableResult,
    config,
    Error,
};
use anyhow::Context;
pub const ENTRYPOINT_FILE: &'static str = "main.toml";
pub struct TagSpecification<'t> {
    //silly field
    pub group_paths: Vec<&'t str>,
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
impl<'st> TagSpecification<'st> {
    pub fn from_table<'t>(handle: TableHandle<'t>) -> Result<TagSpecification<'t>, Error> {
        //silly expression
        Ok(TagSpecification {
            group_paths: extract_array_strings(handle.get("groups")).optional()?
                .map(|o| o.iter().map(|v| v.as_str()).collect())
                .unwrap_or(vec![ENTRYPOINT_FILE]),
        })
    }
}
impl<'st> TagGroup<'st> {
    pub fn from_table<'t>(handle: TableHandle<'t>) -> Result<TagGroup<'t>, Error> {
        use config::GroupOptions;
        //insane
        Ok(TagGroup {
            files: extract_array_strings(handle.get("files"))?,
            layers: extract_value!(Array, handle.get("layers"))?
                .into_iter()
                .map(|t| TagLayer::from_table(extract_value!(Table, t)?))
                .collect::<Result<Vec<TagLayer>, Error>>()
                .context("Error while interpreting binding layer.")?,
            options: extract_value!(Table, handle.get("options")).optional()?
                .map_or(Ok(GroupOptions::default()), |opt_table| GroupOptions::from_table(opt_table))?
        })
    }
}
impl<'st> TagLayer<'st> {
    pub fn from_table<'t>(handle: TableHandle<'t>) -> Result<TagLayer<'t>, Error> {
        use config::LayerOptions;
        Ok(TagLayer {
            map: extract_value!(String, handle.get("map"))?,
            remaps: extract_array_strings(handle.get("remaps")).optional()?.unwrap_or(Vec::new()),
            functions: extract_array_strings(handle.get("functions")).optional()?.unwrap_or(Vec::new()),
            options: extract_value!(Table, handle.get("options")).optional()?
                .map_or(Ok(LayerOptions::default()), |opt_table| LayerOptions::from_table(opt_table))?
        })
    }
}
fn extract_array_strings<'t>(handle: PotentialValueHandle<'t>) -> TableResult<Vec<&'t String>> {
    extract_value!(Array, handle)?
        .into_iter()
        .map(|v| extract_value!(String, v))
        .collect()
}
