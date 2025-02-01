use route_match::route;
use std::net::SocketAddr;

use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

type Err = Box<dyn std::error::Error + Send + Sync>;

async fn serve_response(code: u16, text: &str) -> Result<Response<String>, Err> {
    let response = Response::builder()
        .status(code) // Set the status code
        .body(text.to_string())?;
    Ok(response)
}

async fn on_request(request: Request<hyper::body::Incoming>) -> Result<Response<String>, Err> {
    route! {
        match (&request.method().as_str(), &request.uri().path()) {
            GET /echo/:message => serve_response(200, message).await,
            GET /double/:number => {
                let Ok(x) = number.parse::<f64>() else {
                    return serve_response(
                        400,
                        format!("parse error: {} is not a number", number).as_str()
                    ).await
                };
                serve_response(
                    200,
                    format!("{}", x * 2.0).as_str()
                ).await
            },
            GET /subpath/..:sub => serve_response(200, sub).await
            GET /rest/.. => serve_response(200, "rest").await
            _ => serve_response(404, "not found").await
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Err> {
    let port = 3000;
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let listener = TcpListener::bind(addr).await?;

    println!("Listening on port {}", port);

    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);
        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(io, service_fn(on_request))
                .await
            {
                eprintln!("Error serving connection: {:?}", err);
            }
        });
    }
}
