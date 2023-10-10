use std::time::Instant;
use tui::{
    backend::Backend,
    layout::Alignment,
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use crate::game::{GameInfo, GameState};

/// Renders the user interface widgets (using gui template)
pub fn render<B: Backend>(game: &mut GameInfo, frame: &mut Frame<'_, B>) {
    let time = match game.game_state {
        GameState::STARTED => {
            game.start_time = Instant::now();
            0
        }
        GameState::PLAYING => game.current_time as u64 + game.start_time.elapsed().as_secs(),

        GameState::FINISHED => game.current_time as u64,

        _ => 0,
    };

    frame.render_widget(
        Paragraph::new(format!(
            "\n\
            hello! \n\
            \n\
                press 'w', 'a', 's', 'd' to move around \n\
                press `esc`, `ctrl-c` or `q` to quit :( \n\
                \n\
                Moves: {} \n\
                Time: {} \n\
                \n\
                \n\
                {} \t {} \t {} \t {} \t \n\
                {} \t {} \t {} \t {} \t \n\
                {} \t {} \t {} \t {} \t \n\
                {} \t {} \t {} \t {} \t \n\
                ",
            game.moves,
            time,
            game.numbers[0],
            game.numbers[1],
            game.numbers[2],
            game.numbers[3],
            game.numbers[4],
            game.numbers[5],
            game.numbers[6],
            game.numbers[7],
            game.numbers[8],
            game.numbers[9],
            game.numbers[10],
            game.numbers[11],
            game.numbers[12],
            game.numbers[13],
            game.numbers[14],
            game.numbers[15]
        ))
        .block(
            Block::default()
                .title("Rachel's 15Puzzle")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .alignment(Alignment::Center),
        frame.size(),
    );
}
