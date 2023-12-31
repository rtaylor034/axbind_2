use anyhow::{Context, Error};
use axbind::*;
use toml_context::TableRoot;
use std::path::{Path, PathBuf};
fn program() -> Result<(), Error> {
    let program_args = args::ProgramArgs::from_runinfo(gfunc::run::RunInfo::get_from_env());
    let master_root = gfunc::for_until(&program_args.config_paths, |path| toml_context::TableRoot::from_file_path(path).ok())
        .with_context(|| format!("No valid config files could be found -- paths checked: {:#?}\n (check for invalid toml syntax)",
                                 program_args.config_paths))?;
    let config_parent_path = PathBuf::from(Path::new(master_root.context.branch.as_ref()).parent().unwrap());
    let master_config = config::MasterConfig::from_table(master_root.handle())
        .context("Error interpreting config file.")?;
    //eprintln!(" >> MASTER CONFIG :: {:#?}", master_config);
    macro_rules! get_registry_roots {
        ($name:expr, $path:expr) => {
            gfunc::fnav::rsearch_dir_pred(config_parent_path.join($path), |file_path| file_path.is_file())
                .with_context(|| format!("Error reading {} directory \"{}\"", $name, $path))?
                .into_iter()
                .filter_map(|path| {
                    warn!(TableRoot::from_file_path(&path).with_context(|| format!(
                        "Cannot parse file {:?} to toml (file skipped).",
                        path
                    )))
                    .ok()
                })
                .collect::<Vec<TableRoot>>()
        };
    }
    let map_roots = get_registry_roots!("map", master_config.map_directory);
    let function_roots = get_registry_roots!("function", master_config.function_directory);
    let mut map_registry = registry::Registry::<registry::BindMap>::from_handles(
        map_roots.iter().map(|root| root.handle()),
    );
    let mut function_registry = registry::Registry::<registry::BindFunction>::from_handles(
        function_roots.iter().map(|root| root.handle()),
    );
    //eprintln!(" >> MAP REGISTRY :: {:#?}", map_registry);
    //eprintln!(" >> FUNCTION REGISTRY :: {:#?}", function_registry);
    let tag_directory_paths = gfunc::fnav::rsearch_dir(
        &program_args.root_directory,
        master_config.tag_directory,
        gfunc::fnav::MetaType::Directory,
    )
    .with_context(|| {
        format!(
            "Could not read specified root directory {:?}.",
            program_args.root_directory
        )
    })?;
    //eprintln!(" >> TAG DIRECTORIES FOUND :: {:#?}", tag_directory_paths);
    macro_rules! capture_err {
        ($code:block) => {
            (|| -> Result<(), Error> { Ok($code) })()
        };
    }
    //this is certainly something.
    for tag_directory_path in &tag_directory_paths {
        warn_continue!(capture_err!( {
            let specification_root = TableRoot::from_file_path(
                tag_directory_path.join(master_config.tag_entry_point))
                .with_context(|| format!("Cannot parse entry point file \"{}\" to toml (tag directory skipped).", master_config.tag_entry_point))?;
            let specification = tagfiles::TagSpecification::from_table(specification_root.handle())?;
            for group_name in specification.group_paths.unwrap_or(vec![master_config.tag_entry_point]) {
                warn_continue!(capture_err!( {
                    use optwrite::OptWrite;
                    let group_root = TableRoot::from_file_path(
                        tag_directory_path.join(group_name))
                        .context("Cannot parse binding group to toml (group skipped).")?;
                    let group = tagfiles::TagGroup::from_table(group_root.handle())
                        .context("Error while interpreting binding group (group skipped).")?;
                    let group_options = master_config.group_options.clone().overriden_by(group.options);
                    let mut file_buffer_tuples = {
                        let mut o: Vec<(std::path::PathBuf, String)> = Vec::with_capacity(group.files.len());
                        for file_name in &group.files {
                            let affecting_file_path = tag_directory_path.parent().unwrap().join(file_name);
                            let axbind_file_path =
                                tag_directory_path.parent().unwrap().join(escaped_manip(
                                    group_options.axbind_file_format.unwrap().as_str(),
                                    master_config.meta_options.escape_sequence.unwrap(),
                                    |format| format.replace(
                                        master_config.meta_options.wildcard_char.unwrap(),
                                        file_name)));
                            o.push((affecting_file_path, warn_continue!(std::fs::read_to_string(&axbind_file_path)
                                .with_context(|| format!("Error reading axbind file {:?} (file skipped).", axbind_file_path)))));
                        }
                        o
                    };
                    for (i, layer) in group.layers.iter().enumerate() {
                        warn_continue!(capture_err!( {
                            use optwrite::OptWrite;
                            let (bind_keys, bind_values) = layer.generate_bindings(
                                &mut map_registry,
                                &mut function_registry,
                                &master_config.meta_options,
                                master_config.layer_options.key_format.unwrap().as_str())
                                .context("Error evaluating bindings (layer skipped).")?;
                            let corasick = aho_corasick::AhoCorasick::new(&bind_keys)
                                .context("Error creating 'aho_corasick' object; this is a rare error that should not occur (unless very irregular map keys are specified?), see rust docs for aho_corasick::AhoCorasick::new()")?;
                            let escape_sequence = master_config.layer_options.escape_sequence.clone()
                                .overriden_by(layer.options.escape_sequence).unwrap();
                            for (_, buffer) in file_buffer_tuples.iter_mut() {
                                *buffer = escaped_manip(buffer, escape_sequence, |text| {
                                    corasick.replace_all(text, bind_values.as_slice())
                                });
                            }
                            //absolute bollocks, will 'fix' later
                        }).with_context(|| format!("IN - layer: {}", i))
                        .with_context(|| format!("IN - binding group: \"{}\"", group_name))
                        .with_context(|| format!("IN - tag directory: {:?}", tag_directory_path)));
                    }
                    for (file_path, buffer) in file_buffer_tuples {
                        std::fs::write(&file_path, buffer)
                            .with_context(|| format!("Unable to write to file {:?} (file skipped).", file_path))?;
                    }
                }).with_context(|| format!("binding group: \"{}\"", group_name))
                .with_context(|| format!("tag directory: {:?}", tag_directory_path)));
            }
        }).with_context(|| format!("tag directory: {:?}", tag_directory_path)));
    }
    Ok(())
}

fn main() {
    use std::process::exit;
    match program() {
        Err(e) => {
            display_err!("[FATAL!]", e);
            exit(1);
        }
        Ok(_) => {
            eprintln!("[OK!]");
            exit(0);
        }
    }
}
