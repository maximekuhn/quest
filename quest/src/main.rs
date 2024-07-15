use clap::Parser;

mod opts;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // parse options
    let opts = opts::Opts::parse();

    // run requests
    let output = quest_core::run(opts.into())?;

    // show output
    for result in output.results {
        println!(
            "REQUEST '{}' took {}ms to execute",
            result.name, result.execuration_duration_ms
        );
        match result.response {
            Ok(res) => println!("{}", res),
            Err(err) => println!("request could not be executed because {}", err),
        }
    }

    Ok(())
}

#[allow(clippy::from_over_into)]
impl Into<quest_core::config::Config> for opts::Opts {
    fn into(self) -> quest_core::config::Config {
        quest_core::config::Config {
            http_file_path: self.file,
            env_var_file: self.dotenv,
        }
    }
}
