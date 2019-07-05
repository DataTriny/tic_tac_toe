mod choose_opponent_menu_state;
mod end_game_menu_state;
pub mod main_menu_state;
mod playing_state;

use crate::{
    input::{InputEvent, InputMode},
    rendering::{Error, Renderer},
};

/// Represents a game state such as the main menu, the playing one and so on.
pub trait State {
    fn get_input_mode(&self) -> InputMode;

    fn handle_input(&mut self, input: InputEvent) -> StateTransition;

    fn render(&self, renderer: &Renderer) -> Result<(), Error>;
}

/// Indicates whether we should close the application.
pub type ShouldQuit = bool;

/// Describes all kinds of error that can occure in the state manager.
pub enum StateManagerError {
    NoState,
    Rendering(Error),
}

/// Represents all possible transitions between states.
pub enum StateTransition {
    /// Nothing happens.
    None,
    /// Removes the most recently open state.
    Pop,
    /// Adds a new state on top of the stack.
    Push(Box<dyn State>),
    /// Exits the application.
    Quit,
    /// Replace the top most state with a new one.
    Switch(Box<dyn State>),
}

/// A state machine.
pub struct StateManager {
    states: Vec<Box<dyn State>>,
}

impl StateManager {
    /// Constructs a new state manager.
    pub fn new(first_state: Box<dyn State>) -> Self {
        StateManager {
            states: vec![first_state],
        }
    }

    /// Gets the state on top of the stack (the one currently shown to the user).
    pub fn get_current_state(&self) -> Option<&Box<dyn State>> {
        self.states.last()
    }

    /// Tells the current state to handle user input.
    pub fn handle_input(&mut self, input: InputEvent) -> Result<ShouldQuit, StateManagerError> {
        if let Some(state) = self.states.last_mut() {
            let transition = state.handle_input(input);
            return Ok(self.handle_transition(transition));
        }
        Err(StateManagerError::NoState)
    }

    fn handle_transition(&mut self, transition: StateTransition) -> ShouldQuit {
        match transition {
            StateTransition::None => {}
            StateTransition::Pop => {
                self.states.pop();
            }
            StateTransition::Push(state) => self.states.push(state),
            StateTransition::Quit => return true,
            StateTransition::Switch(state) => {
                self.states.pop();
                self.states.push(state);
            }
        }
        false
    }

    /// Renders the current state to the terminal.
    pub fn render(&self, renderer: &Renderer) -> Result<(), StateManagerError> {
        if let Some(state) = self.states.last() {
            return state
                .render(renderer)
                .map_err(|e| StateManagerError::Rendering(e));
        }
        Err(StateManagerError::NoState)
    }
}
