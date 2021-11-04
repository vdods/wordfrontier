mod app;
mod stateful_list;
mod tabs_state;
pub mod ui;

pub use crate::{
    app::App,
    stateful_list::StatefulList,
    tabs_state::TabsState,
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event as CEvent, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    error::Error,
    io::stdout,
    sync::mpsc,
    thread,
    time::Duration,
};
use tui::{backend::CrosstermBackend, Terminal};

enum Event<I> {
    Input(I),
}

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;

    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);

    let mut terminal = Terminal::new(backend)?;

    // Setup input handling
    let (tx, rx) = mpsc::channel();

    // Effectively have no timeout.
    let event_poll_timeout = Duration::from_secs(1_000_000_000);
    thread::spawn(move || {
        loop {
            if event::poll(event_poll_timeout).unwrap() {
                if let CEvent::Key(key) = event::read().unwrap() {
                    tx.send(Event::Input(key)).unwrap();
                }
            }
        }
    });

    let enhanced_graphics = true;
    let mut app = App::new(" Word Frontier ", enhanced_graphics);

    terminal.clear()?;

    loop {
        terminal.draw(|f| ui::draw(f, &mut app))?;
        match rx.recv()? {
            Event::Input(event) => match event.code {
                KeyCode::Char('q') => {
                    disable_raw_mode()?;
                    execute!(
                        terminal.backend_mut(),
                        LeaveAlternateScreen,
                        DisableMouseCapture
                    )?;
                    terminal.show_cursor()?;
                    break;
                }
                KeyCode::Char(c) => app.on_key(c),
                KeyCode::Left => app.on_left(),
                KeyCode::Up => app.on_up(),
                KeyCode::Right => app.on_right(),
                KeyCode::Down => app.on_down(),
                KeyCode::Tab => app.on_tab(),
                KeyCode::BackTab => app.on_back_tab(),
                KeyCode::F(5) => app.on_reload(),
                _ => {}
            },
        }
        if app.should_quit {
            break;
        }
    }

    Ok(())
}
