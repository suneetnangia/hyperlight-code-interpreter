mod indices;

use anyhow::Result;
use hyperlight_js::ProtoJSSandbox;

pub trait Plugin {
    fn name(&self) -> &str;
    fn register(&self, proto: &mut ProtoJSSandbox) -> Result<()>;
}

pub fn all_plugins(indices_hostname: &str) -> Vec<Box<dyn Plugin>> {
    vec![
        Box::new(indices::IndicesPlugin {
            hostname: indices_hostname.to_string(),
        }),
    ]
}
