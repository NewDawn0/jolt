mod app;
mod render;
mod types;
use crate::{app::App, types::WasmResult};

pub fn start() -> WasmResult<()> {
    console_error_panic_hook::set_once();
    App::new()?;
    Ok(())
}
