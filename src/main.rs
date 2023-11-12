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
    let map_registry = registry::Registry::<registry::BindMap>::from_handles(
        map_roots.iter()
        .map(|root| root.handle()));
    let function_registry = registry::Registry::<registry::BindFunction>::from_handles(
        function_roots.iter()
        .map(|root| root.handle()));
    eprintln!(" >> MAP REGISTRY :: {:#?}", map_registry);
    eprintln!(" >> FUNCTION REGISTRY :: {:#?}", function_registry);
    let tag_directory_paths = gfunc::fnav::rsearch_dir(&program_args.root_directory, master_config.tag_directory, gfunc::fnav::MetaType::Directory)
        .with_context(|| format!("Could not read specified root directory {:?}.", program_args.root_directory))?;
    eprintln!(" >> TAG DIRECTORIES FOUND :: {:#?}", tag_directory_paths);
    for tag_directory_path in &tag_directory_paths {
        let specification_root = warn_continue!(TableRoot::from_file_path(
            tag_directory_path.join(tagfiles::ENTRYPOINT_FILE))
            .with_context(|| format!("Cannot parse entrypoint file ({}) to toml (in tag directory {:?}).",
                    tagfiles::ENTRYPOINT_FILE, tag_directory_path)));
        let specification = tagfiles::TagSpecification::from_table(specification_root.handle())?;
        for group_name in specification.group_paths {
            use optwrite::OptWrite;
            let group_root = warn_continue!(TableRoot::from_file_path(
                tag_directory_path.join(group_name))
                .with_context(|| format!("Cannot parse binding group file '{}' to toml (in tag directory {:?}).",
                    group_name, tag_directory_path)));
            let group = warn_continue!(tagfiles::TagGroup::from_table(group_root.handle())
                .context("Error while interpreting binding group."));
            let group_options = master_config.group_options.clone().overriden_by(group.options);
            for layer in group.layers {
                let layer_options = master_config.layer_options.clone().overriden_by(layer.options);

                //4th layer nested for loop!!1!
                for file_name in &group.files {
                    let affecting_file_path = tag_directory_path.parent().unwrap().join(file_name);
                    let axbind_file_path =
                        tag_directory_path.parent().unwrap().join(escaped_manip(
                            group_options.axbind_file_format.unwrap().as_str(),
                            master_config.meta_options.wildcard_char.unwrap(),
                            |format| format.replace(
                                master_config.meta_options.wildcard_char.unwrap(),
                                file_name)));
                }
            }
        }
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
