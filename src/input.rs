use crossterm::{KeyEvent, RawScreen, TerminalInput};

/// The keyboard keys.
#[derive(Clone, PartialEq)]
pub enum Key {
    Alt(char),
    Backspace,
    Char(char),
    Ctrl(char),
    Down,
    End,
    Escape,
    F(u8),
    Home,
    Left,
    PageDown,
    PageUp,
    Right,
    Unknown,
    Up,
}

/// The types of events that can be received by the application.
pub enum InputEvent {
    Key(Key),
    Line(String),
}

/// The kind of input that a given state handles.
pub enum InputMode {
    Key,
    Line,
}

/// A generic input reader.
pub trait InputReader {
    /// Emits input events based on the input mode.
    fn read_input(&self, mode: InputMode) -> Result<InputEvent, std::io::Error> {
        match mode {
            InputMode::Key => Ok(InputEvent::Key(self.read_key())),
            InputMode::Line => self.read_line().map(|l| InputEvent::Line(l)),
        }
    }

    /// Reads a keyboard key.
    fn read_key(&self) -> Key;

    /// Reads an entire line of text.
    fn read_line(&self) -> Result<String, std::io::Error>;
}

/// A crossterm based input reader.
pub struct CrosstermInputReader {
    input: TerminalInput,
}

impl CrosstermInputReader {
    pub fn new() -> Self {
        CrosstermInputReader {
            input: TerminalInput::new(),
        }
    }
}

impl InputReader for CrosstermInputReader {
    fn read_key(&self) -> Key {
        if let Ok(_) = RawScreen::into_raw_mode() {
            loop {
                let mut reader = self.input.read_sync();
                if let Some(crossterm::InputEvent::Keyboard(k)) = reader.next() {
                    if k != KeyEvent::Null {
                        return Key::from(k);
                    }
                }
            }
        }
        Key::Unknown
    }

    fn read_line(&self) -> Result<String, std::io::Error> {
        self.input.read_line()
    }
}

impl From<KeyEvent> for Key {
    fn from(k: KeyEvent) -> Key {
        match k {
            KeyEvent::Alt(c) => Key::Alt(c),
            KeyEvent::Backspace => Key::Backspace,
            KeyEvent::Char(c) => Key::Char(c),
            KeyEvent::Ctrl(c) => Key::Ctrl(c),
            KeyEvent::Down => Key::Down,
            KeyEvent::End => Key::End,
            KeyEvent::Esc => Key::Escape,
            KeyEvent::F(f) => Key::F(f),
            KeyEvent::Home => Key::Home,
            KeyEvent::Left => Key::Left,
            KeyEvent::PageDown => Key::PageDown,
            KeyEvent::PageUp => Key::PageUp,
            KeyEvent::Right => Key::Right,
            KeyEvent::Up => Key::Up,
            _ => Key::Unknown,
        }
    }
}
