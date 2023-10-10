use crate::game::{GameInfo, GameResult};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Handles the key events and updates the state of [`Game`].
pub fn handle_key_events(key_event: KeyEvent, game: &mut GameInfo) -> GameResult<()> {
    match key_event.code {
        // Exit `ESC` or `q`
        KeyCode::Esc | KeyCode::Char('q') => {
            game.exit();
        }
        // Exit on `Ctrl-C`
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                game.exit();
            }
        }
        // Down on q
        KeyCode::Char('w') => {
            let moved = game.move_key('w');
            game.handle_game_change(moved, 'w')
        }

        // Right of a
        KeyCode::Char('a') => {
            let moved = game.move_key('a');
            game.handle_game_change(moved, 'a')
        }
        //Up on s
        KeyCode::Char('s') => {
            let moved = game.move_key('s');
            game.handle_game_change(moved, 's')
        }
        //Left on d
        KeyCode::Char('d') => {
            let moved = game.move_key('d');
            game.handle_game_change(moved, 'd')
        }

        // Other handlers you could add here.
        _ => {}
    }
    Ok(())
}
