use crate::response::HttpResponse;

pub struct ExecutionOutput {
    pub results: Vec<RequestExecutionDetail>,
}

pub struct RequestExecutionDetail {
    pub name: String,
    pub execuration_duration_ms: u128,
    pub response: Result<HttpResponse, Box<dyn std::error::Error>>,
}
