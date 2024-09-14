use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},  // Correct import for Paragraph
    widgets::canvas::{Canvas, Rectangle},   // Correct import for Canvas and Rectangle
    Frame,
};
use std::f64;

pub fn draw_ui(
    f: &mut Frame,  // No generics for Frame
    player_status: &str,
    map_data: &Vec<Vec<char>>,  // Dynamic map
    player_position: (f64, f64),  // Player position
    inventory: &str,  // Inventory information
    controls: &str,
    log_messages: &str,
) {
    // Divide left area into map and inventory sections
    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())  // Map 70%, Inventory 30%
        .split(f.area());  // Use `.area()` instead of deprecated `.size()`

    // Divide the entire layout into left and right sections
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())  // 50-50 split between map/inventory and status/controls/logs
        .split(f.area());

    // Determine map bounds
    let map_width = map_data[0].len() as f64;
    let map_height = map_data.len() as f64;

    // Render the map using Canvas (localized to player surroundings)
    let map_canvas = Canvas::default()
        .block(Block::default().title("Map").borders(Borders::ALL))
        .paint(|ctx| {
            // Draw dynamic map based on player position and other elements
            for (y, row) in map_data.iter().enumerate() {
                for (x, &cell) in row.iter().enumerate() {
                    let x = x as f64;
                    let y = y as f64;
                    match cell {
                        '#' => ctx.draw(&Rectangle {
                            x,
                            y,
                            width: 1.0,
                            height: 1.0,
                            color: ratatui::style::Color::White,  // Obstacles
                        }),
                        _ => {}  // Empty space
                    }
                }
            }

            // Draw the player dynamically
            let (player_x, player_y) = player_position;
            ctx.draw(&Rectangle {
                x: player_x,
                y: player_y,
                width: 1.0,
                height: 1.0,
                color: ratatui::style::Color::Red,  // Player
            });
        })
        .x_bounds([0.0, map_width])
        .y_bounds([0.0, map_height]);
    f.render_widget(map_canvas, left_chunks[0]);  // Render map in the top 70% of the left area

    // Render the inventory in the bottom 30% of the left area
    let inventory_paragraph = Paragraph::new(inventory)
        .block(Block::default().title("Inventory").borders(Borders::ALL));
    f.render_widget(inventory_paragraph, left_chunks[1]);

    // Divide right side into status, controls, and log messages
    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(30), Constraint::Percentage(30)].as_ref())
        .split(chunks[1]);

    // Draw player status
    let status = Paragraph::new(player_status)
        .block(Block::default().title("Player Status").borders(Borders::ALL));
    f.render_widget(status, right_chunks[0]);

    // Draw controls
    let controls = Paragraph::new(controls)
        .block(Block::default().title("Controls").borders(Borders::ALL));
    f.render_widget(controls, right_chunks[1]);

    // Draw log messages panel
    let log_panel = Paragraph::new(log_messages)
        .block(Block::default().title("Messages").borders(Borders::ALL));
    f.render_widget(log_panel, right_chunks[2]);
}
