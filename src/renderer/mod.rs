pub mod batch;
pub mod shader;

pub use batch::{Vertex, BatchManager};
use self::shader::Shader;

/// The opengl render for the game
#[derive(Debug)]
pub struct Renderer {
    clear_color: [f32; 4],
    base_shader: Shader,
    batch_manager: BatchManager,
}

impl Renderer {
    /// Construct a new [`Renderer`]
    /// # Safety
    /// The renderer provides no way of assuring opengl functions are loaded as this is handled by the window.
    /// Since its methods utilize opengl functions, constructing it and utilising its methods can lead to UB.
    pub unsafe fn new() -> anyhow::Result<Self> {        
        Ok(Self {
            clear_color: [0.3, 0.3, 0.3, 1.0],
            base_shader: Shader::from_string_unchecked(include_str!("shaders/basic.glsl"))?,
            batch_manager: BatchManager::new(100),
        })
    }

    /// Set the clear color
    pub fn set_clear_color(&mut self, r: f32, g: f32, b: f32, a: f32) {
        self.clear_color = [r, g, b, a];
    }

    /// Bind the base shader
    pub fn bind_base_shader(&self) {
        // Safety: Since using [`Renderer::new`] assumes you've intialized an opengl context it is assumed to be safe.
        unsafe { self.base_shader.bind_unchecked() };
    }

    /// Draw to the screen
    pub fn present(&mut self) {
        // Clear the screen
        unsafe {
            gl::ClearColor(
                self.clear_color[0],
                self.clear_color[1],
                self.clear_color[2],
                self.clear_color[3],
            );
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        self.batch_manager.add_polygon([
            Vertex {
                position: [0.5, -0.5, 0.0],
                color: [1.0, 0.0, 0.0, 1.0],
            },
            Vertex {
                position: [-0.5, -0.5, 0.0],
                color: [0.0, 1.0, 0.0, 1.0],
            },
            Vertex {
                position: [0.0, 0.5, 0.0],
                color: [0.0, 0.0, 1.0, 1.0],
            },
        ]);

        // Draw the batches
        unsafe { self.batch_manager.draw() };
    }
}
