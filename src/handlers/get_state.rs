use hyper::{Body, Request, Response};
use crate::components::response;
use serde_json::Value;

// get state handler
pub async fn handler(workspace: String, _req: Request<Body>, dbconn: etcd_client::Client) -> Result<Response<Body>, hyper::Error> {
    let resp = dbconn.clone().get(format!("{}/state", workspace), None).await;

    match resp {
        Ok(v) => {
            if let Some(kv) = v.kvs().first() {
                let value = kv.value_str().unwrap();
                let json: Value = serde_json::from_str(value).unwrap();
                return Ok(response::resp(json, 200, "".to_string()))
            }

            return Ok(response::resp("".to_string(), 404, "Not Found".to_string()))
        },
        Err(e) => return Ok(response::resp(e.to_string(), 500, "Internal Server Error".to_string()))
    }
}