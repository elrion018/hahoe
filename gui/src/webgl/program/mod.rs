use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlShader, WebGlVertexArrayObject};

use super::shader::get_shaders;

pub fn get_program(context: &WebGl2RenderingContext) -> WebGlProgram {
    let (vert_shader, frag_shader) = get_shaders(&context);

    let program = link_program(&context, &vert_shader, &frag_shader).unwrap();
    context.use_program(Some(&program));

    return program;
}

pub fn get_vertex_array_object(context: &WebGl2RenderingContext) -> WebGlVertexArrayObject {
    let vertex_array_object = context
        .create_vertex_array()
        .ok_or("Could not create vertex array object")
        .unwrap();

    return vertex_array_object;
}

fn link_program(
    context: &WebGl2RenderingContext,
    vert_shader: &WebGlShader,
    frag_shader: &WebGlShader,
) -> Result<WebGlProgram, String> {
    let program = context
        .create_program()
        .ok_or_else(|| String::from("Unable to create shader object"))?;

    context.attach_shader(&program, vert_shader);
    context.attach_shader(&program, frag_shader);
    context.link_program(&program);

    if context
        .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(context
            .get_program_info_log(&program)
            .unwrap_or_else(|| String::from("Unknown error creating program object")))
    }
}
