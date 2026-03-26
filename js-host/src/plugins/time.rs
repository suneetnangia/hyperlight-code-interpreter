use std::time::SystemTime;

use anyhow::Result;
use hyperlight_js::ProtoJSSandbox;

use super::Plugin;

pub struct TimePlugin;

impl Plugin for TimePlugin {
    fn name(&self) -> &str {
        "time"
    }

    fn register(&self, proto: &mut ProtoJSSandbox) -> Result<()> {
        proto.register_raw("time", "now_ms", |_args: String| {
            let ms = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64;
            Ok(serde_json::to_string(&ms)?)
        })?;
        proto.register_raw("time", "now_secs", |_args: String| {
            let secs = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs();
            Ok(serde_json::to_string(&secs)?)
        })?;
        Ok(())
    }
}
