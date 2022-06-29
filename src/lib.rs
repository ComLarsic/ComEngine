use prelude::ComCtxBuilder;

pub mod renderer;
pub mod window;
pub mod context;

pub mod prelude {
    pub use crate::*;
    pub use crate::renderer::*;
    pub use crate::renderer::shader::*;
    pub use crate::window::*;
    pub use crate::context::*;
}


/// Initialize comengine and return a context builder
pub fn init() -> ComCtxBuilder {
    ComCtxBuilder::default()
}