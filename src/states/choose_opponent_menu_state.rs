use crate::{
    game::players::{
        BasicComputerPlayerController, HumanPlayerController, UnbeatableComputerPlayerController,
    },
    menus::{Menu, MenuEntry, MenuEntryId, MenuState},
    rendering::{Error, Renderer},
    states::{playing_state::PlayingState, StateTransition},
};

/// The menu in which the user chooses its opponent.
pub struct ChooseOpponentMenuState {
    against_computer_easy_entry: MenuEntryId,
    against_computer_unbeatable_entry: MenuEntryId,
    against_friend_entry: MenuEntryId,
    menu: Menu,
}

impl ChooseOpponentMenuState {
    pub fn new() -> Self {
        let mut menu = Menu::new();
        let against_friend_entry = menu.push(MenuEntry::new("Against a friend", 1));
        let against_computer_easy_entry =
            menu.push(MenuEntry::new("Against the computer (easy)", 2));
        let against_computer_unbeatable_entry =
            menu.push(MenuEntry::new("Against the computer (unbeatable)", 3));
        menu.push(MenuEntry::new("Go back", 4));
        ChooseOpponentMenuState {
            against_computer_easy_entry,
            against_computer_unbeatable_entry,
            against_friend_entry,
            menu,
        }
    }
}

impl MenuState for ChooseOpponentMenuState {
    fn get_menu(&self) -> &Menu {
        &self.menu
    }

    fn handle_selection(&mut self, entry: MenuEntryId) -> StateTransition {
        if entry == self.against_computer_easy_entry {
            return StateTransition::Switch(Box::new(PlayingState::with_opponent(Box::new(
                BasicComputerPlayerController {},
            ))));
        } else if entry == self.against_computer_unbeatable_entry {
            return StateTransition::Switch(Box::new(PlayingState::with_opponent(Box::new(
                UnbeatableComputerPlayerController {},
            ))));
        } else if entry == self.against_friend_entry {
            return StateTransition::Switch(Box::new(PlayingState::with_opponent(Box::new(
                HumanPlayerController {},
            ))));
        }
        StateTransition::Pop
    }

    fn render_header(&self, renderer: &Renderer) -> Result<(), Error> {
        renderer.write("Who would you like to play against?\n\n")?;
        Ok(())
    }
}
