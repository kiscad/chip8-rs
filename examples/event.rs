use crossterm::{
    event::{poll, read, DisableMouseCapture, EnableMouseCapture, Event},
    execute, terminal, Result,
};
use std::io::{stdout, Write};

fn main() {
    terminal::enable_raw_mode().unwrap();

    let mut stdout = stdout();
    execute!(stdout, EnableMouseCapture).unwrap();

    print_events().unwrap();

    execute!(stdout, DisableMouseCapture).unwrap();
    terminal::disable_raw_mode().unwrap();
}

fn print_events1() -> Result<()> {
    loop {
        match read()? {
            Event::FocusGained => println!("FocusGained"),
            Event::FocusLost => println!("FocusLost"),
            Event::Key(event) => println!("{:?}", event),
            Event::Mouse(event) => println!("{:?}", event),
            #[cfg(feature = "bracketed-paste")]
            Event::Paste(data) => println!("{:?}", data),
            Event::Resize(w, h) => println!("New size {}x{}", w, h),
            _ => (),
        }
    }
    Ok(())
}

use std::time::Duration;
fn print_events() -> Result<()> {
    loop {
        if poll(Duration::from_millis(500))? {
            match read()? {
                Event::FocusGained => println!("focus gained"),
                Event::FocusLost => println!("focus lost"),
                Event::Key(event) => println!("{:?}", event),
                Event::Mouse(event) => println!("{:?}", event),
                // #[cfg(feature = "bracketed-paste")]
                Event::Paste(data) => println!("pasted: {:?}", data),
                Event::Resize(w, h) => println!("new size: {}x{}", w, h),
            }
        }
    }
    Ok(())
}
