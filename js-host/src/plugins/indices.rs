use anyhow::Result;
use hyperlight_js::ProtoJSSandbox;

use super::Plugin;

pub struct IndicesPlugin;

impl Plugin for IndicesPlugin {
    fn name(&self) -> &str {
        "indices"
    }

    fn register(&self, proto: &mut ProtoJSSandbox) -> Result<()> {
        proto.register_raw("indices", "get", move |args: String| {
            let parsed: Vec<Option<String>> = serde_json::from_str(&args)?;
            let symbol = parsed.first().cloned().flatten();

            let url = match &symbol {
                Some(s) => format!("https://127.0.0.1/api/v1/indices/{s}"),
                None => "https://127.0.0.1/api/v1/indices".to_string(),
            };

            let response = reqwest::blocking::get(&url)
                .map_err(|_| "failed to perform HTTP request")?;
            if response.status() == 404 {
                return Ok("null".to_string());
            }
            let body = response.text()
                .map_err(|_| "failed to read response body")?;
            Ok(body)
        })?;

        Ok(())
    }
}
