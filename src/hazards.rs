use crate::player::Player;

pub enum HazardEvent {
    DustStorm,
    RadiationLeak,
}

impl HazardEvent {
    pub fn trigger(&self, player: &mut Player, log_messages: &mut Vec<String>) {
        match self {
            HazardEvent::DustStorm => {
                // Ensure health and oxygen cannot go below 0
                player.health = player.health.saturating_sub(10);
                player.oxygen = player.oxygen.saturating_sub(5);
                log_messages.push("A dust storm hits! Health and oxygen levels reduced.".to_string());
            }
            HazardEvent::RadiationLeak => {
                // Ensure health doesn't go below 0
                player.health = player.health.saturating_sub(20);
                log_messages.push("Radiation leak detected! Health severely reduced.".to_string());
            }
        }
    }
}
