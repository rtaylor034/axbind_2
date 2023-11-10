use anyhow::{Context, Error};
use axbind::*;
fn program() -> Result<(), Error> {
    let program_args = args::ProgramArgs::from_runinfo(gfunc::run::RunInfo::get_from_env());
    let root_master = gfunc::for_until(&program_args.config_paths, |path| toml_context::TableRoot::from_file_path(path).ok())
        .with_context(|| format!("No valid config files could be found -- paths checked: {:#?}\n (check for invalid toml syntax)",
                                 program_args.config_paths))?;
    let master_config = config::MasterConfig::from_table(root_master.handle())?;
    eprintln!(" >> MASTER CONFIG :: {:#?}", master_config);
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
