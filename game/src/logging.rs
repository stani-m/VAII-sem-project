use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

#[macro_export]
macro_rules! console_log {
    () => (crate::logging::log("\n"));
    ($($t:tt)*) => (crate::logging::log(&format_args!($($t)*).to_string()));
}
