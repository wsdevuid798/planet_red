pub struct Player {
    pub health: u32,
    pub oxygen: u32,
}

impl Player {
    pub fn new() -> Player {
        Player {
            health: 100,
            oxygen: 100,
        }
    }
}
