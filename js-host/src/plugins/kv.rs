use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use anyhow::Result;
use hyperlight_js::ProtoJSSandbox;

use super::Plugin;

pub struct KvPlugin;

impl Plugin for KvPlugin {
    fn name(&self) -> &str {
        "kv"
    }

    fn register(&self, proto: &mut ProtoJSSandbox) -> Result<()> {
        let store: Arc<Mutex<HashMap<String, String>>> = Arc::new(Mutex::new(HashMap::new()));

        let s = Arc::clone(&store);
        proto.register_raw("kv", "set", move |args: String| {
            let parsed: Vec<String> = serde_json::from_str(&args)?;
            let key = parsed.first().cloned().unwrap_or_default();
            let value = parsed.get(1).cloned().unwrap_or_default();
            s.lock().unwrap().insert(key, value);
            Ok(serde_json::to_string(&true)?)
        })?;

        let s = Arc::clone(&store);
        proto.register_raw("kv", "get", move |args: String| {
            let parsed: Vec<String> = serde_json::from_str(&args)?;
            let key = parsed.first().cloned().unwrap_or_default();
            let value = s.lock().unwrap().get(&key).cloned();
            Ok(serde_json::to_string(&value)?)
        })?;

        let s = Arc::clone(&store);
        proto.register_raw("kv", "delete", move |args: String| {
            let parsed: Vec<String> = serde_json::from_str(&args)?;
            let key = parsed.first().cloned().unwrap_or_default();
            let removed = s.lock().unwrap().remove(&key).is_some();
            Ok(serde_json::to_string(&removed)?)
        })?;

        let s = Arc::clone(&store);
        proto.register_raw("kv", "keys", move |_args: String| {
            let keys: Vec<String> = s.lock().unwrap().keys().cloned().collect();
            Ok(serde_json::to_string(&keys)?)
        })?;

        Ok(())
    }
}
