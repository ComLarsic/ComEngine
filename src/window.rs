/// Find the opengl driver
fn find_sdl_gl_driver() -> Option<u32> {
    let driver = sdl2::render::drivers()
        .enumerate()
        .find(|(_, item)| item.name == "opengl");
    driver.map(|(index, _)| index as u32)
}

/// A simple wrapper around an sdl2 window
pub struct Window {
    _sdl: sdl2::Sdl,
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
    event_pump: sdl2::EventPump,

    // The flag for if the window should close
    should_close: bool,
}

impl Window {
    /// Construct a new window
    pub fn new(title: impl Into<String>, width: u32, height: u32) -> anyhow::Result<Self> {
        // Cast the args
        let title: String = title.into();

        // Initialize sdl2
        let sdl = sdl2::init().map_err(|e| anyhow::anyhow!("{}", e))?;

        // Get the video subsystem
        let video = sdl.video().map_err(|e| anyhow::anyhow!("{}", e))?;

        // Create the window
        let window = video
            .window(&title, width, height)
            .opengl()
            .build()
            .map_err(|e| anyhow::anyhow!("{}", e))?;
        // Create the canvas
        let canvas = window
            .into_canvas()
            .index(find_sdl_gl_driver().ok_or(anyhow::anyhow!("Failed to find opengl driver."))?)
            .build()?;

        // Load the opengl function pointers
        gl::load_with(|name| video.gl_get_proc_address(name) as *const _);

        // Set the opengl context to current
        canvas
            .window()
            .gl_set_context_to_current()
            .map_err(|e| anyhow::anyhow!("{}", e))?;

        // Get the event pump
        let event_pump = sdl.event_pump().map_err(|e| anyhow::anyhow!("{}", e))?;

        Ok(Self {
            _sdl: sdl,
            canvas,
            event_pump,
            should_close: false,
        })
    }

    /// Handle the window events
    fn handle_events(&mut self) {
        // Handle sdl events
        for event in self.event_pump.poll_iter() {
            use sdl2::event::Event;

            match event {
                // Handle the quit event
                Event::Quit { .. } => self.should_close = true,
                _ => {}
            };
        }
    }

    /// Check if the window should close
    pub fn should_close(&self) -> bool {
        self.should_close
    }

    /// Present the window
    pub fn present(&mut self) {
        // Present the canvas
        self.canvas.present();
        // Handle window events
        self.handle_events();
    }
}
