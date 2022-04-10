use anyhow::{bail, Result};
use clokwerk::{Scheduler, TimeUnits};
use reqwest::{
    blocking::{self, Client},
    StatusCode,
};
use serde::{Deserialize, Serialize};
use std::{env, thread, time::Duration};

#[allow(dead_code)]
#[derive(Deserialize, Serialize)]
struct FullCandle {
    mts: u128,
    open: f64,
    close: f64,
    high: f64,
    low: f64,
    volume: f64,
}

#[derive(Deserialize, Serialize)]
struct Candle {
    ts: u64,
    close: f64,
}
impl From<FullCandle> for Candle {
    fn from(full_candle: FullCandle) -> Self {
        Candle {
            ts: (full_candle.mts / 1000) as u64,
            close: full_candle.close,
        }
    }
}

fn fetch(time_frame: &str, pair: &str, limit: u16) -> Result<Vec<Candle>> {
    // See: https://docs.bitfinex.com/reference#rest-public-candles
    let mut response = blocking::get(format!(
        "https://api-pub.bitfinex.com/v2/candles/trade:{time_frame}:t{pair}/hist?limit={limit}"
    ))?
    .json::<Vec<FullCandle>>()?
    .into_iter()
    .map(Candle::from)
    .collect::<Vec<Candle>>();
    response.reverse();
    Ok(response)
}

fn predict_candles(prev_candles: Vec<Candle>, request_url: &str) -> Result<Vec<Candle>> {
    let response = Client::new().post(request_url).json(&prev_candles).send()?;
    match response.status() {
        StatusCode::OK => {
            let next_candles = response.json::<Vec<Candle>>()?;
            Ok(next_candles)
        }
        _ => bail!(response.text()?),
    }
}

fn make_scheduler() -> Result<Scheduler> {
    let mut scheduler = Scheduler::new();
    let interval = env::var("CLIENT_INTERVAL")?.parse::<u32>()?;
    let time_frame = env::var("CLIENT_TIME_FRAME")?;
    let pair = env::var("CLIENT_PAIR")?;
    let limit = env::var("CLIENT_LIMIT")?.parse::<u16>()?;
    let logger_hostname = env::var("LOGGER_HOSTNAME")?;
    let logger_port = env::var("LOGGER_PORT")?.parse::<u16>()?;
    scheduler.every(interval.minutes()).run(move || {
        if let Ok(prev_candles) = fetch(&time_frame, &pair, limit) {
            match predict_candles(
                prev_candles,
                &format!("http://{logger_hostname}:{logger_port}/predict_candles"),
            ) {
                Ok(next_candles) => {
                    if let Some(Candle { ts, close }) = next_candles.first() {
                        println!("{ts}: {close}");
                    }
                }
                Err(error) => println!("{error}"),
            }
        }
    });
    Ok(scheduler)
}

fn main() -> Result<()> {
    let mut scheduler = make_scheduler()?;
    loop {
        scheduler.run_pending();
        thread::sleep(Duration::from_millis(100));
    }
}
