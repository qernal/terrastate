use hyper::{Body, Request, Response};
use crate::components::response;

// lock state handler
pub async fn handler(workspace: String, _req: Request<Body>, dbconn: etcd_client::Client) -> Result<Response<Body>, hyper::Error> {
    // check if it's already locked
    let resp = dbconn.clone().get(format!("{}/state", workspace), None).await;
    match resp {
        Ok(v) => {
            if let Some(_) = v.kvs().first() {
                return Ok(response::resp("", 423, "Locked".to_string()))
            }
        },
        Err(e) => return Ok(response::resp(e.to_string(), 500, "Internal Server Error".to_string()))
    }

    // lock and return
    match dbconn.clone().put(format!("{}/lock", workspace), "true", None).await {
        Ok(_) => return Ok(response::resp("OK".to_string(), 200, "".to_string())),
        Err(e) => return Ok(response::resp(e.to_string(), 500, "Internal Server Error".to_string()))
    }
}