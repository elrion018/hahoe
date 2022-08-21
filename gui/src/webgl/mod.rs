pub mod buffer;
pub mod program;
pub mod shader;

use web_sys::WebGl2RenderingContext;

pub fn draw(context: &WebGl2RenderingContext, vert_count: i32) {
    context.draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, vert_count);
}

pub fn clear(context: &WebGl2RenderingContext) {
    context.clear_color(0.0, 0.0, 0.0, 1.0);
    context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
}
