use nalgebra::Matrix4;

use terrain::model::pixel::Pixel;
use web_sys::{HtmlInputElement, WebGl2RenderingContext, WebGlProgram};

use crate::utils::get_head;

fn get_buffer_array<T>(width: usize, height: usize, size: usize) -> Vec<T> {
    return Vec::<T>::with_capacity(width * height * size);
}

fn check_bitmap_is_empty(bitmap: &Vec<Vec<Pixel>>) -> bool {
    if bitmap.is_empty() || get_head(bitmap).unwrap().is_empty() {
        return true;
    }

    return false;
}

pub fn set_color(context: &WebGl2RenderingContext, bitmap: &Vec<Vec<Pixel>>) {
    if check_bitmap_is_empty(bitmap) {
        return;
    }

    let width = bitmap.len() - 1;
    let height = get_head(bitmap).unwrap().len() - 1;
    let mut color = get_buffer_array(width, height, 24);

    for x in 0..width {
        for y in 0..height {
            color.push(0.2f32);
            color.push(bitmap[x][y].height as f32);
            color.push(0.2f32);
            color.push(1.0f32);
            color.push(0.2f32);
            color.push(bitmap[x + 1][y].height as f32);
            color.push(0.2f32);
            color.push(1.0f32);
            color.push(0.2f32);
            color.push(bitmap[x][y + 1].height as f32);
            color.push(0.2f32);
            color.push(1.0f32);
            color.push(0.2f32);
            color.push(bitmap[x][y + 1].height as f32);
            color.push(0.2f32);
            color.push(1.0f32);
            color.push(0.2f32);
            color.push(bitmap[x + 1][y].height as f32);
            color.push(0.2f32);
            color.push(1.0f32);
            color.push(0.2f32);
            color.push(bitmap[x + 1][y + 1].height as f32);
            color.push(0.2f32);
            color.push(1.0f32);
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

fn bind_array_buffer(context: &WebGl2RenderingContext, array_buffer: Vec<f32>) -> () {
    unsafe {
        let array_buffer_view = js_sys::Float32Array::view(&array_buffer);

        context.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            &array_buffer_view,
            WebGl2RenderingContext::STATIC_DRAW,
        );
    }
}

pub fn set_rectangle(
    context: &WebGl2RenderingContext,
    bitmap: &Vec<Vec<Pixel>>,
    program: &WebGlProgram,
    camera_rad_inputs: &[HtmlInputElement; 4],
) {
    if check_bitmap_is_empty(bitmap) {
        return;
    }

    let width = bitmap.len() - 1;
    let height = get_head(bitmap).unwrap().len() - 1;

    let width_ratio = 2. / width as f32;
    let height_ratio = 2. / height as f32;

    let start_width_ratio = -1.;
    let start_height_ratio = -1.;

    let mut vertices = get_buffer_array(width, height, 24);

    let camera_rads = camera_rad_inputs.clone().map(|node| {
        let value = node.value();
        match value.parse::<f32>() {
            Ok(value) => value,
            Err(_) => 0.0_32,
        }
    });

    let x = (camera_rads[0] - 50.0) / 50.0 * 2.0;
    let y = (camera_rads[1] - 50.0) / 50.0 * 2.0;
    let z = (camera_rads[2] - 50.0) / 50.0 * 2.0;
    let d = camera_rads[3];

    let matrix_location = context.get_uniform_location(program, "u_matrix");

    let default = Matrix4::new(
        1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, d,
    );

    let rotate_x = Matrix4::new(
        1.0,
        0.0,
        0.0,
        0.0,
        0.0,
        x.cos(),
        -x.sin(),
        0.0,
        0.0,
        x.sin(),
        x.cos(),
        0.0,
        0.0,
        0.0,
        0.0,
        1.0,
    );
    let rotate_y = Matrix4::new(
        y.cos(),
        0.0,
        y.sin(),
        0.0,
        0.0,
        1.0,
        0.0,
        0.0,
        -y.sin(),
        0.0,
        y.cos(),
        0.0,
        0.0,
        0.0,
        0.0,
        1.0,
    );

    let rotate_z = Matrix4::new(
        z.cos(),
        -z.sin(),
        0.0,
        0.0,
        z.sin(),
        z.cos(),
        0.0,
        0.0,
        0.0,
        0.0,
        1.0,
        0.0,
        0.0,
        0.0,
        0.0,
        1.0,
    );

    context.uniform_matrix4fv_with_f32_array(
        matrix_location.as_ref(),
        false,
        (&rotate_x * &rotate_y * &rotate_z * &default).as_slice(),
    );

    unsafe {
        let positions_array_buf_view = js_sys::Float32Array::view(&vertices);

        context.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            &positions_array_buf_view,
            WebGl2RenderingContext::STATIC_DRAW,
        );

        let vert_count = (vertices.len() / 3) as i32;
        crate::webgl::draw(&context, vert_count);
    }
}

fn make_triangle_position(
    x1: f32,
    x2: f32,
    y1: f32,
    y2: f32,
    h1: f32,
    h2: f32,
    h3: f32,
    h4: f32,
) -> Vec<f32> {
    let mut vertices = Vec::<f32>::with_capacity(18);
    vertices.push(x1);
    vertices.push(y1);
    vertices.push(h1);
    vertices.push(x2);
    vertices.push(y1);
    vertices.push(h2);
    vertices.push(x1);
    vertices.push(y2);
    vertices.push(h3);
    vertices.push(x1);
    vertices.push(y2);
    vertices.push(h3);
    vertices.push(x2);
    vertices.push(y2);
    vertices.push(h4);
    vertices.push(x2);
    vertices.push(y1);
    vertices.push(h2);
    vertices
}
