use std::cell::RefCell;
use std::rc::Rc;

use terrain::model::pixel::Pixel;
use terrain::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, HtmlCanvasElement, WebGl2RenderingContext, WebGlProgram, WebGlShader};

const ZERO_HEIGHT: f32 = 0.0;

fn set_color(context: &WebGl2RenderingContext, bitmap: &Vec<Vec<Pixel>>) {
    if bitmap.is_empty() {
        return;
    }
    if bitmap[0].is_empty() {
        return;
    }

    let width = bitmap.len();
    let height = bitmap[0].len();
    let mut color = Vec::<f32>::with_capacity((width * height * 24) as usize);

    for x in 0..width {
        for y in 0..height {
            for _ in 0..6 {
                color.push(0.0f32);
                color.push(bitmap[x][y].height as f32);
                color.push(0.0f32);
                color.push(1.0f32);
            }
        }
    }

    unsafe {
        let color_array_buf_view = js_sys::Float32Array::view(&color);

        context.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            &color_array_buf_view,
            WebGl2RenderingContext::STATIC_DRAW,
        );
    }
}

fn make_triangle_position(x1: f32, x2: f32, y1: f32, y2: f32) -> Vec<f32> {
    let mut vertices = Vec::<f32>::with_capacity(18);
    vertices.push(x1);
    vertices.push(y1);
    vertices.push(ZERO_HEIGHT);
    vertices.push(x2);
    vertices.push(y1);
    vertices.push(ZERO_HEIGHT);
    vertices.push(x1);
    vertices.push(y2);
    vertices.push(ZERO_HEIGHT);
    vertices.push(x1);
    vertices.push(y2);
    vertices.push(ZERO_HEIGHT);
    vertices.push(x2);
    vertices.push(y1);
    vertices.push(ZERO_HEIGHT);
    vertices.push(x2);
    vertices.push(y2);
    vertices.push(ZERO_HEIGHT);
    vertices
}

fn set_rectangle(context: WebGl2RenderingContext, bitmap: &Vec<Vec<Pixel>>) {
    if bitmap.is_empty() {
        return;
    }
    if bitmap[0].is_empty() {
        return;
    }

    let width = bitmap.len();
    let height = bitmap[0].len();

    let width_ratio = 2. / width as f32;
    let height_ratio = 2. / height as f32;

    let start_width_ratio = -1.;
    let start_height_ratio = -1.;

    let mut vertices = Vec::<f32>::with_capacity((width * height * 18) as usize);

    for x in 0..width {
        for y in 0..height {
            let x1 = start_width_ratio + x as f32 * width_ratio;
            let x2 = start_width_ratio + (x + 1) as f32 * width_ratio;
            let y1 = start_height_ratio + y as f32 * height_ratio;
            let y2 = start_height_ratio + (y + 1) as f32 * height_ratio;

            vertices.append(&mut make_triangle_position(x1, x2, y1, y2));
        }
    }

    unsafe {
        let positions_array_buf_view = js_sys::Float32Array::view(&vertices);

        context.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            &positions_array_buf_view,
            WebGl2RenderingContext::STATIC_DRAW,
        );

        let vert_count = (vertices.len() / 3) as i32;

        keep_redraw(context, vert_count);
    }
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) -> () {
    web_sys::window()
        .unwrap()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

fn keep_redraw(context: WebGl2RenderingContext, vert_count: i32) -> () {
    let f = Rc::new(RefCell::new(None));
    let g = Rc::clone(&f);

    let buffer_canvas = get_context(true);

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        request_animation_frame(f.borrow().as_ref().unwrap());
        draw(&context, vert_count);
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());
}

fn get_canvas() -> HtmlCanvasElement {
    let document = window().unwrap().document().unwrap();
    let canvas = document
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap();

    return canvas;
}

fn create_canvas() -> HtmlCanvasElement {
    let document = window().unwrap().document().unwrap();
    let canvas = document
        .create_element("canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap();

    return canvas;
}

fn get_context(is_buffer: bool) -> WebGl2RenderingContext {
    let canvas = if is_buffer {
        create_canvas()
    } else {
        get_canvas()
    };

    let context = canvas
        .get_context("webgl2")
        .unwrap()
        .unwrap()
        .dyn_into::<WebGl2RenderingContext>()
        .unwrap();

    return context;
}

fn create_shaders(context: &WebGl2RenderingContext) -> (WebGlShader, WebGlShader) {
    let vertex_shader = create_vertex_shader(context);
    let fragment_shader = create_fragment_shader(context);

    return (vertex_shader, fragment_shader);
}

fn create_vertex_shader(context: &WebGl2RenderingContext) -> WebGlShader {
    let vertex_shader = compile_shader(
        &context,
        WebGl2RenderingContext::VERTEX_SHADER,
        r##"#version 300 es
 
        in vec4 position;
        in vec4 a_color;

        out vec4 v_color;

        void main() {
        
            gl_Position = position;
            v_color = a_color;
        }
        "##,
    )
    .unwrap();

    return vertex_shader;
}

fn create_fragment_shader(context: &WebGl2RenderingContext) -> WebGlShader {
    let fragment_shader = compile_shader(
        &context,
        WebGl2RenderingContext::FRAGMENT_SHADER,
        r##"#version 300 es
    
        precision highp float;

        in vec4 v_color;
        out vec4 outColor;

        void main() {
            outColor = v_color;
        }
        "##,
    )
    .unwrap();

    return fragment_shader;
}

#[wasm_bindgen]
pub fn start() -> Result<(), JsValue> {
    // view용 context를 가져오고 shader들을 생성한다.
    let context = get_context(false);
    let (vertex_shader, fragment_shader) = create_shaders(&context);

    // 생성한 shader들로 program을 생성하고 이를 context가 사용하게끔 한다.
    let program = link_program(&context, &vertex_shader, &fragment_shader)?;
    context.use_program(Some(&program));

    // vertex array object를 생성하고 이를 context에 bind한다.
    let vao = context
        .create_vertex_array()
        .ok_or("Could not create vertex array object")?;
    context.bind_vertex_array(Some(&vao));

    // 컬러를 위한 버퍼를 생성하고 이를 context에 bind한다.
    let color_buffer = context.create_buffer().ok_or("Failed to create buffer")?;
    context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&color_buffer));

    let color_attribute_location = context.get_attrib_location(&program, "a_color");
    let position_attribute_location = context.get_attrib_location(&program, "position");

    let optbitmap = test_runner1();
    let bitmap = optbitmap.unwrap();
    set_color(&context, &bitmap);

    context.enable_vertex_attrib_array(color_attribute_location as u32);
    context.vertex_attrib_pointer_with_i32(
        color_attribute_location as u32,
        4,
        WebGl2RenderingContext::FLOAT,
        false,
        0,
        0,
    );

    let buffer = context.create_buffer().ok_or("Failed to create buffer")?;
    context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));

    context.enable_vertex_attrib_array(position_attribute_location as u32);
    context.vertex_attrib_pointer_with_i32(
        position_attribute_location as u32,
        3,
        WebGl2RenderingContext::FLOAT,
        false,
        0,
        0,
    );

    context.clear_color(0.0, 0.0, 0.0, 1.0);
    context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

    set_rectangle(context, &bitmap);

    Ok(())
}

fn draw(context: &WebGl2RenderingContext, vert_count: i32) {
    context.draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, vert_count);
}

pub fn compile_shader(
    context: &WebGl2RenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = context
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object"))?;
    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    if context
        .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(context
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unknown error creating shader")))
    }
}

pub fn link_program(
    context: &WebGl2RenderingContext,
    vertex_shader: &WebGlShader,
    fragment_shader: &WebGlShader,
) -> Result<WebGlProgram, String> {
    let program = context
        .create_program()
        .ok_or_else(|| String::from("Unable to create shader object"))?;

    context.attach_shader(&program, vertex_shader);
    context.attach_shader(&program, fragment_shader);
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
