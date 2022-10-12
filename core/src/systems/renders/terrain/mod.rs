use gui::dom::{get_canvas, get_document};
use gui::webgl::buffer::init::{
    BufferDataFiller, BufferDataMaker, ColorBufferDataFiller, ColorBufferDataMaker,
    RectangleBufferDataMaker,
};
use gui::webgl::program::get_program;
use specs::{Read, ReadStorage, System};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{HtmlCanvasElement, HtmlInputElement, WebGl2RenderingContext, WebGlProgram};

use std::rc::Rc;

pub struct RenderTerrainSystem {
    pub basics: Rc<gui::GuiBasics>,
}

const CANVAS_ID: &str = "canvas";

impl<'a> System<'a> for RenderTerrainSystem {
    // TODO: resource (time) 사용법 전달드리고 나면, 제거하기 (여기선 필요없음.)
    type SystemData = (
        Read<'a, crate::resources::physics::time::Time>,
        ReadStorage<'a, crate::components::terrain::Terrain>,
    );
    fn run(&mut self, data: Self::SystemData) {
        use specs::Join;
        // MEMO: resource는 이렇게 쓰면 됩니다.
        let (time, terrain) = data;

        for terrain in terrain.join() {
            let document = get_document();
            let canvas = get_canvas(CANVAS_ID);

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

            let context = Rc::new(
                canvas
                    .get_context("webgl2")
                    .unwrap()
                    .unwrap()
                    .dyn_into::<WebGl2RenderingContext>()
                    .unwrap(),
            );
            let program = Rc::new(get_program(&context));
            context.use_program(Some(&program));
            let colorBufferDataMaker = ColorBufferDataMaker { context, program };
            let colorBufferData = colorBufferDataMaker.make_buffer_data(&terrain.bitmap);
            let colorBufferDataFiller = gui::webgl::buffer::init::ColorBufferDataFiller {
                context,
                program,
                buffer_data: Some(colorBufferData),
            };

            colorBufferDataFiller.bind_buffer();
            colorBufferDataFiller.fill_with_buffer_data();

            let rectangleBufferDataMaker = RectangleBufferDataMaker { context, program };
            let rectangleBufferData = rectangleBufferDataMaker.make_buffer_data(&terrain.bitmap);
            let rectangleBufferDataFiller = gui::webgl::buffer::init::RectangleBufferDataFiller {
                context,
                program,
                buffer_data: Some(rectangleBufferData),
            };

            gui::webgl::buffer::update::set_uniform_matrix(&context, &program, &ranges);
            gui::webgl::draw(&context, (rectangleBufferData.len() / 3) as i32);
        }
    }
}
