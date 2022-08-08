use std::net::SocketAddr;

use tokio::net::{TcpListener};
use hyper::server::conn::Http;
use hyper::service::service_fn;
use hyper::{header, Body, Method, Request, Response, StatusCode};

mod domains;

type GenericError = Box<dyn std::error::Error + Send + Sync>;
type Result<T> = std::result::Result<T, GenericError>;

// cardmod entrypoint
#[tokio::main]
async fn main() -> Result<()> {

    let addr: SocketAddr = "0.0.0.0:8000".parse().unwrap();
    let listener = TcpListener::bind(&addr).await?;

    loop {
        let (stream, _) = listener.accept().await?;

        tokio::task::spawn(async move {
            let service = service_fn(move |req| carmod(req));

            if let Err(err) = Http::new().serve_connection(stream, service).await {
                println!("failed to serve connection: {:?}", err);
            }
        });
    }

}

// CardMod API Service.
// TODO: move this to another mod / (mod + struct?)
async fn carmod(req: Request<Body>) -> Result<Response<Body>> {
    match (req.method(), req.uri().path()) {

        // Health API
        (&Method::GET, "/v1/health") => describe_health().await,

        // Cards API
        (&Method::GET, "/v1/cards") => list_cards().await,
        // (&Method::POST, "/v1/cards") => create_card(req).await,

        // handle not found
        _ => {
            // TODO: BaseAPIError
            Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body("{\"error\": \"no route defined\"}".into())
                .unwrap())
        }
    }
}

// Get the current status of the server.
async fn describe_health() -> Result<Response<Body>> {

    // describe: initialize new health entity
    let health = crate::domains::health::new();

    // describe: construct health response
    let res = match serde_json::to_string(&health) {
        Ok(json) => Response::builder()
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(json))
            .unwrap(),
        Err(_) => Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body("{\"error\": \"internal server error\"}".into())
            .unwrap(),
    };

    // describe: close the handler
    Ok(res)

}

// List the current cards.
async fn list_cards() -> Result<Response<Body>> {

    // describe: initialize new health entity
    let cards = [
        crate::domains::card::new(
            String::from("Ifrit"),
            8,
        ),
        crate::domains::card::new(
            String::from("Quistis"),
            9,
        ),
        crate::domains::card::new(
            String::from("MiniMog"),
            9,
        ),
    ];

    // describe: construct health response
    let res = match serde_json::to_string(&cards) {
        Ok(json) => Response::builder()
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(json))
            .unwrap(),
        Err(_) => Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body("{\"error\": \"internal server error\"}".into())
            .unwrap(),
    };

    // describe: close the handler
    Ok(res)

}