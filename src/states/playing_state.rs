use crate::{
    game::{
        board::{Board, GameResult},
        players::{HumanPlayerController, Player, PlayerAction, PlayerController, Role},
    },
    input::{InputEvent, InputMode, Key},
    rendering::{Error, Renderer},
    states::{end_game_menu_state::EndGameMenuState, State, StateTransition},
};
use rand::Rng;

/// The playing state.
pub struct PlayingState {
    board: Board,
    current_player: usize,
    players: Vec<Player>,
}

impl PlayingState {
    /// Constructs a playing state with a default human player and an opponent which kind is determined by its controller.
    pub fn with_opponent(opponent_controller: Box<dyn PlayerController>) -> Self {
        PlayingState::with_players(vec![
            Player::new(Box::new(HumanPlayerController {}), Role::O),
            Player::new(opponent_controller, Role::X),
        ])
    }

    /// Constructs a playing state from a list of two existing players. Used to restart the game.
    pub fn with_players(players: Vec<Player>) -> Self {
        let mut rng = rand::thread_rng();
        let mut state = PlayingState {
            board: Board::new(),
            current_player: rng.gen_range(0, 2),
            players,
        };
        state.handle_action(
            state.players[state.current_player]
                .controller
                .start_turn(&state.board),
        );
        state
    }

    fn handle_action(&mut self, action: PlayerAction) -> StateTransition {
        match action {
            PlayerAction::Move(pos) => self.board.playing_position = pos,
            PlayerAction::Play((x, y)) => {
                self.board.playing_position = (x, y);
                match self
                    .board
                    .set(x, y, self.players[self.current_player].role.clone())
                {
                    GameResult::Draw => {
                        return StateTransition::Switch(Box::new(EndGameMenuState::new(
                            self.board.clone(),
                            self.players.clone(),
                            None,
                        )))
                    }
                    GameResult::Winner(role, solution) => {
                        self.board.highlight_solution(solution);
                        return StateTransition::Switch(Box::new(EndGameMenuState::new(
                            self.board.clone(),
                            self.players
                                .iter()
                                .map(|p| {
                                    if p.role == role {
                                        return Player {
                                            controller: p.controller.clone(),
                                            score: p.score + 1,
                                            role: role.clone(),
                                        };
                                    }
                                    p.clone()
                                })
                                .collect::<Vec<Player>>(),
                            Some(role),
                        )));
                    }
                    _ => {
                        self.current_player = (self.current_player + 1) % 2;
                        return self.handle_action(
                            self.players[self.current_player]
                                .controller
                                .start_turn(&self.board),
                        );
                    }
                }
            }
            _ => {}
        }
        StateTransition::None
    }
}

impl State for PlayingState {
    fn get_input_mode(&self) -> InputMode {
        InputMode::Key
    }

    fn handle_input(&mut self, input: InputEvent) -> StateTransition {
        if let InputEvent::Key(k) = input {
            if k == Key::Escape {
                return StateTransition::Quit;
            } else {
                return self.handle_action(
                    self.players[self.current_player]
                        .controller
                        .handle_key_press(&self.board, k),
                );
            }
        }
        StateTransition::None
    }

    fn render(&self, renderer: &Renderer) -> Result<(), Error> {
        renderer.clear()?;
        self.board.render(renderer)?;
        renderer.write("\n\nIt's ")?;
        self.players[self.current_player].role.render(renderer)?;
        renderer.write("'s turn.\n\nScores:\n")?;
        for p in self.players.iter() {
            p.render(renderer)?;
            renderer.write("\n")?;
        }
        renderer.set_cursor_position((
            (self.board.playing_position.0 as u16) * 2,
            (self.board.playing_position.1 as u16) * 2,
        ))
    }
}
