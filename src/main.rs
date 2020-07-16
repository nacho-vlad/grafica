use grafica::game::Game;
use grafica::render::{
    Render,
    state::GraphicsState,
    mesh::Mesh,
    vertex::Vertex,
    uniforms::Uniforms,
};


struct Test {
    mesh: Mesh,

}

impl Test {
    fn new() -> Self {
        Test {
            mesh: Mesh::new( 
                vec![ Vertex::new([ 0.0,  0.05 ,0.0], [1.0,0.0,0.0]),
                      Vertex::new([-0.05, -0.05, 0.0], [0.0,1.0,0.0]),
                      Vertex::new([ 0.05, -0.05, 0.0], [0.0,0.0,1.0]) ],
                vec![0, 1, 2]
            )
        }
    }
}

impl Game for Test {

    fn update(&mut self) {
    }
    
    fn handle_events(&mut self, event: &winit::event::WindowEvent) -> bool {
        false
    }    

    fn update_uniforms(&self, uniforms: &mut Uniforms) {

    }
}

impl Render for Test {

    fn render(&mut self, graphics: &mut GraphicsState) {
        self.mesh.render(graphics);
    }

}



fn main() {
    let test = Test::new();
    grafica::game::run(test);

}
