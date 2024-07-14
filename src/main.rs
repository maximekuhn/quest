use clap::Parser;

mod args;
mod executor;
mod parser;
mod request;
mod response;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // parse args from CLI
    let args = args::Args::parse();

    // read input .http file
    let file_content = std::fs::read_to_string(args.file)?;

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
