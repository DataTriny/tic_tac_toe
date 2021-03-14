extern crate itertools;

use std::convert::{TryFrom, TryInto};

use itertools::Itertools;

use super::players::Role;
use crate::rendering::{Color, Error, Renderer};

/// Represents board coordinates.
pub type PlayingPosition = (u8, u8);

pub type MagicSquareNumber = u8;

/// Represents the result of the game at a given point in time.
#[derive(Debug, PartialEq)]
pub enum GameResult {
    /// The game is a tie.
    Draw,
    /// The game is not finished yet.
    NotFinished,
    /// The game is finished and one of the players won. Contains the tile of the winning player, as well as the spots that triggered the win.
    Winner(Role, Solution),
}

/// Represents a tile on the board.
#[derive(Clone, Debug, PartialEq)]
pub enum Tile {
    Empty(MagicSquareNumber),
    O(MagicSquareNumber),
    X(MagicSquareNumber),
}

impl Tile {
    /// Renders this tile to the terminal.
    pub fn render(&self, renderer: &Renderer) -> Result<(), Error> {
        match self {
            Tile::Empty(_) => renderer.write(" "),
            Tile::O(_) => renderer.write("o"),
            Tile::X(_) => renderer.write("x"),
        }
        .map(|_| ())
    }
}

/// Represents an array of three adjacent tiles that can lead to a victory.
pub type Solution = [PlayingPosition; 3];

/// Represents a tic-tac-toe board.
#[derive(Clone)]
pub struct Board {
    highlighted_solution: Option<Solution>,
    /// The visual indication of the last played spot.
    pub playing_position: PlayingPosition,
    tiles: Vec<Tile>,
    turns: u8,
}

impl Board {
    /// Constructs a new tic-tac-toe board.
    pub fn new() -> Self {
        Board {
            highlighted_solution: None,
            playing_position: (1, 1),
            tiles: vec![
                Tile::Empty(8), Tile::Empty(1), Tile::Empty(6),
                Tile::Empty(3), Tile::Empty(5), Tile::Empty(7),
                Tile::Empty(4), Tile::Empty(9), Tile::Empty(2)
            ],
            turns: 0,
        }
    }

    /// Computes the current result of the game.
    ///
    /// # Finish result: Winner
    ///
    /// If the player has placed any 3 tiles whose magic square numbers add up to 15.
    ///
    /// See [TicTacToe and Magic Squares - C++ Forum](http://www.cpp.re/forum/general/270825/) for
    ///  the corollary.
    ///
    /// # Finish result: Draw
    ///
    /// If the 9th turn is taken - the board is full.
    pub fn compute_result(&self, for_role: Role) -> GameResult {
        match self
            .tiles
            .iter()
            .enumerate()
            .filter(|(_, tile)| match tile {
                Tile::Empty(_) => false,
                Tile::O(_) => Role::O == for_role,
                Tile::X(_) => Role::X == for_role
            })
            .combinations(3)
            .find(|moves| 15 == moves
                .iter()
                .map(|m| match m.1 {
                    Tile::Empty(magic_square_number) => *magic_square_number,
                    Tile::O(magic_square_number) => *magic_square_number,
                    Tile::X(magic_square_number) => *magic_square_number
                })
                .map(|magic_square_number| magic_square_number as i32)
                .sum()
            )
            .map(|moves| moves
                .iter()
                .map(|m| match u8::try_from(m.0) {
                    Ok(index) => (index % 3, index / 3),
                    // Should not happen
                    Err(_) => (0 as u8, 0 as u8)
                })
                .collect::<Vec<PlayingPosition>>())
        {
            Some(solution) => GameResult::Winner(for_role, solution.try_into().expect("vec with incorrect length")),
            None => if 9 > self.turns {
                GameResult::NotFinished
            } else {
                GameResult::Draw
            }
        }
    }

    /// Gets a tile given its board coordinates.
    pub fn get(&self, x: u8, y: u8) -> &Tile {
        &self.tiles[(y as usize) * 3 + x as usize]
    }

    /// Gets a list of all empty spots on the board.
    pub fn get_available_spots(&self) -> Vec<PlayingPosition> {
        let mut spots = Vec::new();
        for x in 0..3 {
            for y in 0..3 {
                if self.is_empty(x, y) {
                    spots.push((x, y));
                }
            }
        }
        spots
    }

    /// Tells the board to draw a visual indication on a particular solution. Used to show the winning combo.
    pub fn highlight_solution(&mut self, solution: Solution) {
        self.highlighted_solution = Some(solution);
    }

    /// Indicates whether a given spot is empty.
    pub fn is_empty(&self, x: u8, y: u8) -> bool {
        match self.get(x, y) {
            &Tile::Empty(_) => true,
            _ => false
        }
    }

    /// Renders this tic-tac-toe board to the terminal.
    pub fn render(&self, renderer: &Renderer) -> Result<(), Error> {
        let mut highlighting_index = 0;
        for y in 0..3 {
            for x in 0..3 {
                if x > 0 {
                    renderer.write("|")?;
                }
                let reset_background = if let Some(solution) = self.highlighted_solution {
                    let highlighted =
                        if highlighting_index < 3 && solution[highlighting_index] == (x, y) {
                            highlighting_index += 1;
                            renderer.set_background_color(Color::Green)?;
                            true
                        } else {
                            false
                        };
                    highlighted
                } else {
                    false
                };
                self.get(x, y).render(renderer)?;
                if reset_background {
                    renderer.set_background_color(Color::Black)?;
                }
            }
            if y < 2 {
                renderer.write("\n-+-+-\n")?;
            }
        }
        Ok(())
    }

    /// Sets the o or x tile at the given coordinates. Returns the new state of the game.
    pub fn set(&mut self, x: u8, y: u8, for_role: Role) -> GameResult {
        let index = (y as usize) * 3 + x as usize;
        let current_tile = self.tiles[index].clone();
        match current_tile {
            Tile::Empty(magic_square_number) => {
                self.tiles[index] = match for_role {
                    Role::O => Tile::O(magic_square_number),
                    Role::X => Tile::X(magic_square_number)
                };
                self.turns += 1;
                self.compute_result(for_role)
            },
            _ => GameResult::NotFinished
        }
    }

    /// Sets the empty tile at the given coordinates.
    pub fn reset(&mut self, x: u8, y: u8) {
        let index = (y as usize) * 3 + x as usize;
        let current_tile = self.tiles[index].clone();
        self.tiles[index] = match current_tile {
            Tile::Empty(magic_square_number) => Tile::Empty(magic_square_number),
            Tile::O(magic_square_number) => Tile::Empty(magic_square_number),
            Tile::X(magic_square_number) => Tile::Empty(magic_square_number)
        };
        self.turns -= 1;
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryInto;

    use super::{
        Board,
        GameResult,
        super::players::Role,
        Tile,
        WINNING_SOLUTIONS
    };

    /// The list of all possible winning solutions.
    const WINNING_SOLUTIONS: [Solution; 8] = [
        [(0, 0), (1, 0), (2, 0)],
        [(0, 1), (1, 1), (2, 1)],
        [(0, 2), (1, 2), (2, 2)],
        [(0, 0), (0, 1), (0, 2)],
        [(1, 0), (1, 1), (1, 2)],
        [(2, 0), (2, 1), (2, 2)],
        [(0, 0), (1, 1), (2, 2)],
        [(2, 0), (1, 1), (0, 2)]
    ];

    #[test]
    fn compute_result_winner_role_o() {
        for solution in WINNING_SOLUTIONS.iter() {
            let mut board = Board::new();
            let mut game_results = Vec::new();
            let expected_game_results = [
                GameResult::NotFinished,
                GameResult::NotFinished,
                GameResult::Winner(Role::O, *solution)
            ];
            for position in solution.iter() {
                game_results.push(board.set(position.0, position.1, Role::O));
            }
            assert_eq!(expected_game_results.len(), game_results.len());
            assert!(expected_game_results
                .iter()
                .zip(game_results.iter())
                .all(|(expected_game_result, game_result)| expected_game_result == game_result));
        }
    }

    #[test]
    fn compute_result_winner_role_x() {
        for solution in WINNING_SOLUTIONS.iter() {
            let mut board = Board::new();
            let mut game_results = Vec::new();
            let expected_game_results = [
                GameResult::NotFinished,
                GameResult::NotFinished,
                GameResult::Winner(Role::X, *solution)
            ];
            for position in solution.iter() {
                game_results.push(board.set(position.0, position.1, Role::X));
            }
            assert_eq!(expected_game_results.len(), game_results.len());
            assert!(expected_game_results
                .iter()
                .zip(game_results.iter())
                .all(|(expected_game_result, game_result)| expected_game_result == game_result));
        }
    }

    #[test]
    fn compute_result_draw_role_o() {
        let mut board = Board::new();
        board.turns = 7;
        assert_eq!(GameResult::NotFinished, board.set(1, 1, Role::X));
        assert_eq!(GameResult::Draw, board.set(1, 2, Role::O));
    }

    #[test]
    fn compute_result_draw_role_x() {
        let mut board = Board::new();
        board.turns = 7;
        assert_eq!(GameResult::NotFinished, board.set(0, 0, Role::O));
        assert_eq!(GameResult::Draw, board.set(0, 1, Role::X));
    }

    #[test]
    fn set_role_o() {
        let mut board = Board::new();
        board.set(0, 0, Role::O);
        assert_eq!(&Tile::O(8), board.get(0, 0));
    }

    #[test]
    fn set_role_x() {
        let mut board = Board::new();
        board.set(2, 1, Role::X);
        assert_eq!(&Tile::X(7), board.get(2, 1));
    }

    #[test]
    fn set_role_o_increments_turns() {
        let mut board = Board::new();
        board.turns = 7;
        board.set(0, 0, Role::O);
        assert_eq!(8, board.turns);
    }

    #[test]
    fn set_role_x_increments_turns() {
        let mut board = Board::new();
        board.turns = 4;
        board.set(2, 1, Role::X);
        assert_eq!(5, board.turns);
    }

    #[test]
    fn reset_o_tile() {
        let mut board = Board::new();
        board.set(0, 0, Role::O);
        assert_eq!(&Tile::O(8), board.get(0, 0));
        board.reset(0, 0);
        assert_eq!(&Tile::Empty(8), board.get(0, 0));
    }

    #[test]
    fn reset_x_tile() {
        let mut board = Board::new();
        board.set(2, 1, Role::X);
        assert_eq!(&Tile::X(7), board.get(2, 1));
        board.reset(2, 1);
        assert_eq!(&Tile::Empty(7), board.get(2, 1));
    }

    #[test]
    fn reset_empty_tile() {
        let mut board = Board::new();
        board.set(2, 1, Role::X);
        assert_eq!(&Tile::X(7), board.get(2, 1));
        assert_eq!(&Tile::Empty(4), board.get(0, 2));
        board.reset(0, 2);
        assert_eq!(&Tile::Empty(4), board.get(0, 2));
    }

    #[test]
    fn reset_decrements_turns() {
        let mut board = Board::new();
        board.turns = 4;
        board.reset(2, 1);
        assert_eq!(3, board.turns);
    }
}
