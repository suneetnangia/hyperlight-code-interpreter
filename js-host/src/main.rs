mod plugins;

use actix_web::{web, App, HttpResponse, HttpServer};
use anyhow::Result;
use hyperlight_js::{SandboxBuilder, Script};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Clone)]
struct Config {
    indices: IndicesConfig,
    stocks: StocksConfig,
    portfolio: PortfolioConfig,
}

#[derive(Deserialize, Clone)]
struct IndicesConfig {
    hostname: String,
}

#[derive(Deserialize, Clone)]
struct StocksConfig {
    hostname: String,
}

#[derive(Deserialize, Clone)]
struct PortfolioConfig {
    hostname: String,
}

#[derive(Deserialize)]
struct ExecuteRequest {
    /// JavaScript source code to execute
    code: String,
    /// JSON event payload passed to the handler (defaults to `{}`)
    event: Option<serde_json::Value>,
}

#[derive(Serialize)]
struct ExecuteResponse {
    result: serde_json::Value,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

fn run_js(code: &str, event: &str, config: &Config) -> Result<String> {
    let mut proto = SandboxBuilder::new().build()?;

    for plugin in plugins::all_plugins(&config.indices.hostname, &config.stocks.hostname, &config.portfolio.hostname) {
        plugin.register(&mut proto)?;
    }

    let mut sandbox = proto.load_runtime()?;
    sandbox.add_handler("main", Script::from_content(code))?;

    let mut loaded = sandbox.get_loaded_sandbox()?;
    let result = loaded.handle_event("main".to_string(), event.to_string(), None)?;
    Ok(result)
}

async fn execute(
    body: web::Json<ExecuteRequest>,
    config: web::Data<Config>,
) -> HttpResponse {
    let event = body
        .event
        .as_ref()
        .map(|v| v.to_string())
        .unwrap_or_else(|| "{}".to_string());

    let code = body.code.clone();
    let cfg = config.get_ref().clone();

    let result = web::block(move || run_js(&code, &event, &cfg)).await;

    match result {
        Ok(Ok(json_str)) => match serde_json::from_str::<serde_json::Value>(&json_str) {
            Ok(value) => HttpResponse::Ok().json(ExecuteResponse { result: value }),
            Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
                error: format!("failed to parse JS result: {e}"),
            }),
        },
        Ok(Err(e)) => HttpResponse::BadRequest().json(ErrorResponse {
            error: format!("{e}"),
        }),
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: format!("execution panicked: {e}"),
        }),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config_str = std::fs::read_to_string("config.toml")
        .expect("failed to read config.toml");
    let config: Config = toml::from_str(&config_str)
        .expect("failed to parse config.toml");
    let config = web::Data::new(config);

    let bind = std::env::var("BIND_ADDR").unwrap_or_else(|_| "0.0.0.0:8888".to_string());
    println!("Listening on http://{bind}");

    HttpServer::new(move || {
        App::new()
            .app_data(config.clone())
            .route("/execute", web::post().to(execute))
    })
    .bind(&bind)?
    .run()
    .await
}
