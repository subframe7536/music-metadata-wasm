mod tag;
mod utils;

use tag::{read_tag, write_tag, Metadata};
use utils::set_panic_hook;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = "readTag")]
pub fn read_tag_js(buffer: Vec<u8>) -> Metadata {
    set_panic_hook();
    read_tag(buffer)
}
#[wasm_bindgen(js_name = "writeTag")]
pub fn write_tag_js(buf: Vec<u8>, data: Metadata) -> Vec<u8> {
    set_panic_hook();
    write_tag(buf, data)
}

#[test]
fn test_image_mime() {
    let path = "./samples/mp3.mp3";
    let buf = std::fs::read(path).unwrap();
    let data = read_tag(buf);
    assert_eq!(data.pictures.expect("no pictures")[0].mime_type, "image/png");
}