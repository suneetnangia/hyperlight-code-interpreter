use anyhow::Result;
use hyperlight_js::ProtoJSSandbox;
use serde::{Deserialize, Serialize};

use super::Plugin;

#[derive(Deserialize, Serialize)]
pub struct Portfolio {
    pub summary: PortfolioSummary,
    pub holdings: Vec<Holding>,
}

#[derive(Deserialize, Serialize)]
pub struct PortfolioSummary {
    pub total_market_value: f64,
    pub total_unrealized_pnl: f64,
    pub holdings_count: u64,
}

#[derive(Deserialize, Serialize)]
pub struct Holding {
    pub ticker: String,
    pub name: String,
    pub quantity: f64,
    pub avg_cost: f64,
    pub current_price: f64,
    pub market_value: f64,
    pub unrealized_pnl: f64,
    pub sector: String,
    pub allocation_percent: f64,
}


pub struct PortfolioPlugin {
    pub hostname: String,
}

impl Plugin for PortfolioPlugin {
    fn name(&self) -> &str {
        "portfolio"
    }

    fn register(&self, proto: &mut ProtoJSSandbox) -> Result<()> {
        let hostname = self.hostname.clone();
        proto.register_raw("portfolio", "get", move |_args: String| {
            let url = format!("http://{}/api/v1/portfolio", hostname);

            let response = reqwest::blocking::get(&url)
                .map_err(|_| "failed to perform HTTP request")?;
            if response.status() == 404 {
                return Ok("null".to_string());
            }
            let body = response.text()
                .map_err(|_| "failed to read response body")?;

            let portfolio: Portfolio = serde_json::from_str(&body)
                .map_err(|_| "failed to parse portfolio response")?;
            Ok(serde_json::to_string(&portfolio)?)
        })?;

        Ok(())
    }
}
