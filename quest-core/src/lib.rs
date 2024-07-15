mod env;
mod executor;
mod parser;
mod request;
mod response;

pub mod config;
pub mod error;
pub mod output;

pub fn run(config: config::Config) -> Result<output::ExecutionOutput, error::Error> {
    // read .http file
    let file_content = std::fs::read_to_string(config.http_file_path)?;

    // inject environment variables, if any
    let environment = env::Environment::init(config.env_var_file)?;
    let file_content = environment.replace_variables(file_content);

    // parse requests
    let requests = parser::parse_http_file(file_content)?;

    // execute all requests
    let executor = executor::Executor::new();
    let mut results = Vec::with_capacity(requests.len());
    for request in requests {
        let request_name = request.name.clone();
        let now = std::time::Instant::now();
        let result = executor.execute(request);
        let elapsed_ms = now.elapsed().as_millis();

        let execution_details = output::RequestExecutionDetail {
            name: request_name,
            execuration_duration_ms: elapsed_ms,
            response: result,
        };

        results.push(execution_details);
    }

    Ok(output::ExecutionOutput { results })
}
