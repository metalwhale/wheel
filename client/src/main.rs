use clokwerk::{Scheduler, TimeUnits};
use serde::Deserialize;
use std::{env, error::Error, thread, time::Duration};

#[allow(dead_code)]
#[derive(Deserialize)]
struct Candle {
    timestamp: u128,
    open: f64,
    close: f64,
    high: f64,
    low: f64,
    volume: f64,
}

fn fetch(time_frame: &str, pair: &str, limit: u16) -> Result<Vec<Candle>, Box<dyn Error>> {
    // See: https://docs.bitfinex.com/reference#rest-public-candles
    let mut response = reqwest::blocking::get(format!(
        "https://api-pub.bitfinex.com/v2/candles/trade:{time_frame}:t{pair}/hist?limit={limit}"
    ))?
    .json::<Vec<Candle>>()?;
    response.reverse();
    Ok(response)
}

fn make_scheduler() -> Result<Scheduler, Box<dyn Error>> {
    let mut scheduler = Scheduler::new();
    let interval = env::var("CLIENT_INTERVAL")?.parse::<u32>()?;
    let time_frame = env::var("CLIENT_TIME_FRAME")?;
    let pair = env::var("CLIENT_PAIR")?;
    let limit = env::var("CLIENT_LIMIT")?.parse::<u16>()?;
    scheduler.every(interval.minutes()).run(move || {
        if let Ok(candles) = fetch(&time_frame, &pair, limit) {
            if let Some(last_candle) = candles.last() {
                println!("{}: {}", last_candle.timestamp, last_candle.close);
            }
        }
    });
    Ok(scheduler)
}

fn main() {
    match make_scheduler() {
        Ok(mut scheduler) => loop {
            scheduler.run_pending();
            thread::sleep(Duration::from_millis(100));
        },
        Err(error) => {
            println!("{error}");
        }
    }
}
