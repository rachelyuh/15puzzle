use fifteenpuzzle::event::{Event, EventHandler};
use fifteenpuzzle::game::{GameInfo, GameResult, GameState};
use fifteenpuzzle::handler::handle_key_events;
use fifteenpuzzle::tui::Tui;
use std::io;
use tui::backend::CrosstermBackend;
use tui::Terminal;

fn main() -> GameResult<()> {
    // Create an application
    let mut game = GameInfo::new();

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    // Start the main loop
    while game.game_state != GameState::EXIT {
        tui.draw(&mut game)?;
        match tui.events.next()? {
            Event::Tick => game.tick(),
            Event::Key(key_event) => handle_key_events(key_event, &mut game)?,
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }
    }

    tui.exit()?;
    Ok(())
}
