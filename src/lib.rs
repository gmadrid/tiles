#![recursion_limit = "256"]

use wasm_bindgen::prelude::*;

macro_rules! app_runner {
    ($func:ident, $app_class:path) => {
        #[wasm_bindgen]
        pub fn $func() -> Result<(), JsValue> {
            yew::start_app::<$app_class>();
            Ok(())
        }
    };
}

mod tiles;

pub use crate::tiles::TilesApp;

app_runner!(run_tiles_app, TilesApp);
