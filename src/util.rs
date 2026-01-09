use helper::pub_fields;
use wasm_bindgen::JsValue;

pub type WasmResult<T> = Result<T, JsValue>;

#[derive(Default)]
#[pub_fields]
pub struct Pos<T> {
    x: T,
    y: T,
}
impl<T> Pos<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}
