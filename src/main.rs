use axum::{routing::get, Router, extract::Query, response::Html, http::StatusCode};
use serde::Deserialize;
use std::{net::SocketAddr, fmt::format};
use tower_http::cors::{Any, CorsLayer};

mod custom_error;


#[tokio::main]
async fn main() {
    let cors = CorsLayer::new().allow_origin(Any);
    // Route all requests on "/" endpoint to anonymous handler.
    //
    // A handler is an async function which returns something that implements
    // `axum::response::IntoResponse`.

    // A closure or a function can be used as handler.

    let app = Router::new()
        .route("/", get(handler_ipv4))
        .layer(cors);
    //        Router::new().route("/", get(|| async { "Hello, world!" }));

    // Address that server will bind to.
    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));

    // Use `hyper::server::Server` which is re-exported through `axum::Server` to serve the app.
    axum::Server::bind(&addr)
        // Hyper server takes a make service.
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Deserialize)]
struct NetworkData {
    n: String, // Network
    r: String, // Router
    h0: String, // Host 0
    h1: String, // Host 1
    br: String, // Broadcast
}


// Maybe update to Regex
fn validate_ipv4(s: &str) -> bool
{
    s.chars().all(|c| c.is_ascii_digit() || c == '.' || c =='/')
}

async fn handler_ipv4(Query(data): Query<NetworkData>) -> Result<String,StatusCode> {
    // TODO sanitize input
    if !validate_ipv4(&data.n) { return Err(StatusCode::BAD_REQUEST)}
    if !validate_ipv4(&data.r) { return Err(StatusCode::BAD_REQUEST)}
    if !validate_ipv4(&data.h0) { return Err(StatusCode::BAD_REQUEST)}
    if !validate_ipv4(&data.h1) { return Err(StatusCode::BAD_REQUEST)}
    if !validate_ipv4(&data.br) { return Err(StatusCode::BAD_REQUEST)}

    let formatted_template = format!("
    <svg viewBox=\"0 0 174 174\" xmlns=\"http://www.w3.org/2000/svg\" xml:space=\"preserve\" style=\"fill-rule:evenodd;clip-rule:evenodd;stroke-linecap:round;stroke-linejoin:round;stroke-miterlimit:1.5\">
    <path d=\"M161.3 29.526c0-.842-.684-1.526-1.526-1.526H86.526c-.842 0-1.526.684-1.526 1.526v73.248c0 .842.684 1.526 1.526 1.526h73.248c.842 0 1.526-.684 1.526-1.526V29.526Z\" style=\"fill:#fff;stroke:#000;stroke-width:.44px\" transform=\"translate(-191.096 -62.38) scale(2.25819)\"/>
    <text x=\"120\" y=\"195.375\" style=\"font-family:&quot;Roboto-Regular&quot;,&quot;Roboto&quot;;font-size:5px\" transform=\"translate(-65 -188.15)\">Network: {NET}</text>
    <text x=\"120\" y=\"195.375\" style=\"font-family:&quot;Roboto-Regular&quot;,&quot;Roboto&quot;;font-size:5px\" transform=\"translate(-65 -24.85)\">Broadcast: {BR}</text>
    <text x=\"120\" y=\"195.375\" style=\"font-family:&quot;Roboto-Regular&quot;,&quot;Roboto&quot;;font-size:5px\" transform=\"translate(-51 -166.54)\">{ROUTER}</text>
    <circle cx=\"18\" cy=\"16\" r=\"14\" style=\"fill:#fff;stroke:#000;stroke-width:.9px\" transform=\"translate(67.071 29.746) scale(1.10714)\"/>
    <path d=\"M88 49.46h-2v5h-1l2 4 2-4h-1v-5Zm-3-1v-2h-5v-1l-4 2 4 2v-1h5Zm1-3h2v-5h1l-2-4-2 4h1v5Zm3 1v2h5v1l4-2-4-2v1h-5Z\"/>
    <text x=\"136.029\" y=\"195.375\" style=\"font-family:&quot;Roboto-Regular&quot;,&quot;Roboto&quot;;font-size:5px\" transform=\"translate(-57 -91.92)\">Switch</text>
    <path style=\"fill:#fff;stroke:#000;stroke-width:1.03px\" transform=\"translate(71.5 74.866) scale(.96875)\" d=\"M0 3h32v20H0z\"/>
    <path d=\"M90 80.46v-1l4 2-4 2v-1h-7v-2h7Zm-7 6v1l-4-2 4-2v1h7v2h-7Zm7 2v-1l4 2-4 2v-1h-7v-2h7Zm-7 6v1l-4-2 4-2v1h7v2h-7Z\"/>
    <text x=\"120\" y=\"195.375\" style=\"font-family:&quot;Roboto-Regular&quot;,&quot;Roboto&quot;;font-size:5px\" transform=\"translate(-17 -50.022)\">{HOST1}</text>
    <path d=\"M32 4.1A2.11 2.11 0 0 0 29.903 2H2.097A2.11 2.11 0 0 0 0 4.1v16.8A2.11 2.11 0 0 0 2.097 23h27.806A2.11 2.11 0 0 0 32 20.9V4.1Z\" style=\"fill:#fff;stroke:#000;stroke-width:1.1px\" transform=\"matrix(.90625 0 0 .90476 104.5 114.918)\"/>
    <path d=\"M32 12.5C32 6.705 31.753 2 31.448 2H.552C.247 2 0 6.705 0 12.5S.247 23 .552 23h30.896c.305 0 .552-4.705.552-10.5Z\" style=\"stroke:#000;stroke-width:1.56px\" transform=\"matrix(.90625 0 0 .04762 104.5 137.133)\"/>
    <text x=\"120\" y=\"195.375\" style=\"font-family:&quot;Roboto-Regular&quot;,&quot;Roboto&quot;;font-size:5px\" transform=\"translate(-81 -50.022)\">{HOST0}</text>
    <path d=\"M32 4.1A2.11 2.11 0 0 0 29.903 2H2.097A2.11 2.11 0 0 0 0 4.1v16.8A2.11 2.11 0 0 0 2.097 23h27.806A2.11 2.11 0 0 0 32 20.9V4.1Z\" style=\"fill:#fff;stroke:#000;stroke-width:1.1px\" transform=\"matrix(.90625 0 0 .90476 40.5 114.918)\"/>
    <path d=\"M32 12.5C32 6.705 31.753 2 31.448 2H.552C.247 2 0 6.705 0 12.5S.247 23 .552 23h30.896c.305 0 .552-4.705.552-10.5Z\" style=\"stroke:#000;stroke-width:1.56px\" transform=\"matrix(.90625 0 0 .04762 40.5 137.133)\"/>
    <circle cx=\"189\" cy=\"166\" r=\"1\" transform=\"translate(-107 -32.84)\"/><circle cx=\"189\" cy=\"166\" r=\"1\" transform=\"translate(-102 -32.84)\"/><circle cx=\"189\" cy=\"166\" r=\"1\" transform=\"translate(-97 -32.84)\"/>
    <path d=\"M200 110V96m-32 53c.889-21.778-1.322-30.48 16-29\" style=\"fill:none;stroke:#000;stroke-width:1px\" transform=\"translate(-113 -32.84)\"/>
    <path d=\"M168 149c.889-21.778-1.322-30.48 16-29\" style=\"fill:none;stroke:#000;stroke-width:1px\" transform=\"matrix(-1 0 0 1 287 -32.84)\"/>
    </svg>",
        NET = data.n,
        BR = data.br,
        ROUTER = data.r,
        HOST0 = data.h0,
        HOST1 = data.h1,
    );

    Ok(formatted_template)
}