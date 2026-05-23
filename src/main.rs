use std::char;

use color_eyre::{eyre::{Ok, Result}, owo_colors::colors::Green};
use ratatui::{DefaultTerminal, Frame, crossterm::event::{self, Event}, layout::{Constraint, Layout}, style::{Color, Style, Stylize}, widgets::{Block, BorderType, List, ListItem, ListState, Paragraph, Widget}};

#[derive(Debug, Default)]
struct AppState {
    items: Vec<TodoItem>,
    list_state: ListState,
}

#[derive(Debug, Default)]
struct TodoItem {
    is_done:bool,
    description: String,
}

fn main() -> Result<()> {
    let mut state = AppState::default();

    state.items.push(TodoItem { 
        is_done: false, 
        description: String::from("Finish application"),
    });
    state.items.push(TodoItem { 
        is_done: false, 
        description: String::from("Finish studies"),
    });
    state.items.push(TodoItem { 
        is_done: false, 
        description: String::from("Finish project"),
    });
    state.items.push(TodoItem { 
        is_done: false, 
        description: String::from("Finish project"),
    });
    state.items.push(TodoItem { 
        is_done: false, 
        description: String::from("Finish project"),
    });

    color_eyre::install()?;

    let terminal = ratatui::init();

    let result = run(terminal, &mut state);

    ratatui::restore();
    result
}


fn run(mut terminal: DefaultTerminal, app_state:&mut AppState) -> Result<()> {
    loop {
        // Rendering
        terminal.draw(|f| render(f, app_state))?;

        // Input Handling
        if let Event::Key(key) = event::read()? {
            if key.kind != event::KeyEventKind::Press {
                continue;
            }
            match key.code {
                event::KeyCode::Esc => {
                    break;
                }
                event::KeyCode::Char(char) => match char {
                    'D' => {
                        if let Some(index) = app_state.list_state.selected() {
                            app_state.items.remove(index);

                            if app_state.items.is_empty() {
                                app_state.list_state.select(None);
                            } else if index >= app_state.items.len() {
                                app_state
                                    .list_state
                                    .select(Some(app_state.items.len()) - 1);
                            }
                        }
                        app_state.list_state.select_next();
                    }
                    'j' => {
                        app_state.list_state.select_next();
                    }
                    'k' => {
                        app_state.list_state.select_previous();
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }
    Ok(())
}

fn render(frame: &mut Frame, app_state: &mut AppState) {
    let [border_area] = Layout::vertical([Constraint::Fill(1)])
        .margin(1)
        .areas(frame.area());
  
    let [inner_area] = Layout::vertical([Constraint::Fill(1)])
        .margin(1)
        .areas(border_area);

    Block::bordered()
        .border_type(BorderType::Rounded)
        .fg(Color::Yellow)
        .render(border_area, frame.buffer_mut());

    let list = List::new(app_state
        .items
        .iter()
        .map(|x| ListItem::from(x.description.as_str()))
    )
    .highlight_symbol(">")
    .highlight_style(Style::default().fg(Color::Green));

    frame.render_stateful_widget(list, inner_area, &mut app_state.list_state);
}