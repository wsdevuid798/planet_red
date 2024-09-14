use crate::{map, player::Player};

pub struct World {
    pub terrain: Vec<Vec<char>>,
}

impl World {
    pub fn new(player_position: (usize, usize)) -> World {
        // Generate an empty map
        let mut terrain = map::Map::new(20, 15);  // Create a 20x15 map

        // Update the map with the player's initial position
        terrain.update_map_with_player(player_position);

        World { terrain: terrain.data }
    }

    // Get the map
    pub fn get_map(&self) -> &Vec<Vec<char>> {
        &self.terrain
    }

    // Move the player and update the map
    pub fn move_player(&mut self, direction: &str, player: &mut Player) {
        let (x, y) = (player.x as usize, player.y as usize);

        // Clear the player's current position on the map
        self.terrain[y][x] = '.';

        // Update player's position based on the direction
        let new_position = match direction {
            "north" if y > 0 => (x, y - 1),
            "south" if y < self.terrain.len() - 1 => (x, y + 1),
            "west" if x > 0 => (x - 1, y),
            "east" if x < self.terrain[0].len() - 1 => (x + 1, y),
            _ => (x, y),  // No movement if out of bounds
        };

        // Update the player's new position
        player.x = new_position.0 as f64;
        player.y = new_position.1 as f64;

        // Update the map with the player's new position
        self.terrain[new_position.1][new_position.0] = 'P';  // 'P' represents the player
    }
}
