use crate::{
    game::{
        board::{Board, Tile},
        players::Player,
    },
    menus::{Menu, MenuEntry, MenuEntryId, MenuState},
    rendering::{Error, Renderer},
    states::{playing_state::PlayingState, StateTransition},
};

/// The menu displayed when the game ends. Will show the winning combo.
pub struct EndGameMenuState {
    board: Board,
    menu: Menu,
    play_again_entry: MenuEntryId,
    players: Vec<Player>,
    winner: Option<Tile>,
}

impl EndGameMenuState {
    pub fn new(board: Board, players: Vec<Player>, winner: Option<Tile>) -> Self {
        let mut menu = Menu::new();
        let play_again_entry = menu.push(MenuEntry::new("Play again", 1));
        menu.push(MenuEntry::new("Quit", 2));
        EndGameMenuState {
            board,
            menu,
            play_again_entry,
            players,
            winner,
        }
    }
}

impl MenuState for EndGameMenuState {
    fn get_menu(&self) -> &Menu {
        &self.menu
    }

    fn handle_selection(&mut self, entry: MenuEntryId) -> StateTransition {
        if entry == self.play_again_entry {
            return StateTransition::Switch(Box::new(PlayingState::with_players(
                self.players.clone(),
            )));
        }
        StateTransition::Quit
    }

    fn render_header(&self, renderer: &Renderer) -> Result<(), Error> {
        self.board.render(renderer)?;
        renderer.write("\n\n")?;
        if let Some(ref tile) = self.winner {
            tile.render(renderer)?;
            renderer.write(" won!")?;
        } else {
            renderer.write("It's a draw!")?;
        }
        renderer.write("\n\nScores:\n")?;
        for p in self.players.iter() {
            p.render(renderer)?;
            renderer.write("\n")?;
        }
        renderer.write("\nWhat do you want to do now?\n\n")?;
        Ok(())
    }
}
