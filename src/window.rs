use glium::{
    backend::glutin::{Display, DisplayCreationError},
    glutin::{
        ContextBuilder, ElementState, Event, EventsLoop, VirtualKeyCode, WindowBuilder, WindowEvent,
    },
};

pub struct Window {
    display: Display,
    dimensions: (f32, f32),
    dimensions_changed: bool,
    pub close_requested: bool,
}

impl Window {
    pub fn from_builder<F>(
        events_loop: &EventsLoop,
        func: F,
    ) -> Result<Window, DisplayCreationError>
    where
        F: FnOnce(WindowBuilder, ContextBuilder) -> (WindowBuilder, ContextBuilder),
    {
        let window_builder = WindowBuilder::new();
        let context_builder = ContextBuilder::new();

        let (window_builder, context_builder) = func(window_builder, context_builder);
        let display = Display::new(window_builder, context_builder, events_loop)?;
        let dims = display.gl_window().get_inner_size().map(|x| (x.width as f32, x.height as f32)).unwrap_or((1.0, 1.0));
        Ok(Window {
            display,
            dimensions: dims,
            dimensions_changed: false,
            close_requested: false,
        })
    }

    pub fn display(&self) -> &Display {
        &self.display
    }

    pub fn get_dimensions_from_display(&self) -> (f32, f32) {
        self.display.gl_window().get_inner_size().map(|x| (x.width as f32, x.height as f32)).unwrap_or((1.0, 1.0))
    }

    pub fn dimensions(&self) -> (f32, f32) {
        self.dimensions
    }

    pub fn changed_dims(&self) -> Option<(f32, f32)> {
        if self.dimensions_changed {
            Some(self.dimensions)
        }
        else {
            None
        }
    }

    pub fn reset(&mut self) {
        self.dimensions_changed = false;
    }

    pub fn handle_event(&mut self, event: &Event) {
        match event {
            Event::WindowEvent { event, .. } => match event {
                
                WindowEvent::CloseRequested => self.close_requested = true,
                WindowEvent::Resized(size) => {
                    self.dimensions = (size.width as f32, size.height as f32);
                    self.dimensions_changed = true;
                },
                _ => {}
            },

            _ => {} ,
        }
    }
}