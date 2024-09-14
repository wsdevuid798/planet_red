use ratatui::{
    backend::CrosstermBackend,  // No generic arguments here
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn draw_ui(
    f: &mut Frame,  // No arguments, completely empty!
    player_status: &str,
    map_data: &Vec<Vec<char>>,
    controls: &str,
    log_messages: &str,
) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)].as_ref())
        .split(f.area());

    // Convert the 2D map grid to a string for rendering
    let map_str: String = map_data
        .iter()
        .map(|row| row.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join("\n");

    // Draw the map using Paragraph
    let map_paragraph = Paragraph::new(map_str)
        .block(Block::default().title("Map").borders(Borders::ALL));
    f.render_widget(map_paragraph, chunks[0]);

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
