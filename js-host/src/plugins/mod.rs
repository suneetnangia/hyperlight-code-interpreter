mod fetch;
mod kv;
mod math;
mod time;

use anyhow::Result;
use hyperlight_js::ProtoJSSandbox;

pub trait Plugin {
    fn name(&self) -> &str;
    fn register(&self, proto: &mut ProtoJSSandbox) -> Result<()>;
}

pub fn all_plugins() -> Vec<Box<dyn Plugin>> {
    vec![
        Box::new(math::MathPlugin),
        Box::new(time::TimePlugin),
        Box::new(kv::KvPlugin),
        Box::new(fetch::FetchPlugin),
    ]
}
