use super::item::Item;

#[derive(Debug, Clone)]
pub struct Inventory {
    pub items: Vec<Item>,
}

impl Inventory {
    pub fn new() -> Inventory {
        Inventory {
            items: Vec::new(),
        }
    }
    pub fn add(&mut self, item: Item) {
        self.items.push(item);
    }
    pub fn has(&self, item: &Item) -> bool {
        self.items.contains(item)
    }
    pub fn remove(&mut self, item: &Item) -> bool {
        if let Some(pos) = self.items.iter().position(|x| x == item) {
            self.items.remove(pos);
            true
        } else {
            false
        }
    }
}
