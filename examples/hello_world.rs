fn main() -> anyhow::Result<()> {
    // Initialize the comgl context
    let mut ctx = comengine::init()
        .title("Hello, world!")
        .size(1280, 720)
        .build()?;

    // The main loop
    while !ctx.should_close() {
        // Update the context and present the frame
        ctx.present();
    }

    Ok(())
}