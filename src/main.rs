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
