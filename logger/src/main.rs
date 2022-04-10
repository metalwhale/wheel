use actix_web::{post, web::Json, App, HttpResponse, HttpServer, Responder};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Deserialize, Serialize)]
struct Candle {
    ts: u64,
    close: f64,
}

#[post("/predict_candles")]
async fn predict_candles(prev_candles: Json<Vec<Candle>>) -> impl Responder {
    match prev_candles.last() {
        Some(latest_candle) => HttpResponse::Ok().json(vec![latest_candle]),
        None => HttpResponse::BadRequest().body("Data is too short."),
    }
}

#[actix_web::main]
async fn main() -> Result<()> {
    let port = env::var("LOGGER_PORT")?.parse::<u16>()?;
    HttpServer::new(|| App::new().service(predict_candles))
        .bind(("0.0.0.0", port))?
        .run()
        .await?;
    Ok(())
}
