use axbind::*;
fn program() -> Result<(), MainErr> {
    let program_args = args::ProgramArgs::from_runinfo(gfunc::run::RunInfo::get_from_env());
    let config_root = gfunc::for_until(&program_args.config_paths, |path| toml_context::TableRoot::from_file_path(path).ok());
    Ok(())
}

fn main() {
    match program() {
        Err(e) => eprintln!("[FATAL!] {}", e),
        Ok(_) => eprintln!(" >> OK <<"),
    }
}
