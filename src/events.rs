use crate::player::Player;
use crate::hazards::HazardEvent;

pub enum Event {
    Hazard(HazardEvent),
    Other(String),
}

pub fn handle_event(event: Event, player: &mut Player, log_messages: &mut Vec<String>) {
    match event {
        Event::Hazard(hazard) => {
            hazard.trigger(player, log_messages); // Log hazard messages
        }
        Event::Other(description) => {
            log_messages.push(format!("Event: {}", description));  // Log non-hazard events
        }
    }
}

// Example function that triggers random events, including hazards
pub fn random_event(player: &mut Player, log_messages: &mut Vec<String>) {
    let event = if rand::random() {
        Event::Hazard(HazardEvent::DustStorm)
    } else {
        Event::Other("Found a mysterious artifact!".to_string())
    };

    handle_event(event, player, log_messages);
}
