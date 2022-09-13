use web_sys::{WebGl2RenderingContext, WebGlProgram};

pub fn bind_color_buffer(context: &WebGl2RenderingContext, program: &WebGlProgram) {
    let color_buffer = context
        .create_buffer()
        .ok_or("Failed to create buffer")
        .unwrap();
    context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&color_buffer));
    let color_attribute_location = context.get_attrib_location(&program, "a_color");
    context.enable_vertex_attrib_array(color_attribute_location as u32);
    context.vertex_attrib_pointer_with_i32(
        color_attribute_location as u32,
        4,
        WebGl2RenderingContext::FLOAT,
        false,
        0,
        0,
    );
}

pub fn bind_vertex_buffer(context: &WebGl2RenderingContext, program: &WebGlProgram) {
    let buffer = context
        .create_buffer()
        .ok_or("Failed to create buffer")
        .unwrap();
    context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));
    let position_attribute_location = context.get_attrib_location(&program, "a_position");
    context.enable_vertex_attrib_array(position_attribute_location as u32);
    context.vertex_attrib_pointer_with_i32(
        position_attribute_location as u32,
        3,
        WebGl2RenderingContext::FLOAT,
        false,
        0,
        0,
    );
}
