use crate::prelude::{Window, Renderer, shader::Shader};

/// The builder for the ctx
pub struct ComCtxBuilder {
    title: String,
    width: u32,
    height: u32,
}

impl ComCtxBuilder {
    /// Set the title
    pub fn title(mut self, title: impl Into<String>) -> Self {
        let title: String = title.into();
        self.title = title;
        self
    }

    /// Set the size
    pub fn size(mut self, width: u32, height: u32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    /// Construct the builder
    pub fn build(self) -> anyhow::Result<ComCtx> {
        ComCtx::new(self.title, self.width, self.height)
    }
}

impl Default for ComCtxBuilder {
    fn default() -> Self {
        Self { 
            title: "Hello, Com!".into(), 
            width: 1280, 
            height: 720 
        }
    }
}

/// The single-struct context for the com-engine
pub struct ComCtx {
    window: Window,
    renderer: Renderer,
}

impl ComCtx {
    /// Construct a new [`ComCtx`]
    pub fn new(title: impl Into<String>, width: u32, height: u32) -> anyhow::Result<Self> {
        // Create the window and initialize the opengl context
        let window = Window::new(title, width, height)?;
        // Create the renderer
        // Becomes safe since the opengl functions are loaded
        let renderer = unsafe { Renderer::new() }?;

        Ok(Self { window, renderer })
    }

    /// Presents the window and updates the context
    pub fn present(&mut self) {
        // Present the window
        self.window.present();
        // Present the renderer to the screen
        self.renderer.present();
        self.renderer.bind_base_shader();
    }

    /// Check if the window should close
    pub fn should_close(&self) -> bool {
        self.window.should_close()
    }

    // -- Rendering --

    /// Bind a shader
    pub fn bind_shader(&self, shader: &Shader) {
        // Safety: Becomes safe since the ctx ensures the opengl functions are loaded 
        unsafe { shader.bind_unchecked() };
    }
}