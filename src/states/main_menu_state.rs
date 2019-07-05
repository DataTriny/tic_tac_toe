use crate::{
    menus::{Menu, MenuEntry, MenuEntryId, MenuState},
    rendering::{Error, Renderer},
    states::{choose_opponent_menu_state::ChooseOpponentMenuState, StateTransition},
};

/// The main menu.
pub struct MainMenuState {
    menu: Menu,
    play_entry: MenuEntryId,
}

impl MainMenuState {
    pub fn new() -> Self {
        let mut menu = Menu::new();
        let play_entry = menu.push(MenuEntry::new("Play", 1));
        menu.push(MenuEntry::new("Quit", 2));
        MainMenuState { menu, play_entry }
    }
}

impl MenuState for MainMenuState {
    fn get_menu(&self) -> &Menu {
        &self.menu
    }

    fn handle_selection(&mut self, entry: MenuEntryId) -> StateTransition {
        if entry == self.play_entry {
            return StateTransition::Push(Box::new(ChooseOpponentMenuState::new()));
        }
        StateTransition::Quit
    }

    fn render_header(&self, renderer: &Renderer) -> Result<(), Error> {
        renderer.write("Tic Tac Toe\n\n")?;
        Ok(())
    }
}
