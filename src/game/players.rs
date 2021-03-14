use crate::{
    game::{
        artificial_intelligence::minimax,
        board::{Board, PlayingPosition},
    },
    input::Key,
    rendering::{Error, Renderer},
};
use rand::Rng;

#[derive(Clone, Debug, PartialEq)]
pub enum Role {
    O,
    X,
}

impl Role {
    /// Renders this role to the terminal.
    pub fn render(&self, renderer: &Renderer) -> Result<(), Error> {
        match self {
            Role::O => renderer.write("o"),
            Role::X => renderer.write("x"),
        }
        .map(|_| ())
    }
}

/// Represents a player.
#[derive(Clone)]
pub struct Player {
    /// The way this player will interact with the board.
    pub controller: Box<dyn PlayerController>,
    /// The current count of wins for this player.
    pub score: usize,
    /// The kind of tile that this player will place on the board.
    pub role: Role,
}

impl Player {
    /// Constructs a new player.
    pub fn new(controller: Box<dyn PlayerController>, role: Role) -> Self {
        Player {
            controller,
            score: 0,
            role,
        }
    }

    /// Renders this player to the terminal.
    pub fn render(&self, renderer: &Renderer) -> Result<(), Error> {
        self.role.render(renderer)?;
        renderer.write(": ")?;
        renderer.write(&self.score.to_string())?;
        Ok(())
    }
}

/// Describes all actions that can be performed by a player.
pub enum PlayerAction {
    /// Moving the cursor.
    Move(PlayingPosition),
    /// Do nothing.
    None,
    /// Places tile on the given spot.
    Play(PlayingPosition),
}

/// A generic player controller.
pub trait PlayerController {
    fn box_clone(&self) -> Box<PlayerController>;

    /// Handles a key press.
    fn handle_key_press(&self, board: &Board, key: Key) -> PlayerAction;

    /// Called when player's turn starts.
    fn start_turn(&self, board: &Board) -> PlayerAction;
}

impl Clone for Box<PlayerController> {
    fn clone(&self) -> Box<PlayerController> {
        self.box_clone()
    }
}

/// A basic computer player that will play a random spot each turn.
#[derive(Clone)]
pub struct BasicComputerPlayerController {}

impl PlayerController for BasicComputerPlayerController {
    fn box_clone(&self) -> Box<PlayerController> {
        Box::new((*self).clone())
    }

    fn handle_key_press(&self, _: &Board, _: Key) -> PlayerAction {
        // Never respond to key presses.
        PlayerAction::None
    }

    fn start_turn(&self, board: &Board) -> PlayerAction {
        let spots = board.get_available_spots();
        let mut rng = rand::thread_rng();
        // Pick a random (but empty) spot.
        PlayerAction::Play(spots[rng.gen_range(0, spots.len())])
    }
}

/// A human controlled player.
#[derive(Clone)]
pub struct HumanPlayerController {}

impl PlayerController for HumanPlayerController {
    fn box_clone(&self) -> Box<PlayerController> {
        Box::new((*self).clone())
    }

    fn handle_key_press(&self, board: &Board, key: Key) -> PlayerAction {
        let pos = board.playing_position;
        match key {
            Key::Char('\n') if board.is_empty(pos.0, pos.1) => PlayerAction::Play(pos),
            Key::Down if pos.1 < 2 => PlayerAction::Move((pos.0, pos.1 + 1)),
            Key::Left if pos.0 > 0 => PlayerAction::Move((pos.0 - 1, pos.1)),
            Key::Right if pos.0 < 2 => PlayerAction::Move((pos.0 + 1, pos.1)),
            Key::Up if pos.1 > 0 => PlayerAction::Move((pos.0, pos.1 - 1)),
            _ => PlayerAction::None,
        }
    }

    fn start_turn(&self, _: &Board) -> PlayerAction {
        // Do not do anything when turn starts.
        PlayerAction::None
    }
}

/// A computer player that uses a minimax algorithm.
#[derive(Clone)]
pub struct UnbeatableComputerPlayerController {}

impl PlayerController for UnbeatableComputerPlayerController {
    fn box_clone(&self) -> Box<PlayerController> {
        Box::new((*self).clone())
    }

    fn handle_key_press(&self, _: &Board, _: Key) -> PlayerAction {
        // Never respond to key presses.
        PlayerAction::None
    }

    fn start_turn(&self, board: &Board) -> PlayerAction {
        let mut temp_board = board.clone();
        // Play the best available move.
        PlayerAction::Play(minimax(&mut temp_board, Role::X).pos)
    }
}
