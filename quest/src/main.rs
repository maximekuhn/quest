use clap::Parser;
use quest_core::{env, executor, parser};

mod opts;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // parse options
    let opts = opts::Opts::parse();

    // read input .http file
    let file_content = std::fs::read_to_string(opts.file)?;

    // read environment variables (from a .env file)
    let env = env::Environment::init(opts.dotenv)?;

    // replace variables, if any
    let file_content = env.replace_variables(file_content);

    // parse file content into HTTP request(s)
    let http_requests = parser::parse_http_file(file_content)?;
    println!("successfully parsed {} requests", http_requests.len());

    // execute all HTTP requests and display response
    let executor = executor::Executor::new();
    for request in http_requests {
        println!("executing {} {}", request.method, request.url);
        match executor.execute(request) {
            Ok(response) => println!("{}", response),
            Err(err) => println!("failed to execute request: {}", err),
        }
    }

    Ok(())
}
