pub struct Inventory {
    pub items: Vec<String>,
}

impl Inventory {
    pub fn new() -> Inventory {
        Inventory { items: vec![] }
    }

    pub fn add_item(&mut self, item: &str) {
        self.items.push(item.to_string());
    }

    pub fn list_inventory(&self) -> String {
        self.items.join(", ")
    }
}
