mod events;
mod hazards;
mod inventory;
mod map;
mod player;
mod tui;
mod world;

use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, enable_raw_mode, disable_raw_mode, Clear, ClearType},
    cursor::{Hide, Show},
    event::{self, Event, KeyCode},
};
use ratatui::backend::CrosstermBackend;  // No generic arguments here
use ratatui::Terminal;
use std::io::{self, Stdout};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout: Stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, Hide, Clear(ClearType::All))?;

    let backend = CrosstermBackend::new(stdout);  // No arguments here
    let mut terminal = Terminal::new(backend)?;   // No arguments here

    let mut player = player::Player::new();
    let mut world = world::World::new(&player);

    // Collect log messages as a vector
    let mut log_messages: Vec<String> = Vec::new();

    loop {
        // Get the updated map (Vec<Vec<char>>)
        let map_data = world.get_map();

        // Draw the UI
        terminal.draw(|f| {
            let player_status = format!("Health: {}\nOxygen: {}", player.health, player.oxygen);
            let controls = "W, A, S, D to move; Q to quit";

            // Join the log messages into a single string
            let log_str = log_messages.join("\n");

            // Call the draw_ui function
            tui::draw_ui(f, &player_status, &map_data, &controls, &log_str);
        })?;

        // Handle player input and update log messages
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => break,
                KeyCode::Char('w') => {
                    world.move_player("north", &mut log_messages);
                },
                KeyCode::Char('s') => {
                    world.move_player("south", &mut log_messages);
                },
                KeyCode::Char('a') => {
                    world.move_player("west", &mut log_messages);
                },
                KeyCode::Char('d') => {
                    world.move_player("east", &mut log_messages);
                },
                _ => {}
            }
        }

        // Trigger random events and log them
        events::random_event(&mut player, &mut log_messages);

        // Keep the log size within the limit
        if log_messages.len() > 10 {
            log_messages.remove(0);  // Remove the oldest message
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, Show)?;
    Ok(())
}
