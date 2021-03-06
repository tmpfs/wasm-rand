use rand::prelude::*;
use rayon::prelude::*;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
pub use wasm_bindgen_rayon::init_thread_pool;

#[cfg(target_arch = "wasm32")]
extern crate wasm_bindgen;

#[cfg(all(test, target_arch = "wasm32"))]
extern crate wasm_bindgen_test;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[cfg(target_arch = "wasm32")]
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
    console_log!("WASM: module started.");
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn thread_rng_random() -> JsValue {
    let values: Vec<u32> = (0..16).into_par_iter()
        .map(|_| {
            let mut rng = rand::thread_rng();
            rng.gen::<u32>()
        })
        .collect();
    JsValue::from_serde(&values).unwrap()
}
