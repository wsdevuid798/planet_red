pub struct Inventory {
    items: Vec<(String, u32)>,  // Each item has a name and a quantity
}

impl Inventory {
    // Create a new empty inventory
    pub fn new() -> Inventory {
        Inventory { items: Vec::new() }
    }

    // Add an item to the inventory
    pub fn add_item(&mut self, item: &str, quantity: u32) {
        // Check if the item already exists in the inventory
        for (name, qty) in &mut self.items {
            if name == item {
                *qty += quantity;
                return;
            }
        }

        // If item doesn't exist, add it as a new entry
        self.items.push((item.to_string(), quantity));
    }

    // Remove an item from the inventory
    pub fn remove_item(&mut self, item: &str, quantity: u32) {
        for (name, qty) in &mut self.items {
            if name == item {
                if *qty > quantity {
                    *qty -= quantity;
                    return;
                }
            }
        }
        self.items.retain(|(_, qty)| *qty > 0);  // Only keep items with quantity > 0
    }

    // Get a string representation of the inventory
    pub fn list_inventory(&self) -> String {
        let mut inventory_list = String::new();
        for (item, qty) in &self.items {
            inventory_list.push_str(&format!("{}: x{}\n", item, qty));
        }
        inventory_list
    }
}
