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
            gui::webgl::buffer::update::update_color(context, &terrain.bitmap);
            gui::webgl::buffer::init::bind_vertex_buffer(context, program);
            gui::webgl::buffer::update::update_rectangle(context, &terrain.bitmap, program, ranges);
        }
    }
}
