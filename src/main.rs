use anyhow::{Context, Error};
use axbind::*;
fn program() -> Result<(), Error> {
    let program_args = args::ProgramArgs::from_runinfo(gfunc::run::RunInfo::get_from_env());
    let config_root = gfunc::for_until(&program_args.config_paths, |path| toml_context::TableRoot::from_file_path(path).ok())
        .with_context(|| format!("No valid config files could be found -- paths checked: {:#?}\n (check for invalid toml syntax)",
                                 &program_args.config_paths))?;
    Ok(())
}

fn main() {
    match program() {
        Err(e) => eprintln!("[FATAL!] {}", e),
        Ok(_) => eprintln!(" >> OK <<"),
    }
}
