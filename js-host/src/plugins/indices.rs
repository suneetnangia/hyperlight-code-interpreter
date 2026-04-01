use anyhow::Result;
use hyperlight_js::ProtoJSSandbox;

use super::Plugin;

pub struct IndicesPlugin {
    pub hostname: String,
}

impl Plugin for IndicesPlugin {
    fn name(&self) -> &str {
        "indices"
    }

    fn register(&self, proto: &mut ProtoJSSandbox) -> Result<()> {
        let hostname = self.hostname.clone();
        proto.register_raw("indices", "get", move |args: String| {
            let parsed: Vec<Option<String>> = serde_json::from_str(&args)?;
            let symbol = parsed.first().cloned().flatten();

            let url = match &symbol {
                Some(s) => format!("http://{}/api/v1/indices/{s}", hostname),
                None => format!("http://{}/api/v1/indices", hostname),
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
