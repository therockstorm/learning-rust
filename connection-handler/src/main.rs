// use aws_lambda_events::event::apigw::ApiGatewayWebsocketProxyRequest;
use lambda_runtime::{error::HandlerError, lambda, Context};
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Clone, PartialEq, Deserialize)]
struct RequestContext {
    #[serde(rename = "connectionId")]
    connection_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
struct Req {
    #[serde(rename = "requestContext")]
    request_context: RequestContext,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
struct Res {
    status_code: i16
}

fn handler(event: Req, _: Context) -> Result<Res, HandlerError> {
    println!("Received {:?}", event);

    // for record in event.records {
    //     println!(
    //         "{}",
    //         record.body.unwrap_or_else(|| "empty body".to_string())
    //     );
    // }

    Ok(Res{status_code: 200})
}

fn main() -> Result<(), Box<dyn Error>> {
    lambda!(handler);
    Ok(())
}

#[cfg(test)]
mod tests {
    // use super::*;
    // use serde_json::json;

    // #[test]
    // fn handler_handles() {
    //     let event = json!({ "answer": 42 });
    //     assert_eq!(
    //         handler(event.clone(), Context::default()).expect("expected Ok(_) value"),
    //         event
    //     )
    // }
}
