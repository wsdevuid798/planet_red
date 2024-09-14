pub struct Map {
    pub width: usize,
    pub height: usize,
    pub data: Vec<Vec<char>>,  // 2D array representing the map
}

impl Map {
    // Generate a new map with default walls and empty spaces
    pub fn new(width: usize, height: usize) -> Map {
        let mut data = vec![vec!['.'; width]; height];  // '.' represents empty space

        // Example of adding some walls (you can improve this to generate random maps)
        for y in 0..height {
            for x in 0..width {
                if x == 0 || y == 0 || x == width - 1 || y == height - 1 {
                    data[y][x] = '#';  // '#' represents a wall
                }
            }
        }

        Map { width, height, data }
    }

    // Update the map with the player's position
    pub fn update_map_with_player(&mut self, player_position: (usize, usize)) {
        let (x, y) = player_position;

        // Ensure the player is within bounds
        if x < self.width && y < self.height {
            self.data[y][x] = 'P';  // 'P' represents the player
        }
    }

    // Get the map as a 2D vector of characters
    pub fn get_map(&self) -> &Vec<Vec<char>> {
        &self.data
    }
}
