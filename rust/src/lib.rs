use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = document)]
    fn write(s: &str);
}

#[wasm_bindgen(start)]
pub fn main() {
    write("Hello vite & wasm-pack!!");
}
