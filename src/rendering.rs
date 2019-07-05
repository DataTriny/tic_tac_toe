use crossterm::{ClearType, ErrorKind, Terminal, TerminalColor, TerminalCursor};

pub type CursorPosition = (u16, u16);
pub type ScreenSize = (u16, u16);

#[derive(Clone)]
/// Terminal colors.
pub enum Color {
    Black,
    Blue,
    Cyan,
    DarkBlue,
    DarkCyan,
    DarkGreen,
    DarkGrey,
    DarkMagenta,
    DarkRed,
    DarkYellow,
    Green,
    Grey,
    Magenta,
    Red,
    Reset,
    White,
    Yellow,
}

/// Errors that can occure while rendering to the terminal.
pub enum Error {
    Fmt(std::fmt::Error),
    Io(std::io::Error),
    Other(String),
}

/// Represents a generic terminal renderer.
pub trait Renderer {
    /// Clears all lines.
    fn clear(&self) -> Result<(), Error>;

    /// Only clears the line on which the cursor is standing.
    fn clear_current_line(&self) -> Result<(), Error>;

    /// Gets the cursor coordinates (zero based coordinates).
    fn get_cursor_position(&self) -> CursorPosition;

    /// Gets the size of the terminal.
    fn get_screen_size(&self) -> ScreenSize;

    /// Sets the background color of the terminal.
    fn set_background_color(&self, color: Color) -> Result<(), Error>;

    /// Moves the cursor to the given coordinates.
    fn set_cursor_position(&self, position: CursorPosition) -> Result<(), Error>;

    /// Sets the color of the characters in the terminal.
    fn set_foreground_color(&self, color: Color) -> Result<(), Error>;

    /// Writes some text to the terminal.
    fn write(&self, value: &str) -> Result<usize, Error>;
}

/// A terminal renderer that uses the crossterm crate.
pub struct CrosstermRenderer {
    colored_terminal: TerminalColor,
    cursor: TerminalCursor,
    terminal: Terminal,
}

impl CrosstermRenderer {
    /// Constructs a new crossterm based renderer.
    pub fn new() -> Self {
        CrosstermRenderer {
            colored_terminal: TerminalColor::new(),
            cursor: TerminalCursor::new(),
            terminal: Terminal::new(),
        }
    }
}

impl Renderer for CrosstermRenderer {
    fn clear(&self) -> Result<(), Error> {
        self.terminal
            .clear(ClearType::All)
            .map_err(|e| Error::from(e))
    }

    fn clear_current_line(&self) -> Result<(), Error> {
        self.terminal
            .clear(ClearType::CurrentLine)
            .map_err(|e| Error::from(e))
    }

    fn get_cursor_position(&self) -> CursorPosition {
        self.cursor.pos()
    }

    fn get_screen_size(&self) -> ScreenSize {
        self.terminal.terminal_size()
    }

    fn set_background_color(&self, color: Color) -> Result<(), Error> {
        self.colored_terminal
            .set_bg(crossterm::Color::from(color))
            .map_err(|e| Error::from(e))
    }

    fn set_cursor_position(&self, position: CursorPosition) -> Result<(), Error> {
        self.cursor
            .goto(position.0, position.1)
            .map_err(|e| Error::from(e))
    }

    fn set_foreground_color(&self, color: Color) -> Result<(), Error> {
        self.colored_terminal
            .set_fg(crossterm::Color::from(color))
            .map_err(|e| Error::from(e))
    }

    fn write(&self, value: &str) -> Result<usize, Error> {
        self.terminal.write(value).map_err(|e| Error::from(e))
    }
}

impl From<Color> for crossterm::Color {
    fn from(c: Color) -> crossterm::Color {
        match c {
            Color::Black => crossterm::Color::Black,
            Color::Blue => crossterm::Color::Blue,
            Color::Cyan => crossterm::Color::Cyan,
            Color::DarkBlue => crossterm::Color::DarkBlue,
            Color::DarkCyan => crossterm::Color::DarkCyan,
            Color::DarkGreen => crossterm::Color::DarkGreen,
            Color::DarkGrey => crossterm::Color::DarkGrey,
            Color::DarkMagenta => crossterm::Color::DarkMagenta,
            Color::DarkRed => crossterm::Color::DarkRed,
            Color::DarkYellow => crossterm::Color::DarkYellow,
            Color::Green => crossterm::Color::Green,
            Color::Grey => crossterm::Color::Grey,
            Color::Magenta => crossterm::Color::Magenta,
            Color::Red => crossterm::Color::Red,
            Color::Reset => crossterm::Color::Reset,
            Color::White => crossterm::Color::White,
            Color::Yellow => crossterm::Color::Yellow,
        }
    }
}

impl From<ErrorKind> for Error {
    fn from(e: ErrorKind) -> Error {
        match e {
            ErrorKind::FmtError(f) => Error::Fmt(f),
            ErrorKind::IoError(i) => Error::Io(i),
            ErrorKind::ResizingTerminalFailure(r) => Error::Other(r),
            _ => Error::Other("Unknown error".to_string()),
        }
    }
}
