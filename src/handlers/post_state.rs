use hyper::{body, Body, Request, Response};
use crate::components::response;
use serde_json::Value;

// post state handler
pub async fn handler(workspace: String, req: Request<Body>, dbconn: etcd_client::Client) -> Result<Response<Body>, hyper::Error> {
    let body = body::to_bytes(req.into_body()).await?;
    let json_req: Value = match serde_json::from_slice(&body) {
        Ok(v) => v,
        Err(e) => return Ok(response::resp(e.to_string(), 400, "Validation failed".to_string()))
    };

    match dbconn.clone().put(format!("{}/state", workspace), json_req.to_string(), None).await {
        Ok(_) => return Ok(response::resp("OK".to_string(), 200, "".to_string())),
        Err(e) => return Ok(response::resp(e.to_string(), 500, "Internal Server Error".to_string()))
    }
}