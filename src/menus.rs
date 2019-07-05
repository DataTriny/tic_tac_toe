use crate::{
    input::{InputEvent, InputMode},
    rendering::{Color, Error, Renderer},
    states::{State, StateTransition},
};
use textwrap::wrap;

/// Represents an item in a menu.
pub struct MenuEntry {
    /// The color that will be used to draw this entry.
    pub color: Color,
    /// Indicates whether this entry can be triggered. If you don't want to present it to the user, consider setting `is_visible = false` instead.
    pub is_enabled: bool,
    /// Indicates whether this entry will be drawn.
    pub is_visible: bool,
    /// The number that will trigger this entry.
    pub key: usize,
    /// The text to display.
    pub text: String,
}

impl MenuEntry {
    /// Constructs a new menu entry with the given text and key.
    pub fn new<S>(text: S, key: usize) -> Self
    where
        S: Into<String>,
    {
        MenuEntry {
            color: Color::White,
            is_enabled: true,
            is_visible: true,
            key,
            text: text.into(),
        }
    }

    /// Renders this entry to the terminal.
    fn render(&self, renderer: &Renderer) -> Result<(), Error> {
        if self.is_visible {
            let margin = if self.is_enabled {
                let key_string = self.key.to_string();
                renderer.set_foreground_color(self.color.clone())?;
                renderer.write(&key_string)?;
                renderer.write(") ")?;
                key_string.len() + 2
            } else {
                renderer.set_foreground_color(Color::Red)?;
                renderer.write("x) ")?;
                renderer.set_foreground_color(self.color.clone())?;
                4
            };
            let indent_string = (0..margin).map(|_| " ").collect::<String>();
            for (index, line) in wrap(&self.text, (renderer.get_screen_size().0 as usize) - margin)
                .iter()
                .enumerate()
            {
                if index > 0 {
                    renderer.write(&indent_string)?;
                }
                renderer.write(line)?;
                renderer.write("\n")?;
            }
        }
        Ok(())
    }
}

/// Represents a menu entry identifier that is guarantied to be unique across a menu.
#[derive(Clone, Copy, PartialEq)]
pub struct MenuEntryId(usize);

/// Represents a menu.
pub struct Menu {
    entries: Vec<(MenuEntryId, MenuEntry)>,
    next_id: usize,
}

impl Menu {
    /// Constructs a new menu.
    pub fn new() -> Self {
        Menu {
            entries: Vec::new(),
            next_id: 0,
        }
    }

    /// Handles input for this menu. It will return an identifier to the choosen entry, or `None` if it failed to parse user input.
    pub fn handle_input(&self, input: &str) -> Option<MenuEntryId> {
        if let Ok(key) = input.parse::<usize>() {
            if let Some(entry) = self.entries.iter().filter(|m| m.1.key == key).next() {
                if entry.1.is_enabled && entry.1.is_visible {
                    return Some(entry.0);
                }
            }
        }
        None
    }

    /// Pushes a new entry to this menu. Returns the newly created entry identifier.
    pub fn push(&mut self, entry: MenuEntry) -> MenuEntryId {
        let id = MenuEntryId(self.next_id);
        self.entries.push((id, entry));
        self.next_id += 1;
        id
    }

    /// Renders this menu to the terminal.
    fn render(&self, renderer: &Renderer) -> Result<(), Error> {
        for e in self.entries.iter() {
            e.1.render(renderer)?;
        }
        Ok(())
    }
}

/// A convenient trait to create states that only deal with a menu.
pub trait MenuState: State {
    /// Should return a reference to the menu.
    fn get_menu(&self) -> &Menu;

    /// Used to handle user's choice.
    fn handle_selection(&mut self, entry: MenuEntryId) -> StateTransition;

    /// Used to render something on top of the screen.
    fn render_header(&self, renderer: &Renderer) -> Result<(), Error>;
}

impl<T> State for T
where
    T: MenuState,
{
    fn get_input_mode(&self) -> InputMode {
        InputMode::Line
    }

    fn handle_input(&mut self, input: InputEvent) -> StateTransition {
        if let InputEvent::Line(ref input) = input {
            if let Some(entry) = self.get_menu().handle_input(input) {
                return self.handle_selection(entry);
            }
        }
        StateTransition::None
    }

    fn render(&self, renderer: &Renderer) -> Result<(), Error> {
        renderer.clear()?;
        self.render_header(renderer)?;
        self.get_menu().render(renderer)
    }
}
