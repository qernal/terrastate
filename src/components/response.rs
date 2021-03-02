use hyper::{header, Body, Response, StatusCode};
use serde::{Serialize};

// response struct
#[derive(Serialize)]
struct GenericResponse<T: Serialize> {
    status: i16,
    message: String,
    data: T
}

// generic building struct
pub fn builder<T: Serialize>(data: T, status: i16, message: String) -> Response<Body> {
    let result = GenericResponse{
        data,
        message,
        status
    };

    // select status code
    let status_code = match status {
        200 => StatusCode::OK,
        201 => StatusCode::CREATED,
        400 => StatusCode::BAD_REQUEST,
        401 => StatusCode::UNAUTHORIZED,
        403 => StatusCode::FORBIDDEN,
        404 => StatusCode::NOT_FOUND,
        409 => StatusCode::CONFLICT,
        422 => StatusCode::UNPROCESSABLE_ENTITY,
        500 => StatusCode::INTERNAL_SERVER_ERROR,
        _ => StatusCode::INTERNAL_SERVER_ERROR
    };

    let body = Body::from(serde_json::to_string(&result).unwrap());

    Response::builder()
        .header(header::CONTENT_TYPE, "application/json")
        .status(status_code)
        .body(body)
        .unwrap()
}

// HTTP function for creating hyper response
pub fn resp<T: Serialize>(data: T, status_code: i16, message: String) -> Response<Body> {
    builder(data, status_code, message)
}