pub mod vertex;
pub mod state;
pub mod mesh;
pub mod uniforms;


pub trait Render {
    fn render(&mut self, graphics: &mut state::GraphicsState);
}
