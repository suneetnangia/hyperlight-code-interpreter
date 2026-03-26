use anyhow::Result;
use hyperlight_js::ProtoJSSandbox;

use super::Plugin;

pub struct MathPlugin;

impl Plugin for MathPlugin {
    fn name(&self) -> &str {
        "math"
    }

    fn register(&self, proto: &mut ProtoJSSandbox) -> Result<()> {
        proto.register("math", "sqrt", |x: f64| x.sqrt())?;
        proto.register("math", "pow", |x: f64, y: f64| x.powf(y))?;
        proto.register("math", "abs", |x: f64| x.abs())?;
        proto.register("math", "floor", |x: f64| x.floor())?;
        proto.register("math", "ceil", |x: f64| x.ceil())?;
        proto.register("math", "round", |x: f64| x.round())?;
        proto.register("math", "log", |x: f64| x.ln())?;
        proto.register("math", "min", |a: f64, b: f64| a.min(b))?;
        proto.register("math", "max", |a: f64, b: f64| a.max(b))?;
        Ok(())
    }
}
