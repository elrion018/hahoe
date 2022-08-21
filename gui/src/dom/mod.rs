use wasm_bindgen::JsCast;
use web_sys::{window, Document, HtmlCanvasElement};

pub fn get_document() -> Document {
    let document = window().unwrap().document().unwrap();

    return document;
}

pub fn get_canvas(id: &str) -> HtmlCanvasElement {
    let document = get_document();
    let canvas = document
        .get_element_by_id(id)
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap();

    return canvas;
}
