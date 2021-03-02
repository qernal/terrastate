mod handlers;
mod components;

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use handlers::*;
use etcd_client::{Client, Error, ConnectOptions};
use regex::Regex;
use std::collections::HashMap;
use components::response;
use std::env;
use base64::encode;

type GenericError = Box<dyn std::error::Error + Send + Sync>;

#[derive(Clone)]
struct Config {
    db_path: String,
    db_user: String,
    db_pass: String,
    bind_server_address: String,
    bind_server_port: String,
    tf_user: String,
    tf_pass: String
}

impl Config {
    pub fn from_env_vars() -> Config {
        return Config {
            db_path: env::var("TS_DB_HOST").unwrap().to_string(),
            db_pass: env::var("TS_DB_PASS").unwrap().to_string(),
            db_user: env::var("TS_DB_USER").unwrap().to_string(),
            bind_server_address: env::var("TS_SERVER_ADDRESS").unwrap().to_string(),
            bind_server_port: env::var("TS_SERVER_PORT").unwrap().to_string(),
            tf_user: env::var("TF_USER").unwrap().to_string(),
            tf_pass: env::var("TF_PASS").unwrap().to_string()
        }
    }
}

#[derive(Clone)]
struct Route<'a> {
    path: &'a str,
    method: String,
    handler: String,
    exact: bool
}

// get path params against a regex
pub fn get_params(regex: &str, request_path: String) -> Result<HashMap<String, String>, String>  {
    println!("Running regex of {} against {}", regex, request_path);
    let re = Regex::new(regex).unwrap();
    let captures = re.captures(&request_path.as_str());

    match captures {
        Some(capture) => {
            let dict: HashMap<String, String> = re
                .capture_names()
                .flatten()
                .filter_map(|n| Some((n.to_string(), capture.name(n)?.as_str().to_string())))
                .collect();
            println!("{:#?}", dict);
            return Ok(dict);
        }
        None => {
            println!("No matches found");
            return Err("No Matches".to_string());
        }
    }
}

// service handler
async fn s_handler(req: Request<Body>, dbconn: etcd_client::Client, routes: [Route<'_>; 7], config: Config) -> Result<Response<Body>, hyper::Error> {
    println!("Incoming req: {:?}", req);
    println!("Incoming headers: {:?}", req.headers());

    let mut route = "";
    let mut workspace: String = "".to_string();

    // validate auth tokens
    if req.headers().contains_key("Authorization") {
        let basic_auth = req.headers().get("Authorization");
        println!("Header parts for auth: {:?}", basic_auth);
        let mut ba_parts = basic_auth.unwrap().to_str().unwrap().split_whitespace();

        let ba_type = ba_parts.next().unwrap();
        let ba_auth = ba_parts.last().unwrap();

        if ba_type.to_lowercase() == "basic" {
            if ba_auth != encode(format!("{}:{}", config.tf_user, config.tf_pass)) {
                return Ok(response::resp("", 403, "Forbidden".to_string()));
            }
        } else {
            return Ok(response::resp("", 403, "Forbidden".to_string()));
        }

        println!("{:?}", basic_auth);
    } else {
        // failed auth
        return Ok(response::resp("", 403, "Forbidden".to_string()));
    }

    // loop through each endpoint and generate path
    for s_route in routes.iter() {
        // skip route if method is mismatched
        if req.method().to_string() != s_route.method {
            continue;
        }

        if s_route.exact {
            println!("Check if {:?} matches {:?}", s_route.path, req.uri().path().to_string());
            if s_route.path == req.uri().path().to_string() {
                route = s_route.handler.as_str();
                break;
            }
        } else {
            // validate path match
            let path = format!(r"(?i){}", s_route.path);
            let captures = get_params(path.as_str(), req.uri().path().to_string());

            match captures {
                Ok(capture) => {
                    let c_workspace = capture["workspace"].clone();
                    workspace = c_workspace.clone();
                    route = s_route.handler.as_str();
                    break;
                }
                Err(_) => {}
            }
        }
    }

    match route {
        "get_status" => {
            return get_status::handler(req, dbconn)
        },
        "post_state" => {
            return post_state::handler(workspace, req, dbconn).await;
        },
        "get_state" => {
            return get_state::handler(workspace, req, dbconn).await;
        },
        "delete_state" => {
            return delete_state::handler(workspace, req, dbconn).await;
        },
        "lock_state" => {
            return lock_state::handler(workspace, req, dbconn).await;
        },
        "unlock_state" => {
            return unlock_state::handler(workspace, req, dbconn).await;
        }
        // 404 for other routes
        _ => {
            return Ok(
                response::resp("", 404, "Not Found".to_string())
            )
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let config = Config::from_env_vars();
    let addr = format!("{}:{}", config.bind_server_address.clone(), config.bind_server_port.clone()).parse().unwrap();

    // db client
    let db_options = Some(ConnectOptions::new().with_user(
        config.db_user.clone(),
        config.db_pass.clone(),
    ));
    let db_client = Client::connect([config.db_path.clone()], db_options).await?;

    let routes = [
        Route {
            path: "/",
            method: "GET".to_string(),
            handler: "get_status".to_string(),
            exact: true
        },
        Route {
            path: "/states",
            method: "GET".to_string(),
            handler: "get_states".to_string(),
            exact: true
        },
        Route {
            path: r"/states/(?P<workspace>[a-zA-Z\-_]+)",
            method: "GET".to_string(),
            handler: "get_state".to_string(),
            exact: false
        },
        Route {
            path: r"/states/(?P<workspace>[a-zA-Z\-_]+)",
            method: "POST".to_string(),
            handler: "post_state".to_string(),
            exact: false
        },
        Route {
            path: r"/states/(?P<workspace>[a-zA-Z\-_]+)",
            method: "DELETE".to_string(),
            handler: "delete_state".to_string(),
            exact: false
        },
        Route {
            path: r"/states/(?P<workspace>[a-zA-Z\-_]+)",
            method: "LOCK".to_string(),
            handler: "lock_state".to_string(),
            exact: false
        },
        Route {
            path: r"/states/(?P<workspace>[a-zA-Z\-_]+)",
            method: "UNLOCK".to_string(),
            handler: "unlock_state".to_string(),
            exact: false
        },
    ];

    let service = make_service_fn(move |_| {
        let db_connection = db_client.clone();
        let c_routes = routes.clone();
        let c_config = config.clone();

        async move {
            Ok::<_, GenericError>(service_fn(move |req| {
                s_handler(req, db_connection.to_owned(), c_routes.clone(), c_config.clone())
            }))
        }
    });

    let server = Server::bind(&addr).serve(service);
    println!("Listening on http://{}", addr);
    server.await?;
    Ok(())
}