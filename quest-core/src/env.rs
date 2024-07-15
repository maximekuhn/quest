use std::{collections::HashMap, env, io, path::PathBuf};

pub struct Environment {
    variables: HashMap<String /* name */, String /* value */>,
}

impl Default for Environment {
    fn default() -> Self {
        Self {
            variables: default_variables(),
        }
    }
}

impl Environment {
    pub fn init(dot_env_file: Option<PathBuf>) -> Result<Self, io::Error> {
        // load environment files from an env file, if any
        let _ = dotenvy::from_path(dot_env_file.unwrap_or(".env".into()));
        Ok(Self::default())
    }

    pub fn replace_variables(&self, raw_http_file_content: String) -> String {
        // do it the dummy way, will make it better later
        let mut file_content = raw_http_file_content;
        for (name, value) in &self.variables {
            // replace {{name}} with value
            file_content = file_content.replace(format!("{{{{{}}}}}", name).as_str(), value);
        }
        file_content
    }
}

fn default_variables() -> HashMap<String, String> {
    env::vars().collect()
}
