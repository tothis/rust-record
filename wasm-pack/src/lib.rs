extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
// 将js函数 导入到rust
extern "C" {
    type HTMLDocument;
    static document: HTMLDocument;
    #[wasm_bindgen(method)]
    fn createElement(this: &HTMLDocument, tagName: &str) -> Element;
    #[wasm_bindgen(method, getter)]
    fn body(this: &HTMLDocument) -> Element;

    type Element;
    #[wasm_bindgen(method, setter = innerHTML)]
    fn set_inner_html(this: &Element, html: &str);
    #[wasm_bindgen(method, js_name = appendChild)]
    fn append_child(this: &Element, other: Element);

    // 导入console.log
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
// 把rust run 函数 导出到js
pub fn run() {
    let val = document.createElement("p");
    val.set_inner_html("rust wasm-pack");
    document.body().append_child(val);
}

#[wasm_bindgen(js_name = onLoad)]
pub fn on_load() {
    log("start");
}