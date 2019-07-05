use crate::{
    input::InputReader,
    rendering::Renderer,
    states::{State, StateManager},
};

/// Represents the application.
pub struct App<I, R> {
    input_reader: I,
    renderer: R,
    state_manager: StateManager,
}

impl<I, R> App<I, R>
where
    I: InputReader,
    R: Renderer,
{
    /// Constructs the application with the given renderer, input reader and an initial state.
    pub fn new(renderer: R, input_reader: I, first_state: Box<dyn State>) -> Self {
        App {
            input_reader,
            renderer,
            state_manager: StateManager::new(first_state),
        }
    }

    /// Starts the application.
    pub fn run(&mut self) {
        loop {
            if let Err(_) = self.state_manager.render(&self.renderer) {}
            if let Some(state) = self.state_manager.get_current_state() {
                if let Ok(i) = self.input_reader.read_input(state.get_input_mode()) {
                    if let Ok(should_quit) = self.state_manager.handle_input(i) {
                        if should_quit {
                            break;
                        }
                        if let Err(_) = self.state_manager.render(&self.renderer) {}
                    }
                }
            }
        }
    }
}
