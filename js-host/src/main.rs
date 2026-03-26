mod plugins;

use anyhow::Result;
use hyperlight_js::{SandboxBuilder, Script};

fn main() -> Result<()> {
    let js_code = std::env::args().nth(1).unwrap_or_else(|| {
        r#"
import * as math from "math";
import * as time from "time";
import * as kv   from "kv";

function handler(event) {
    const start = time.now_ms();

    // Math plugin: compute hypotenuse & round
    const hyp = math.sqrt(math.pow(event.a, 2) + math.pow(event.b, 2));
    const rounded = math.round(hyp * 100) / 100;

    // KV plugin: store and retrieve values
    kv.set("greeting", "Hello from the VM!");
    kv.set("hypotenuse", String(rounded));
    const greeting = kv.get("greeting");
    const keys = kv.keys();

    // Math plugin: more operations
    const log_val = math.round(math.log(event.a) * 1000) / 1000;
    const clamped = math.max(0, math.min(100, event.a + event.b));

    const elapsed = time.now_ms() - start;

    return {
        math: { hypotenuse: rounded, log_a: log_val, clamped },
        kv:   { greeting, keys },
        time: { elapsed_ms: elapsed, timestamp: time.now_secs() },
    };
}
export { handler };
"#
        .into()
    });

    let event = std::env::args()
        .nth(2)
        .unwrap_or_else(|| r#"{"a": 3, "b": 4}"#.into());

    // Build sandbox — creates a Hyperlight micro-VM with QuickJS inside
    let mut proto = SandboxBuilder::new().build()?;

    // Register host plugins (callable from JS via `import * as X from "host:X"`)
    for plugin in plugins::all_plugins() {
        println!("  ⚙ Registering plugin: {}", plugin.name());
        plugin.register(&mut proto)?;
    }

    // Load the JavaScript runtime into the VM
    let mut sandbox = proto.load_runtime()?;

    // Register the JS code as a handler named "main"
    sandbox.add_handler("main", Script::from_content(&js_code))?;

    // Compile all handlers and get execution-ready sandbox
    let mut loaded = sandbox.get_loaded_sandbox()?;

    // Execute the handler with the event JSON
    let result = loaded.handle_event("main".to_string(), event, None)?;

    // Pretty-print the JSON result
    let pretty: serde_json::Value = serde_json::from_str(&result)?;
    println!("{}", serde_json::to_string_pretty(&pretty)?);

    Ok(())
}
