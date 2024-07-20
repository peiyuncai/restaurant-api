use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct Menu {
    menu_id: Uuid,
    title: String,
    description: String,
    menu_items: Vec<MenuItem>,
}

impl Menu {
    pub fn add_menu_items(&mut self, menu_items: Vec<MenuItem>) -> bool {
        self.menu_items.extend(menu_items);
        true
    }
}

#[derive(Clone, Debug)]
pub struct MenuItem {
    menu_item_id: Uuid,
    title: String,
    description: String,
    price: f64,
}

impl MenuItem {
    pub fn price(&self) -> f64 {
        self.price
    }
}

