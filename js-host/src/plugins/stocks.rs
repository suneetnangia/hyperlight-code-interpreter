use anyhow::Result;
use hyperlight_js::ProtoJSSandbox;
use serde::{Deserialize, Serialize};

use super::Plugin;

#[derive(Deserialize, Serialize)]
pub struct Stock {
    pub ticker: String,
    pub name: String,
    pub price: f64,
    pub change: f64,
    pub change_percent: f64,
    pub volume: f64,
    pub market_cap: f64,
    pub timestamp: String,
}

pub struct StocksPlugin {
    pub hostname: String,
}

impl Plugin for StocksPlugin {
    fn name(&self) -> &str {
        "stocks"
    }

    fn register(&self, proto: &mut ProtoJSSandbox) -> Result<()> {
        let hostname = self.hostname.clone();
        proto.register_raw("stocks", "get", move |args: String| {
            let parsed: Vec<Option<String>> = serde_json::from_str(&args)?;
            let symbol = parsed.first().cloned().flatten();

            let url = match &symbol {
                Some(s) => format!("http://{}/api/v1/stocks/{s}", hostname),
                None => format!("http://{}/api/v1/stocks", hostname),
            };

            let response = reqwest::blocking::get(&url)
                .map_err(|_| "failed to perform HTTP request")?;
            if response.status() == 404 {
                return Ok("null".to_string());
            }
            let body = response.text()
                .map_err(|_| "failed to read response body")?;

            match &symbol {
                Some(_) => {
                    let stock: Stock = serde_json::from_str(&body)
                        .map_err(|_| "failed to parse stock response")?;
                    Ok(serde_json::to_string(&stock)?)
                }
                None => {
                    let stocks: Vec<Stock> = serde_json::from_str(&body)
                        .map_err(|_| "failed to parse stocks response")?;
                    Ok(serde_json::to_string(&stocks)?)
                }
            }
        })?;

        Ok(())
    }
}
