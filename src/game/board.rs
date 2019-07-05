use crate::rendering::{Color, Error, Renderer};

/// Represents board coordinates.
pub type PlayingPosition = (u8, u8);

/// Represents the result of the game at a given point in time.
pub enum GameResult {
    /// The game is a tie.
    Draw,
    /// The game is not finished yet.
    NotFinished,
    /// The game is finished and one of the players won. Contains the tile of the winning player, as well as the sspots that triggered the win.
    Winner(Tile, Solution),
}

/// Represents a tile on the board.
#[derive(Clone, Copy, PartialEq)]
pub enum Tile {
    Empty,
    O,
    X,
}

impl Tile {
    /// Renders this tile to the terminal.
    pub fn render(&self, renderer: &Renderer) -> Result<(), Error> {
        match self {
            Tile::Empty => renderer.write(" "),
            Tile::O => renderer.write("o"),
            Tile::X => renderer.write("x"),
        }
        .map(|_| ())
    }
}

/// Represents an array of three adjacent tiles that can lead to a victory.
pub type Solution = [PlayingPosition; 3];

/// The list of all possible winning solutions.
pub const WINNING_SOLUTIONS: [Solution; 8] = [
    [(0, 0), (1, 0), (2, 0)],
    [(0, 1), (1, 1), (2, 1)],
    [(0, 2), (1, 2), (2, 2)],
    [(0, 0), (0, 1), (0, 2)],
    [(1, 0), (1, 1), (1, 2)],
    [(2, 0), (2, 1), (2, 2)],
    [(0, 0), (1, 1), (2, 2)],
    [(2, 0), (1, 1), (0, 2)],
];

/// Represents a tic-tac-toe board.
#[derive(Clone)]
pub struct Board {
    highlighted_solution: Option<Solution>,
    /// The visual indication of the last played spot.
    pub playing_position: PlayingPosition,
    tiles: Vec<Tile>,
    turns: i8,
}

impl Board {
    /// Constructs a new tic-tac-toe board.
    pub fn new() -> Self {
        Board {
            highlighted_solution: None,
            playing_position: (1, 1),
            tiles: vec![Tile::Empty; 9],
            turns: 0,
        }
    }

    /// Computes the current result of the game.
    pub fn compute_result(&self) -> GameResult {
        for solution in WINNING_SOLUTIONS.iter() {
            let tile = self.get(solution[0].0, solution[0].1);
            if tile != &Tile::Empty
                && self.get(solution[1].0, solution[1].1) == tile
                && self.get(solution[2].0, solution[2].1) == tile
            {
                return GameResult::Winner(tile.clone(), *solution);
            }
        }
        if self.turns == 9 {
            return GameResult::Draw;
        }
        GameResult::NotFinished
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
        self.get(x, y) == &Tile::Empty
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

    /// Sets the tile at the given coordinates. Returns the new state of the game.
    pub fn set(&mut self, x: u8, y: u8, tile: Tile) -> GameResult {
        self.tiles[(y as usize) * 3 + x as usize] = tile;
        self.turns += match tile {
            Tile::Empty => -1,
            _ => 1,
        };
        self.compute_result()
    }
}
