mod utils;
mod pages;
pub mod virtual_dom;
use wasm_bindgen::prelude::*;

use pages::index;

#[wasm_bindgen(start)]
pub fn run() -> Result <(), JsValue> {
    index::render()?;    
    Ok(())
}