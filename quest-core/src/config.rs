use std::path::PathBuf;

pub struct Config {
    pub http_file_path: PathBuf,
    pub env_var_file: Option<PathBuf>,
}
