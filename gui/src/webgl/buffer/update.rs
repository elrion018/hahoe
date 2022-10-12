use nalgebra::Matrix4;

use terrain::model::pixel::Pixel;
use web_sys::{HtmlInputElement, WebGl2RenderingContext, WebGlProgram};

use crate::utils::get_head;

fn get_buffer_data<T>(width: usize, height: usize, size: usize) -> Vec<T> {
    return Vec::<T>::with_capacity(width * height * size);
}

pub fn get_color_buffer_data(bitmap: &Vec<Vec<Pixel>>) -> Vec<f32> {
    let width = bitmap.len() - 1;
    let height = get_head(bitmap).unwrap().len() - 1;
    let mut color_buffer_data = get_buffer_data(width, height, 24);

    for x in 0..width {
        for y in 0..height {
            color_buffer_data.push(0.2f32);
            color_buffer_data.push(bitmap[x][y].height as f32);
            color_buffer_data.push(0.2f32);
            color_buffer_data.push(1.0f32);
            color_buffer_data.push(0.2f32);
            color_buffer_data.push(bitmap[x + 1][y].height as f32);
            color_buffer_data.push(0.2f32);
            color_buffer_data.push(1.0f32);
            color_buffer_data.push(0.2f32);
            color_buffer_data.push(bitmap[x][y + 1].height as f32);
            color_buffer_data.push(0.2f32);
            color_buffer_data.push(1.0f32);
            color_buffer_data.push(0.2f32);
            color_buffer_data.push(bitmap[x][y + 1].height as f32);
            color_buffer_data.push(0.2f32);
            color_buffer_data.push(1.0f32);
            color_buffer_data.push(0.2f32);
            color_buffer_data.push(bitmap[x + 1][y].height as f32);
            color_buffer_data.push(0.2f32);
            color_buffer_data.push(1.0f32);
            color_buffer_data.push(0.2f32);
            color_buffer_data.push(bitmap[x + 1][y + 1].height as f32);
            color_buffer_data.push(0.2f32);
            color_buffer_data.push(1.0f32);
        }
    }

    return color_buffer_data;
}

pub fn get_rectangle_buffer_data(bitmap: &Vec<Vec<Pixel>>) -> Vec<f32> {
    let width = bitmap.len() - 1;
    let height = get_head(bitmap).unwrap().len() - 1;

    let width_ratio = 2. / width as f32;
    let height_ratio = 2. / height as f32;

    let start_width_ratio = -1.;
    let start_height_ratio = -1.;

    let mut rectangle_buffer_data = get_buffer_data(width, height, 24);

    for x in 0..width {
        for y in 0..height {
            let x1 = start_width_ratio + x as f32 * width_ratio;
            let x2 = start_width_ratio + (x + 1) as f32 * width_ratio;
            let y1 = start_height_ratio + y as f32 * height_ratio;
            let y2 = start_height_ratio + (y + 1) as f32 * height_ratio;

            rectangle_buffer_data.append(&mut make_triangle_position(
                x1,
                x2,
                y1,
                y2,
                bitmap[x][y].height as f32,
                bitmap[x + 1][y].height as f32,
                bitmap[x][y + 1].height as f32,
                bitmap[x + 1][y + 1].height as f32,
            ));
        }
    }

    return rectangle_buffer_data;
}

fn check_bitmap_is_empty(bitmap: &Vec<Vec<Pixel>>) -> bool {
    if bitmap.is_empty() || get_head(bitmap).unwrap().is_empty() {
        return true;
    }

    return false;
}

pub fn set_uniform_matrix(
    context: &WebGl2RenderingContext,
    program: &WebGlProgram,
    camera_rad_inputs: &[HtmlInputElement; 4],
) {
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
}
