use crate::{
    TableHandle,
    extract_value,
    TableResultOptional,
    PotentialValueHandle,
    TableResult,
    config,
    Error,
    registry::*,
    OwnedMapping,
    escaped_manip,
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
    //kinda silly that mutable references are required, but calls verify()
    pub fn generate_bindings<'r>(
        &self,
        map_registry: &mut Registry<BindMap<'r>>,
        function_registry: &mut Registry<BindFunction>,
        meta_options: &config::MetaOptions,
        base_layer_options: &config::LayerOptions,
    ) -> Result<(Vec<String>, Vec<String>), Error> {
        macro_rules! get { ($($args:expr),*) => { TagLayer::reg_get($($args,)*) } }
        let (reference_keys, mut reference_values): (Vec<_>, Vec<_>) = get!(map_registry, self.map)?.bindings.clone().into_iter()
            .unzip();
        //can be done with pure inline map functions but done this way for readiblity (and also becuase
        //harrddd)
        for reference_value in reference_values.iter_mut() {
            for remap_name in &self.remaps {
                if let Some(remapped_value) = get!(map_registry, remap_name)?.bindings.get(reference_value) {
                    *reference_value = *remapped_value;
                }
            }
        }
        let mut o_values: Vec<String> = reference_values.into_iter().map(|v| v.to_owned()).collect();
        for o_value in o_values.iter_mut() {
            for function_name in &self.functions {
                *o_value = get!(function_registry, function_name)?.apply(o_value, meta_options)
                    .with_context(|| format!("Error applying function '{}' to value '{}'", function_name, o_value))?;
            }
        }
        use optwrite::OptWrite;
        let layer_options = base_layer_options.clone().overriden_by(self.options.clone());
        //should really make a function/macro for just replacing wildcardchar.
        let o_keys: Vec<String> = reference_keys.into_iter()
            .map(|key| escaped_manip(layer_options.key_format.unwrap(), meta_options.escape_char.unwrap(), |key_format|
                key_format.replace(meta_options.wildcard_char.unwrap(), key)))
            .collect();

        Ok((o_keys, o_values))
    }
    fn reg_get<S: AsRef<str>, T: RegistryItem>(registry: &mut Registry<T>, key: S) -> Result<&T, Error> {
        registry.verify_get(&key)?.with_context(|| format!("No {} with name '{}' could be found.",
            T::ITEM_TYPE, key.as_ref()))
    }
}
fn extract_array_strings<'t>(handle: PotentialValueHandle<'t>) -> TableResult<Vec<&'t String>> {
    extract_value!(Array, handle)?
        .into_iter()
        .map(|v| extract_value!(String, v))
        .collect()
}
