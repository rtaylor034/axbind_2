use crate::{
    config, escaped_manip, extract_value, registry::*, Error, PotentialValueHandle, TableHandle,
    TableResult, TableResultOptional, extract_array_strings,
};
use anyhow::Context;
pub struct TagSpecification<'t> {
    //silly field
    pub group_paths: Option<Vec<&'t str>>,
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
            group_paths: extract_array_strings(handle.get("groups"))
                .optional()?
                .map(|o| o.iter().map(|v| v.as_str()).collect()),
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
            options: extract_value!(Table, handle.get("options"))
                .optional()?
                .map_or(Ok(GroupOptions::default()), |opt_table| {
                    GroupOptions::from_table(opt_table)
                })?,
        })
    }
}
impl<'st> TagLayer<'st> {
    pub fn from_table<'t>(handle: TableHandle<'t>) -> Result<TagLayer<'t>, Error> {
        use config::LayerOptions;
        Ok(TagLayer {
            map: extract_value!(String, handle.get("map"))?,
            remaps: extract_array_strings(handle.get("remaps"))
                .optional()?
                .unwrap_or(Vec::new()),
            functions: extract_array_strings(handle.get("functions"))
                .optional()?
                .unwrap_or(Vec::new()),
            options: extract_value!(Table, handle.get("options"))
                .optional()?
                .map_or(Ok(LayerOptions::default()), |opt_table| {
                    LayerOptions::from_table(opt_table)
                })?,
        })
    }
    //kinda silly that mutable references are required, but calls verify()
    pub fn generate_bindings<'r>(
        &self,
        map_registry: &mut Registry<BindMap<'r>>,
        function_registry: &mut Registry<BindFunction>,
        meta_options: &config::MetaOptions,
        base_key_format: &str,
    ) -> Result<(Vec<String>, Vec<String>), Error> {
        macro_rules! get { ($($args:expr),*) => { TagLayer::reg_get($($args,)*) } }
        let (reference_keys, mut reference_values): (Vec<_>, Vec<_>) =
            get!(map_registry, self.map)?
                .bindings
                .clone()
                .into_iter()
                .unzip();
        //can be done with pure inline map functions but done this way for readiblity (and also becuase
        //harrddd)
        for reference_value in reference_values.iter_mut() {
            for remap_name in &self.remaps {
                if let Some(remapped_value) = get!(map_registry, remap_name)?
                    .bindings
                    .get(reference_value)
                {
                    *reference_value = *remapped_value;
                }
            }
        }
        let mut o_values: Vec<String> =
            reference_values.into_iter().map(|v| v.to_owned()).collect();
        for o_value in o_values.iter_mut() {
            for function_name in &self.functions {
                *o_value = get!(function_registry, function_name)?
                    .apply(o_value, meta_options)
                    .with_context(|| {
                        format!(
                            "Error applying function '{}' to value '{}'",
                            function_name, o_value
                        )
                    })?;
            }
        }
        use optwrite::OptWrite;
        let key_format = Some(base_key_format)
            .overriden_by(self.options.key_format.map(|v| v.as_str()))
            .unwrap();
        //should really make a function/macro for just replacing wildcardchar.
        let o_keys: Vec<String> = reference_keys
            .into_iter()
            .map(|key| {
                escaped_manip(key_format, meta_options.escape_sequence.unwrap(), |chunk| {
                    chunk.replace(meta_options.wildcard_char.unwrap(), key)
                })
            })
            .collect();

        Ok((o_keys, o_values))
    }
    fn reg_get<S: AsRef<str>, T: RegistryItem<Identity<'st> = T>>(
        registry: &mut Registry<T>,
        key: S,
    ) -> Result<&T, Error> {
        registry.verify_get(&key)?.with_context(|| {
            format!(
                "No {} with name '{}' could be found.",
                T::ITEM_TYPE,
                key.as_ref()
            )
        })
    }
}
