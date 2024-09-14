use crate::map;

pub struct World {
    pub terrain: Vec<Vec<char>>,  // 2D grid for the map
    pub player_position: (usize, usize),  // Player's position
}

impl World {
    pub fn new(player: &crate::player::Player) -> World {
        let mut terrain = map::generate_map();  // Generate initial empty map
        map::update_map_with_player(&mut terrain, (5, 5));  // Set player position initially at (5,5)

        World {
            terrain,
            player_position: (5, 5),
        }
    }

    pub fn move_player(&mut self, direction: &str, log_messages: &mut Vec<String>) {
        let (x, y) = self.player_position;
        let (new_x, new_y) = match direction {
            "north" if y > 0 => (x, y - 1),
            "south" if y < 9 => (x, y + 1),
            "west" if x > 0 => (x - 1, y),
            "east" if x < 9 => (x + 1, y),
            _ => {
                log_messages.push(format!("Player can't move further {}!", direction));
                return;
            }
        };

        // Clear previous player position
        self.terrain[y][x] = '.';
        // Update new player position
        self.terrain[new_y][new_x] = '@';
        self.player_position = (new_x, new_y);
        log_messages.push(format!("Player moved {}", direction));
    }

    // Provide the current terrain to the UI
    pub fn get_map(&self) -> Vec<Vec<char>> {
        self.terrain.clone()
    }
}
