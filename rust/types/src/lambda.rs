use aws_lambda_events::{
    http::{HeaderMap, HeaderValue},
    lambda_function_urls::LambdaFunctionUrlResponse,
};
use lambda_runtime::Error;
use serde::Serialize;
use tracing::info;

// /// The header name for the question format.
// /// The value should be one of the `QuestionFormat` enum values.
// /// CloudFront has to be configured to vary the cache depending on the header contents.
// pub const QUESTION_FORMAT_HEADER_NAME: &str = "x-bitie-question-format";

/// A shortcut for returning the lambda response in the required format.
/// Always returns OK.
/// Adds `Content-Type=text/html` header.
pub fn text_response(body: Option<String>, status: i64) -> Result<LambdaFunctionUrlResponse, Error> {
    // a collector for all headers added along the way
    let mut headers = HeaderMap::new();
    headers.append("Content-Type", HeaderValue::from_static("text/html; charset=utf-8"));

    Ok(LambdaFunctionUrlResponse {
        status_code: status,
        headers,
        cookies: Default::default(),
        body,
        is_base64_encoded: false,
    })
}

/// A shortcut for returning the lambda response in the required format.
/// May return an error if the serialization of the body object fails.
/// Adds `Content-Type=application/json` header.
pub fn json_response<T: Serialize>(body: Option<&T>, status: i64) -> Result<LambdaFunctionUrlResponse, Error> {
    let body = match body {
        Some(v) => match serde_json::to_string(v) {
            Ok(v) => Some(v),
            Err(e) => {
                info!("Failed to serialize the response: {:?}", e);
                return text_response(Some(e.to_string()), 400);
            }
        },
        None => None,
    };

    // a collector for all headers added along the way
    let mut headers = HeaderMap::new();
    headers.append(
        "Content-Type",
        HeaderValue::from_static("application/json; charset=utf-8"),
    );

    Ok(LambdaFunctionUrlResponse {
        status_code: status,
        headers,
        cookies: Default::default(),
        body,
        is_base64_encoded: false,
    })
}
