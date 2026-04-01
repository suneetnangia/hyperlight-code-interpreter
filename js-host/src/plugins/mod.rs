mod indices;
mod portfolio;
mod stocks;

use anyhow::Result;
use hyperlight_js::ProtoJSSandbox;

pub trait Plugin {
    fn name(&self) -> &str;
    fn register(&self, proto: &mut ProtoJSSandbox) -> Result<()>;
}

pub fn all_plugins(indices_hostname: &str, stocks_hostname: &str, portfolio_hostname: &str) -> Vec<Box<dyn Plugin>> {
    vec![
        Box::new(indices::IndicesPlugin {
            hostname: indices_hostname.to_string(),
        }),
        Box::new(stocks::StocksPlugin {
            hostname: stocks_hostname.to_string(),
        }),
        Box::new(portfolio::PortfolioPlugin {
            hostname: portfolio_hostname.to_string(),
        }),
    ]
}
