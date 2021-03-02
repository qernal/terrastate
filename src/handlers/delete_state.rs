use hyper::{Body, Request, Response};
use crate::components::response;

// delete state handler
pub async fn handler(workspace: String, _req: Request<Body>, dbconn: etcd_client::Client) -> Result<Response<Body>, hyper::Error> {
    let resp = dbconn.clone().delete(format!("{}/state", workspace), None).await;

    match resp {
        Ok(_) => return Ok(response::resp("".to_string(), 200, "Deleted".to_string())),
        Err(e) => return Ok(response::resp(e.to_string(), 500, "Internal Server Error".to_string()))
    }
}