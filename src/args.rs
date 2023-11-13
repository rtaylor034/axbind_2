use gfunc::run::{RunInfo, ValidateExit};
use gfunc::simple_envpath;

use std::path::PathBuf;

pub struct ProgramArgs {
    pub config_paths: Vec<PathBuf>,
    pub root_directory: PathBuf,
}

impl ProgramArgs {
    pub fn from_runinfo(run_info: RunInfo) -> ProgramArgs {
        let valid_singlet_opts: [(&'static str, Option<char>); 0] = [];
        let valid_valued_opts: [&'static str; 2] = ["tagdir", "config"];
        let valued_opts = run_info.values.validate(valid_valued_opts).auto_exit();
        let _singlet_opts = run_info.options.validate(valid_singlet_opts).auto_exit();
        let args = run_info
            .arguements
            .validate_exact([|_: &_| true])
            .auto_exit();

        let root_directory = PathBuf::from(&args[0]);
        let config_paths = match valued_opts.get("config") {
            Some(cfgpath) => vec![PathBuf::from(cfgpath)],
            None => [
                "$XDG_CONFIG_HOME/axbind/config.toml",
                "$HOME/.config/axbind/config.toml",
                "/etc/axbind/config.toml",
            ]
            .iter()
            .filter_map(|path| simple_envpath(path).ok())
            .collect(),
        };
        ProgramArgs {
            root_directory,
            config_paths,
        }
    }
}
