use crate::inventory::Inventory;

pub struct Player {
    pub x: f64,
    pub y: f64,
    pub health: u32,
    pub oxygen: u32,
    pub inventory: Inventory,  // Add inventory field
}

impl Player {
    pub fn new() -> Player {
        Player {
            x: 0.0,  // Initial player position
            y: 0.0,
            health: 100,
            oxygen: 100,
            inventory: Inventory::new(),  // Initialize empty inventory
        }
    }

    // Method to get the player's current position as a tuple
    pub fn get_position(&self) -> (f64, f64) {
        (self.x, self.y)
    }

    // Example method for moving the player north
    pub fn move_north(&mut self) {
        self.y -= 1.0;
    }

    // Example methods for other directions
    pub fn move_south(&mut self) {
        self.y += 1.0;
    }

    pub fn move_east(&mut self) {
        self.x += 1.0;
    }

    pub fn move_west(&mut self) {
        self.x -= 1.0;
    }
}
