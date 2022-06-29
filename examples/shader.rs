use comengine::prelude::*;

fn main() -> anyhow::Result<()> {
    // Initialize the context
    let mut ctx = comengine::init()
        .title("Hello, Com!")
        .size(1280, 720)
        .build()?;

    // Construct a shader
    let shader = Shader::from_string(&ctx, r#"
    #stage vertex
    #version 330 core
    layout(location=0) in vec3 aPos;

    void main() {
        gl_Position = vec4(aPos, 1.0f);
    }

    #stage fragment
    #version 330 core

    out vec4 FragColor;

    void main() {
        FragColor = vec4(1.0f, 0.0f, 0.0f, 1.0f);
    }
    "#)?;
    
    // The update loop
    while !ctx.should_close() {
        // Bind the shader
        ctx.bind_shader(&shader);
        // Draw everything batched using the current shader
        ctx.present();
    }
    
    Ok(())
}
