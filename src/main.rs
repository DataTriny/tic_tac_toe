mod app;
mod game;
mod input;
mod menus;
mod rendering;
mod states;

use app::App;
use input::CrosstermInputReader;
use rendering::CrosstermRenderer;
use states::main_menu_state::MainMenuState;

fn main() {
    App::new(
        CrosstermRenderer::new(),
        CrosstermInputReader::new(),
        Box::new(MainMenuState::new()),
    )
    .run();
}
