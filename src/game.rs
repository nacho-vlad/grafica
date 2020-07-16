use winit::{
    event::*,
    event_loop::{EventLoop, ControlFlow},
    window::WindowBuilder
};
use crate::render::{
    uniforms::Uniforms,
    Render,
    state::GraphicsState,
};
use futures::executor::block_on;


pub trait Game : Render {
    fn update(&mut self);
    fn update_uniforms(&self, uniforms: &mut Uniforms);
    fn handle_events(& mut self, event: &winit::event::WindowEvent) -> bool;
}


pub fn run<G: Game + 'static>(mut game: G) {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .build(&event_loop)
        .unwrap();

    let mut graphics = block_on(GraphicsState::new(&window));

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent { 
                ref event, 
                window_id,
            } if window_id == window.id() => if !game.handle_events(event) {
                match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::KeyboardInput {
                        input,
                        ..
                    } => {
                        match input {
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            } => *control_flow = ControlFlow::Exit,
                            _ => {}
                        }
                    }
                    WindowEvent::Resized(physical_size) => {
                        graphics.resize(*physical_size);
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        graphics.resize(**new_inner_size);
                    }
                    _ => { }
                }
            }
            Event::RedrawRequested(_) => {
                game.update();
                game.update_uniforms(&mut graphics.uniforms);
                game.render(&mut graphics);
            }
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            _ => {}
        }
    });
}


