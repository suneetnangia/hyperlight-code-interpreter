use std::time::Instant;

use hyperlight_js::{SandboxBuilder, Script};

#[test]
fn test_microvm_creation_time() {
    let js_code = "function handler() { return { ok: true }; }\nexport { handler };";

    let start = Instant::now();
    let proto = SandboxBuilder::new().build().expect("failed to build sandbox");
    let build_duration = start.elapsed();

    let start = Instant::now();
    let mut sandbox = proto.load_runtime().expect("failed to load runtime");
    let load_runtime_duration = start.elapsed();

    let start = Instant::now();
    sandbox
        .add_handler("main", Script::from_content(js_code))
        .expect("failed to add handler");
    let mut loaded = sandbox.get_loaded_sandbox().expect("failed to get loaded sandbox");
    let load_sandbox_duration = start.elapsed();

    let start = Instant::now();
    let result = loaded
        .handle_event("main".to_string(), "{}".to_string(), None)
        .expect("failed to handle event");
    let handle_event_duration = start.elapsed();

    let total = build_duration + load_runtime_duration + load_sandbox_duration + handle_event_duration;

    println!("MicroVM creation breakdown:");
    println!("  SandboxBuilder::build()  : {:?}", build_duration);
    println!("  load_runtime()           : {:?}", load_runtime_duration);
    println!("  add_handler + get_loaded : {:?}", load_sandbox_duration);
    println!("  handle_event()           : {:?}", handle_event_duration);
    println!("  Total                    : {:?}", total);
    println!("  Result                   : {}", result);

    // Sanity check: full creation should complete within 10 seconds
    assert!(
        total.as_secs() < 10,
        "MicroVM creation took too long: {:?}",
        total
    );
}
