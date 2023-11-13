use anyhow::{Context, Error};
use axbind::*;
use toml_context::TableRoot;
fn program() -> Result<(), Error> {
    let program_args = args::ProgramArgs::from_runinfo(gfunc::run::RunInfo::get_from_env());
    let master_root = gfunc::for_until(&program_args.config_paths, |path| toml_context::TableRoot::from_file_path(path).ok())
        .with_context(|| format!("No valid config files could be found -- paths checked: {:#?}\n (check for invalid toml syntax)",
                                 program_args.config_paths))?;
    let master_config = config::MasterConfig::from_table(master_root.handle())?;
    eprintln!(" >> MASTER CONFIG :: {:#?}", master_config);
    macro_rules! get_registry_roots {
        ($name:expr, $path:expr) =>
        {
             gfunc::fnav::rsearch_dir_pred($path, |file_path| file_path.is_file())
                .with_context(|| format!("Error reading {} directory. ({})", $name, $path))?
                .into_iter()
                .filter_map(|path| warn!(TableRoot::from_file_path(&path)
                    .with_context(|| format!("Cannot parse file {:?} to toml. (file skipped)", path))).ok())
                .collect::<Vec<TableRoot>>()
        }
    }
    let map_roots = get_registry_roots!("map", master_config.map_directory);
    let function_roots = get_registry_roots!("function", master_config.function_directory);
    let mut map_registry = registry::Registry::<registry::BindMap>::from_handles(
        map_roots.iter()
        .map(|root| root.handle()));
    let mut function_registry = registry::Registry::<registry::BindFunction>::from_handles(
        function_roots.iter()
        .map(|root| root.handle()));
    eprintln!(" >> MAP REGISTRY :: {:#?}", map_registry);
    eprintln!(" >> FUNCTION REGISTRY :: {:#?}", function_registry);
    let tag_directory_paths = gfunc::fnav::rsearch_dir(&program_args.root_directory, master_config.tag_directory, gfunc::fnav::MetaType::Directory)
        .with_context(|| format!("Could not read specified root directory {:?}.", program_args.root_directory))?;
    eprintln!(" >> TAG DIRECTORIES FOUND :: {:#?}", tag_directory_paths);
    macro_rules! capture_err {
        ($code:block) => {
            (|| -> Result<(), Error> {
                Ok($code)
            })()
        }
    }
    //this is certainly something.
    for tag_directory_path in &tag_directory_paths {
        warn_continue!(capture_err!( {
            let specification_root = TableRoot::from_file_path(
                tag_directory_path.join(tagfiles::ENTRYPOINT_FILE))
                .with_context(|| format!("Cannot parse entrypoint file ({}) to toml.", tagfiles::ENTRYPOINT_FILE))?;
            let specification = tagfiles::TagSpecification::from_table(specification_root.handle())?;
            for group_name in specification.group_paths {
                warn_continue!(capture_err!( {
                    use optwrite::OptWrite;
                    let group_root = TableRoot::from_file_path(
                        tag_directory_path.join(group_name))
                        .context("Cannot parse binding group to toml.")?;
                    let group = tagfiles::TagGroup::from_table(group_root.handle())
                        .context("Error while interpreting binding group.")?;
                    let group_options = master_config.group_options.clone().overriden_by(group.options);
                    let mut file_buffer_tuples = {
                        let mut o: Vec<(std::path::PathBuf, String)> = Vec::with_capacity(group.files.len());
                        for file_name in &group.files {
                            let affecting_file_path = tag_directory_path.parent().unwrap().join(file_name);
                            let axbind_file_path =
                                tag_directory_path.parent().unwrap().join(escaped_manip(
                                    group_options.axbind_file_format.unwrap().as_str(),
                                    master_config.meta_options.wildcard_char.unwrap(),
                                    |format| format.replace(
                                        master_config.meta_options.wildcard_char.unwrap(),
                                        file_name)));
                            o.push((affecting_file_path, std::fs::read_to_string(&axbind_file_path)
                                .with_context(|| format!("Error reading axbind file {:?}.", axbind_file_path))?));
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
                                .context("Error evaluating bindings")?;
                            let corasick = aho_corasick::AhoCorasick::new(bind_keys)
                                .context("Error creating 'aho_corasick' object; this is a rare error that should not occur (unless very irregular map keys are specified?), see rust docs for aho_corasick::AhoCorasick::new()")?;
                            let escape_char = master_config.layer_options.escape_char.clone()
                                .overriden_by(layer.options.escape_char).unwrap();
                            for (_, buffer) in file_buffer_tuples.iter_mut() {
                                *buffer = escaped_manip(buffer, escape_char, |text| {
                                    corasick.replace_all(text, bind_values.as_slice())
                                });
                            }
                        }).with_context(|| format!("Error in layer {}.", i)));
                    }
                    for (file_path, buffer) in file_buffer_tuples {
                        std::fs::write(&file_path, buffer)
                            .with_context(|| format!("Unable to write to file {:?}.", file_path))?;
                    }
                }).with_context(|| format!("Error in tag group '{}'.", group_name)));
            }
        }).with_context(|| format!("Error in tag directory {:?}", tag_directory_path)));
    }
    Ok(())
}

fn main() {
    match program() {
        Err(e) => { 
            eprintln!("[FATAL!]");
            let mut spaces = String::new();
            for (i, cause) in e.chain().enumerate() {
                spaces.push(' ');
                eprintln!(" ({}){}> {}", i, spaces, cause);
            }
        }
        Ok(_) => eprintln!(" >> OK <<"),
    }
}
