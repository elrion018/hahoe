use specs::{Component, VecStorage};
use terrain::model::pixel::Pixel;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Terrain {
    pub color_buffer_data: Vec<f32>,
    pub rectangle_buffer_data: Vec<f32>,
}
