use std::rc::Rc;

use wasm_bindgen::{JsCast, JsValue};
use web_sys::{HtmlCanvasElement, HtmlInputElement, WebGl2RenderingContext, WebGlProgram};

use crate::webgl::program::get_vertex_array_object;

pub struct GuiBasics {
    pub canvas: Rc<HtmlCanvasElement>,
    pub context: Rc<WebGl2RenderingContext>,
    pub program: Rc<WebGlProgram>,
    pub ranges: Rc<[HtmlInputElement; 4]>,
}

impl GuiBasics {
    pub fn new() -> Self {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id("canvas").unwrap();
        let ranges = [
            HtmlInputElement::from(JsValue::from(
                document.get_element_by_id("x_range").unwrap(),
            )),
            HtmlInputElement::from(JsValue::from(
                document.get_element_by_id("y_range").unwrap(),
            )),
            HtmlInputElement::from(JsValue::from(
                document.get_element_by_id("z_range").unwrap(),
            )),
            HtmlInputElement::from(JsValue::from(
                document.get_element_by_id("d_range").unwrap(),
            )),
        ];
        let canvas = canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();

        let context = canvas
            .get_context("webgl2")
            .unwrap()
            .unwrap()
            .dyn_into::<WebGl2RenderingContext>()
            .unwrap();

        let program = crate::webgl::program::get_program(&context);
        let vertex_array_object = get_vertex_array_object(&context);

        context.bind_vertex_array(Some(&vertex_array_object));

        GuiBasics {
            canvas: Rc::new(canvas),
            context: Rc::new(context),
            program: Rc::new(program),
            ranges: Rc::new(ranges),
        }
    }
}
