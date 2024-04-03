use std::io::Write;

use crossterm::{
    cursor::{Hide, MoveTo},
    terminal::{Clear, ClearType},
    QueueableCommand,
};

pub fn clear_screen() {
    let mut out = std::io::stdout();
    out.queue(Hide).unwrap(); // Optionally hide the cursor
    out.queue(Clear(ClearType::All)).unwrap(); // Clear the screen
    out.queue(MoveTo(0, 0)).unwrap(); // Move the cursor to the top-left corner
    out.flush().unwrap(); // Flush the output to the terminal
}
