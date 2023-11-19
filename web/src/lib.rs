use gloo_utils::format::JsValueSerdeExt;
use js_sys::*;
use mcnbt::{ByteOrder, Tag};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn explore(data: Uint8Array, big_endian: bool) -> Result<JsValue, String> {
    match Tag::from_bytes(
        &data.to_vec(),
        if big_endian {
            ByteOrder::BigEndian
        } else {
            ByteOrder::LittleEndian
        },
    ) {
        Ok(tag) => Ok(JsValue::from_serde(&tag).unwrap()),
        Err(e) => Err(e.to_string()),
    }
}
