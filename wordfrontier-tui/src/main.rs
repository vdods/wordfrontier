mod app;
mod config;
mod stateful_list;
mod tabs_state;
pub mod ui;

pub use crate::{
    app::App,
    config::Config,
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    // Load the app config
    let config: Config = argh::from_env();
    let db_hub_config = wordfrontier::DbHubConfig::new(
        &config.target_lang_short_name,
        &config.reference_lang_short_name,
        Some("http://localhost:7000".into()),
    )?;
    wordfrontier::DbHub::create_and_populate_missing_databases(&db_hub_config).await?;
    let db_hub = wordfrontier::DbHub::from_config(db_hub_config)?;

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

    let mut app = App::new(config, db_hub);

    terminal.clear()?;

    loop {
        terminal.draw(|f| ui::draw(f, &mut app))?;
        match rx.recv()? {
            // TODO: Just forward KeyCode to App
            Event::Input(event) => match event.code {
                KeyCode::Char('q') => {
                    // TODO: Handle this stuff in an outer function so that it's guaranteed
                    // to run even in the case of an error or panic.
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
                KeyCode::Enter => app.on_enter(),
                KeyCode::Backspace => app.on_backspace(),
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
