use color_eyre::eyre::{Ok, Result};
use ratatui::{DefaultTerminal, crossterm::event::{self, Event}};

fn main() -> Result<()> {
    println!("Hello, world!");
    color_eyre::install()?;

    let terminal = ratatui::init();

    let result = run(terminal);

    ratatui::restore();
    result
}


fn run(mut terminal: DefaultTerminal) -> Result<()> {
    loop {
        // Rendering

        // Input Handling
        if let Event::Key(key) = event::read()? {
            match key.code {
                event::KeyCode::Esc => {
                    break;
                }
                _ => {}
            }
        }
    }
    Ok(())
}