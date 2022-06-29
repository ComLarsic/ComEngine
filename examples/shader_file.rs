use comengine::prelude::*;

fn main() -> anyhow::Result<()> {
    // Initialize the context
    let mut ctx = comengine::init()
        .title("Hello, Com!")
        .size(1280, 720)
        .build()?;

    // Construct a shader
    let shader = Shader::from_file(&ctx, "examples/example_shader.glsl")?;
    
    // The update loop
    while !ctx.should_close() {
        // Bind the shader
        ctx.bind_shader(&shader);
        // Draw everything batched using the current shader
        ctx.present();
    }
    
    Ok(())
}
