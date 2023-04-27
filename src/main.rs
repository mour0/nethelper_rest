use axum::{routing::get, Router, extract::Query, http::StatusCode, Extension};
use sanitize_html::{rules::predefined::DEFAULT, sanitize_str};
use serde::Deserialize;
use std::{net::SocketAddr, time::Duration};
use tower_http::cors::{Any, CorsLayer};

use sqlx::{SqlitePool, sqlite::SqliteConnectOptions};



const MAX_AGE: u64 = 86400;

#[tokio::main]
async fn main() {
    println!("[+] Started");
    // -- Database --
    let pool = SqlitePool::connect_with(
        SqliteConnectOptions::new()
            .filename(std::path::Path::new("history.db"))
            .create_if_missing(true)
    ).await.expect("Failed to connect to db");

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS history (
            email TEXT NOT NULL PRIMARY KEY,
            last_input TEXT
        )"
    ).execute(&pool).await.expect("Failed to execute Query");

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .max_age(Duration::from_secs(MAX_AGE));

    let app = Router::new()
        .route("/ipv4", get(handler_ipv4))
        .route("/history", get(handler_history))
        .route("/save", get(handler_save))
        .layer(Extension(pool))
        .layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// --- HISTORY ---

// Query Map
#[derive(sqlx::FromRow)]
struct History {
    last_input: String,
}

async fn handler_history(Query(email): Query<HistoryQuery>, db_pool: Extension<SqlitePool>) -> Result<String, StatusCode>
{
    println!("[+] History handler");
    let mut conn = db_pool.acquire().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let history_entries = sqlx::query_as::<_, History>(
        "SELECT last_input FROM history WHERE email = ?"
    )
    .bind(email.email)
    .fetch_one(&mut conn)
    .await;

    match history_entries {
        Ok(h) => {
            //println!("[?] last_input: {}", h.last_input);
            let saved = format!("{}", h.last_input);
            return Ok(saved);
        },
        Err(_) => { 
            //println!("[?] Err last_input");
            return Err(StatusCode::NO_CONTENT); 
        },
    }
}

#[derive(Deserialize)]
struct HistoryQuery {
    email: String,
}

// === IMAGE ===
#[derive(Deserialize)]
struct NetworkQuery {
    n: String, // Network
    r: String, // Router
    h0: String, // Host 0
    h1: String, // Host 1
    br: String, // Broadcast
}

//fn validate_ipv4(s: &str) -> bool
//{
//    s.chars().all(|c| c.is_ascii_digit() || c == '.' || c =='/')
//}

async fn handler_ipv4(Query(data): Query<NetworkQuery>) -> Result<String,StatusCode> {

    println!("[+] SVG handler");
    //if !validate_ipv4(&data.n) { return Err(StatusCode::BAD_REQUEST)}
    //if !validate_ipv4(&data.r) { return Err(StatusCode::BAD_REQUEST)}
    //if !validate_ipv4(&data.h0) { return Err(StatusCode::BAD_REQUEST)}
    //if !validate_ipv4(&data.h1) { return Err(StatusCode::BAD_REQUEST)}
    //if !validate_ipv4(&data.br) { return Err(StatusCode::BAD_REQUEST)}

    //let network = sanitize_str(&DEFAULT, &data.n).map_err(|_| StatusCode::BAD_REQUEST)?;
    let network_addr = match sanitize_str(&DEFAULT, &data.n) {
        Ok(res) => res,
        Err(_) => return Err(StatusCode::BAD_REQUEST),
    };

    let router_addr = match sanitize_str(&DEFAULT, &data.r) {
        Ok(res) => res,
        Err(_) => return Err(StatusCode::BAD_REQUEST),
    };

    let h0_addr = match sanitize_str(&DEFAULT, &data.h0) {
        Ok(res) => res,
        Err(_) => return Err(StatusCode::BAD_REQUEST),
    };

    let h1_addr = match sanitize_str(&DEFAULT, &data.h1) {
        Ok(res) => res,
        Err(_) => return Err(StatusCode::BAD_REQUEST),
    };

    let broadcast_addr = match sanitize_str(&DEFAULT, &data.br) {
        Ok(res) => res,
        Err(_) => return Err(StatusCode::BAD_REQUEST),
    };

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
        NET = network_addr,
        BR = broadcast_addr,
        ROUTER = router_addr,
        HOST0 = h0_addr,
        HOST1 = h1_addr,
    );

    Ok(formatted_template)
}


// == SAVE ==
#[derive(Deserialize)]
struct SaveQuery {
    n: String, // Network
    r: String, // Router
    h0: String, // Host 0
    h1: String, // Host 1
    br: String, // Broadcast
    email: String,
}

async fn handler_save(Query(data): Query<SaveQuery>, db_pool: Extension<SqlitePool>) -> StatusCode {

    println!("[+] Save handler");
    let network_addr = match sanitize_str(&DEFAULT, &data.n) {
        Ok(res) => res,
        Err(_) => return StatusCode::BAD_REQUEST,
    };

    let router_addr = match sanitize_str(&DEFAULT, &data.r) {
        Ok(res) => res,
        Err(_) => return StatusCode::BAD_REQUEST,
    };

    let h0_addr = match sanitize_str(&DEFAULT, &data.h0) {
        Ok(res) => res,
        Err(_) => return StatusCode::BAD_REQUEST,
    };

    let h1_addr = match sanitize_str(&DEFAULT, &data.h1) {
        Ok(res) => res,
        Err(_) => return StatusCode::BAD_REQUEST,
    };

    let broadcast_addr = match sanitize_str(&DEFAULT, &data.br) {
        Ok(res) => res,
        Err(_) => return StatusCode::BAD_REQUEST,
    };

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
        NET = network_addr,
        BR = broadcast_addr,
        ROUTER = router_addr,
        HOST0 = h0_addr,
        HOST1 = h1_addr,
    );

    let email = data.email;
    let mut conn = match db_pool.acquire().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR) {
        Ok(c) => c,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR,
    };

    let is_present = sqlx::query("SELECT * FROM history WHERE email = ?")
        .bind(&email)
        .fetch_one(&mut conn)
        .await;

    match is_present {
        Ok(_) => {
            match sqlx::query("UPDATE history SET last_input = ? WHERE email = ?")
                .bind(&formatted_template)
                .bind(&email)
                .execute(&mut conn)
                .await {
                    Ok(_) => { 
                        println!("[+] UPDATE {}", email);
                        return StatusCode::OK 
                    },
                    Err(_) => { return StatusCode::INTERNAL_SERVER_ERROR },
                }
            
        },

        Err(_) => {
            match sqlx::query("INSERT INTO history (email, last_input) VALUES (?, ?)")
                .bind(&email)
                .bind(&formatted_template)
                .execute(&mut conn)
                .await {
                    Ok(_) => { 
                        println!("[+] ISNERT {}", email); 
                        return StatusCode::OK 
                    },
                    Err(_) => { return StatusCode::INTERNAL_SERVER_ERROR },
                }
        }
    }

}