use std::rc::Rc;

use terrain::model::pixel::Pixel;
use web_sys::{WebGl2RenderingContext, WebGlProgram};

use crate::utils::get_head;

pub trait BufferDataMaker {
    fn make_buffer_data(&self, bitmap: &Vec<Vec<Pixel>>) -> Vec<f32>;

    fn make_empty_buffer_data<T>(&self, width: usize, height: usize, size: usize) -> Vec<T> {
        return Vec::<T>::with_capacity(width * height * size);
    }
}
pub trait BufferDataFiller {
    fn bind_buffer(&self);

    fn fill_with_buffer_data(&self);
}

pub struct ColorBufferDataMaker {
    pub context: Rc<WebGl2RenderingContext>,
    pub program: Rc<WebGlProgram>,
}

impl ColorBufferDataMaker {
    pub fn new(context: Rc<WebGl2RenderingContext>, program: Rc<WebGlProgram>) -> Self {
        ColorBufferDataMaker { context, program }
    }
}

impl BufferDataMaker for ColorBufferDataMaker {
    fn make_buffer_data(&self, bitmap: &Vec<Vec<Pixel>>) -> Vec<f32> {
        let width = bitmap.len() - 1;
        let height = get_head(bitmap).unwrap().len() - 1;
        let mut buffer_data = self.make_empty_buffer_data(width, height, 24);

        for x in 0..width {
            for y in 0..height {
                buffer_data.push(0.2f32);
                buffer_data.push(bitmap[x][y].height as f32);
                buffer_data.push(0.2f32);
                buffer_data.push(1.0f32);
                buffer_data.push(0.2f32);
                buffer_data.push(bitmap[x + 1][y].height as f32);
                buffer_data.push(0.2f32);
                buffer_data.push(1.0f32);
                buffer_data.push(0.2f32);
                buffer_data.push(bitmap[x][y + 1].height as f32);
                buffer_data.push(0.2f32);
                buffer_data.push(1.0f32);
                buffer_data.push(0.2f32);
                buffer_data.push(bitmap[x][y + 1].height as f32);
                buffer_data.push(0.2f32);
                buffer_data.push(1.0f32);
                buffer_data.push(0.2f32);
                buffer_data.push(bitmap[x + 1][y].height as f32);
                buffer_data.push(0.2f32);
                buffer_data.push(1.0f32);
                buffer_data.push(0.2f32);
                buffer_data.push(bitmap[x + 1][y + 1].height as f32);
                buffer_data.push(0.2f32);
                buffer_data.push(1.0f32);
            }
        }

        return buffer_data;
    }
}

pub struct ColorBufferDataFiller {
    pub context: Rc<WebGl2RenderingContext>,
    pub program: Rc<WebGlProgram>,
    pub buffer_data: Option<Vec<f32>>,
}

impl ColorBufferDataFiller {
    pub fn new(
        context: Rc<WebGl2RenderingContext>,
        program: Rc<WebGlProgram>,
        buffer_data: Option<Vec<f32>>,
    ) -> Self {
        ColorBufferDataFiller {
            context,
            program,
            buffer_data,
        }
    }
}

impl BufferDataFiller for ColorBufferDataFiller {
    fn bind_buffer(&self) {
        let buffer = self
            .context
            .create_buffer()
            .ok_or("Failed to create buffer")
            .unwrap();
        self.context
            .bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));

        let attribute_location = self.context.get_attrib_location(&self.program, "a_color");

        self.context
            .enable_vertex_attrib_array(attribute_location as u32);
        self.context.vertex_attrib_pointer_with_i32(
            attribute_location as u32,
            4,
            WebGl2RenderingContext::FLOAT,
            false,
            0,
            0,
        );
    }

    fn fill_with_buffer_data(&self) {
        unsafe {
            let array_buffer_view = js_sys::Float32Array::view(self.buffer_data.as_ref().unwrap());

            self.context.buffer_data_with_array_buffer_view(
                WebGl2RenderingContext::ARRAY_BUFFER,
                &array_buffer_view,
                WebGl2RenderingContext::STATIC_DRAW,
            );
        }
    }
}

pub struct RectangleBufferDataMaker {
    pub context: Rc<WebGl2RenderingContext>,
    pub program: Rc<WebGlProgram>,
}

impl RectangleBufferDataMaker {
    pub fn new(context: Rc<WebGl2RenderingContext>, program: Rc<WebGlProgram>) -> Self {
        RectangleBufferDataMaker { context, program }
    }
}

impl BufferDataMaker for RectangleBufferDataMaker {
    fn make_buffer_data(&self, bitmap: &Vec<Vec<Pixel>>) -> Vec<f32> {
        let width = bitmap.len() - 1;
        let height = get_head(bitmap).unwrap().len() - 1;

        let width_ratio = 2. / width as f32;
        let height_ratio = 2. / height as f32;

        let start_width_ratio = -1.;
        let start_height_ratio = -1.;

        let mut rectangle_buffer_data = self.make_empty_buffer_data(width, height, 24);

        for x in 0..width {
            for y in 0..height {
                let x1 = start_width_ratio + x as f32 * width_ratio;
                let x2 = start_width_ratio + (x + 1) as f32 * width_ratio;
                let y1 = start_height_ratio + y as f32 * height_ratio;
                let y2 = start_height_ratio + (y + 1) as f32 * height_ratio;
                let h1 = bitmap[x][y].height as f32;
                let h2 = bitmap[x + 1][y].height as f32;
                let h3 = bitmap[x][y + 1].height as f32;
                let h4 = bitmap[x + 1][y + 1].height as f32;

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

                rectangle_buffer_data.append(&mut vertices);
            }
        }

        return rectangle_buffer_data;
    }
}

pub struct RectangleBufferDataFiller {
    pub context: Rc<WebGl2RenderingContext>,
    pub program: Rc<WebGlProgram>,
    pub buffer_data: Option<Vec<f32>>,
}

impl RectangleBufferDataFiller {
    pub fn new(
        context: Rc<WebGl2RenderingContext>,
        program: Rc<WebGlProgram>,
        buffer_data: Option<Vec<f32>>,
    ) -> Self {
        RectangleBufferDataFiller {
            context,
            program,
            buffer_data,
        }
    }
}

impl BufferDataFiller for RectangleBufferDataFiller {
    fn bind_buffer(&self) {
        let buffer = self
            .context
            .create_buffer()
            .ok_or("Failed to create buffer")
            .unwrap();
        self.context
            .bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));
        let position_attribute_location = self
            .context
            .get_attrib_location(&self.program, "a_position");
        self.context
            .enable_vertex_attrib_array(position_attribute_location as u32);
        self.context.vertex_attrib_pointer_with_i32(
            position_attribute_location as u32,
            3,
            WebGl2RenderingContext::FLOAT,
            false,
            0,
            0,
        );
    }

    fn fill_with_buffer_data(&self) {
        unsafe {
            let array_buffer_view = js_sys::Float32Array::view(self.buffer_data.as_ref().unwrap());

            self.context.buffer_data_with_array_buffer_view(
                WebGl2RenderingContext::ARRAY_BUFFER,
                &array_buffer_view,
                WebGl2RenderingContext::STATIC_DRAW,
            );
        }
    }
}
