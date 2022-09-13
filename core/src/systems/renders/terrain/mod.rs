use specs::{Read, ReadStorage, System};
use std::rc::Rc;

pub struct RenderTerrainSystem {
    pub basics: Rc<gui::GuiBasics>,
}

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
            let gui::GuiBasics {
                canvas: _,
                program,
                ranges,
                context,
            } = self.basics.as_ref();

            gui::webgl::buffer::init::bind_color_buffer(context, program);
            let color_buffer_data =
                gui::webgl::buffer::update::get_color_buffer_data(&terrain.bitmap);
            gui::webgl::buffer::update::fill_buffer_data(context, &color_buffer_data);

            gui::webgl::buffer::init::bind_vertex_buffer(context, program);
            let rectangle_array_buffer =
                gui::webgl::buffer::update::get_rectangle_buffer_data(&terrain.bitmap);
            gui::webgl::buffer::update::fill_buffer_data(context, &rectangle_array_buffer);

            gui::webgl::buffer::update::set_uniform_matrix(context, program, ranges);
            gui::webgl::draw(context, (rectangle_array_buffer.len() / 3) as i32);
        }
    }
}
