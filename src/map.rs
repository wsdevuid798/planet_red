pub fn generate_map() -> Vec<Vec<char>> {
    let mut map = vec![vec!['.'; 10]; 10];  // Create a 10x10 grid with '.' as empty space
    map
}

pub fn update_map_with_player(map: &mut Vec<Vec<char>>, player_position: (usize, usize)) {
    let (x, y) = player_position;
    map[y][x] = '@';  // Mark the player's position with '@'
}
