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
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io::{self, Stdout};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout: Stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, Hide, Clear(ClearType::All))?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Initialize player, map, and inventory
    let mut player = player::Player::new();  // Player with position, health, etc.
    let mut world = world::World::new((5, 5));  // Map and World with player's initial position at (5, 5)
    player.inventory.add_item("Health Potion", 3);  // Add initial items to inventory

    // Collect log messages as a vector
    let mut log_messages: Vec<String> = Vec::new();

    loop {
        // Get the updated map and player position
        let map_data = world.get_map();  // Dynamic map from World struct
        let player_position = player.get_position();  // Get player's current position

        // Get the inventory as a string to display in the UI
        let inventory_display = player.inventory.list_inventory();

        // Draw the UI
        terminal.draw(|f| {
            let player_status = format!("Health: {}\nOxygen: {}", player.health, player.oxygen);
            let controls = "W, A, S, D to move; Q to quit";

            // Join the log messages into a single string
            let log_str = log_messages.join("\n");

            // Call the draw_ui function to render map, inventory, status, and controls
            tui::draw_ui(f, &player_status, &map_data, player_position, &inventory_display, &controls, &log_str);
        })?;

        // Handle player input and update log messages
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => break,
                KeyCode::Char('w') => {
                    world.move_player("north", &mut player);  // Move player north
                    log_messages.push(String::from("Player moved north"));
                },
                KeyCode::Char('s') => {
                    world.move_player("south", &mut player);  // Move player south
                    log_messages.push(String::from("Player moved south"));
                },
                KeyCode::Char('a') => {
                    world.move_player("west", &mut player);   // Move player west
                    log_messages.push(String::from("Player moved west"));
                },
                KeyCode::Char('d') => {
                    world.move_player("east", &mut player);   // Move player east
                    log_messages.push(String::from("Player moved east"));
                },
                _ => {}
            }
        }

        // Trigger random events and log them
        events::random_event(&mut player, &mut log_messages);

        // Limit the log size to 10 messages
        if log_messages.len() > 10 {
            log_messages.remove(0);  // Remove the oldest message
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, Show)?;
    Ok(())
}
