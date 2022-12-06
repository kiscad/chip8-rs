use crossterm::{
    cursor, execute, queue,
    style::{self, Stylize},
    terminal::{self, Clear, ClearType},
    ExecutableCommand, QueueableCommand, Result,
};
/// cursor
/// event
/// style
/// terminal
use std::io::{stdout, Write};

fn main() {
    // test_lazy_function();
    // test_lazy_macro();
    // test_direct_function();
    // test_direct_macro();
    test_style();
}

fn test_lazy_function() {
    let mut stdout = stdout();
    stdout.queue(cursor::MoveTo(5, 5)).unwrap();
    stdout.flush().unwrap();
}

fn test_lazy_macro() {
    let mut stdout = stdout();
    queue!(stdout, cursor::MoveTo(5, 5), Clear(ClearType::All),).unwrap();
    stdout.flush().unwrap();
}

fn test_direct_function() {
    let mut stdout = stdout();
    stdout.execute(cursor::MoveTo(5, 5)).unwrap();
}

fn test_direct_macro() {
    let mut stdout = stdout();
    execute!(stdout, cursor::MoveTo(5, 5)).unwrap();
}

fn test_style() -> Result<()> {
    let mut stdout = stdout();

    stdout.execute(terminal::Clear(terminal::ClearType::All))?;

    for y in 0..40 {
        for x in 0..100 {
            if (y == 0 || y == 40 - 1) || (x == 0 || x == 100 - 1) {
                stdout
                    .queue(cursor::MoveTo(x, y))?
                    .queue(style::PrintStyledContent("█".magenta()))?;
            }
        }
    }
    stdout.flush()?;
    Ok(())
}
