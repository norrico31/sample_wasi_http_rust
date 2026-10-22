use std::time::Instant;
use wstd::http::{Body, Request, Response, Result, StatusCode};

#[wstd::http_server]
async fn main(req: Request<Body>) -> Result<Response<Body>> {
    let started = Instant::now();

    let method = req.method().clone();
    let path = req.uri().path().to_owned();

    let result = match (method.as_str(), path.as_str()) {
        ("GET", "/health") => {
            json(StatusCode::OK, r#"{"status":"healthy"}"#)
        }

        ("GET", "/users") => {
            json(
                StatusCode::OK,
                r#"{"users":[{"id":1,"name":"Alice"},{"id":2,"name":"Bob"}]}"#,
            )
        }

        ("GET", "/users/1") => {
            json(
                StatusCode::OK,
                r#"{"id":1,"name":"Alice","role":"admin"}"#,
            )
        }

        ("POST", "/users") => {
            // Prototype only: static response; no database yet.
            json(
                StatusCode::CREATED,
                r#"{"id":3,"name":"New User","message":"User created"}"#,
            )
        }

        ("GET", "/users/999") => {
            json(
                StatusCode::NOT_FOUND,
                r#"{"error":"User not found"}"#,
            )
        }
        ("GET", "/test-error") => {
            json(
                StatusCode::INTERNAL_SERVER_ERROR,
                r#"{"error":""intentional test error"}"#,
            )
        }
        ("GET", "/test-trap") => {
            panic!("intentional WASM trap test");

        }
        ("GET", "/test-timeout") => {
            loop {
                std::hint::spin_loop();
            }
        }
        ("GET", _) | ("POST", _) => {
            json(
                StatusCode::NOT_FOUND,
                r#"{"error":"Route not found"}"#,
            )
        }
        _ => {
            json(
                StatusCode::METHOD_NOT_ALLOWED,
                r#"{"error":"Method not allowed"}"#,
            )
        }

    };

    let response = result?;
    let status = response.status().as_u16();
    let duration_us = started.elapsed().as_micros();
    let outcome = if status >= 500 {
        "server_error"   
    } else if status >=  400 {
        "client_error"
    } else {
        "success"
    };

    println!("observation method={} path={} status={} duration_us={} outcome={}",
        method, path, status, duration_us, outcome
        );

    Ok(response)
}

fn json(
    status: StatusCode,
    body: &'static str,
) -> Result<Response<Body>> {
    Ok(
        Response::builder()
            .status(status)
            .header("content-type", "application/json")
            .body(body.into())
            .expect("valid HTTP response"),
    )
}

