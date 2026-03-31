use anyhow::Result;
use hyperlight_js::ProtoJSSandbox;
use reqwest::blocking::Client;
use serde::Deserialize;

use super::Plugin;

pub struct FetchPlugin;

#[derive(Deserialize)]
struct FetchArgs {
    url: String,
    #[serde(default)]
    method: Option<String>,
    #[serde(default)]
    body: Option<String>,
    #[serde(default)]
    headers: Option<std::collections::HashMap<String, String>>,
}

#[derive(serde::Serialize)]
struct FetchResponse {
    status: u16,
    body: String,
}

impl Plugin for FetchPlugin {
    fn name(&self) -> &str {
        "fetch"
    }

    fn register(&self, proto: &mut ProtoJSSandbox) -> Result<()> {
        const ALLOWED_URLS: &[&str] = &[
            "https://ipinfo.io/ip",
            // Add more allowed URLs here as needed
        ];

        proto.register_raw("fetch", "fetch", move |args: String| {
            let parsed: FetchArgs = serde_json::from_str(&args)
                .map_err(|e| anyhow::anyhow!("invalid fetch args: {e}"))?;

            // Validate the URL is well-formed
            let url: url::Url = parsed
                .url
                .parse()
                .map_err(|e| anyhow::anyhow!("invalid URL: {e}"))?;

            // Only allow HTTPS (and HTTP for localhost)
            match url.scheme() {
                "https" => {}
                "http" => {
                    let host = url.host_str().unwrap_or_default();
                    if host != "localhost" && host != "127.0.0.1" && host != "::1" {
                        return Err(anyhow::anyhow!("HTTP is only allowed for localhost; use HTTPS for remote hosts").into());
                    }
                }
                other => return Err(anyhow::anyhow!("unsupported scheme: {other}").into()),
            }

            // Only allow the hard-coded URLs
            let url_str = url.as_str().trim_end_matches('/');
            if !ALLOWED_URLS.iter().any(|&allowed| url_str.starts_with(allowed)) {
                return Err(anyhow::anyhow!(
                    "fetch blocked: URL '{url_str}' is not in the allowed list"
                ).into());
            }

            let client = Client::builder()
                .timeout(std::time::Duration::from_secs(10))
                .build()
                .map_err(|e| anyhow::anyhow!("failed to build HTTP client: {e}"))?;

            let method = parsed
                .method
                .as_deref()
                .unwrap_or("GET")
                .to_uppercase();

            let mut request = match method.as_str() {
                "GET" => client.get(url.as_str()),
                "POST" => client.post(url.as_str()),
                "PUT" => client.put(url.as_str()),
                "PATCH" => client.patch(url.as_str()),
                "DELETE" => client.delete(url.as_str()),
                "HEAD" => client.head(url.as_str()),
                other => return Err(anyhow::anyhow!("unsupported HTTP method: {other}").into()),
            };

            if let Some(hdrs) = &parsed.headers {
                for (k, v) in hdrs {
                    request = request.header(k.as_str(), v.as_str());
                }
            }

            if let Some(body) = &parsed.body {
                request = request.body(body.clone());
            }

            let response = request
                .send()
                .map_err(|e| anyhow::anyhow!("fetch failed: {e}"))?;

            let status = response.status().as_u16();
            let body = response
                .text()
                .map_err(|e| anyhow::anyhow!("failed to read response body: {e}"))?;

            Ok(serde_json::to_string(&FetchResponse { status, body })?)
        })?;

        Ok(())
    }
}
