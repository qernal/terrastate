use hyper::{Body, Method, Request, Response, StatusCode};
use etcd_client::{Client, Error};
use crate::components::response;
use serde::{Serialize};

#[derive(Serialize)]
struct Status {
    health: String
}

// status handler
// TODO: add etcd connection checking
pub fn handler(req: Request<Body>, dbconn: etcd_client::Client) -> Result<Response<Body>, hyper::Error> {
    return Ok(
        response::resp(Status { health: "OK".to_string() }, 200, "".to_string())
    )
}