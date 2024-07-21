use serde::Deserialize;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct Menu {
    menu_id: Uuid,
    name: String,
    menu_items: Vec<MenuItem>,
}

impl Menu {
    pub fn new(name: String, menu_items: Vec<MenuItem>) -> Self {
        Menu {
            menu_id: Uuid::new_v4(),
            name,
            menu_items,
        }
    }

    pub fn add_menu_items(&mut self, menu_items: Vec<MenuItem>) -> bool {
        self.menu_items.extend(menu_items);
        true
    }

    pub fn id(&self) -> Uuid {
        self.menu_id
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct MenuItem {
    menu_item_id: Uuid,
    name: String,
    price: f64,
}

impl MenuItem {
    pub fn new(name: String, price: f64) -> Self {
        MenuItem {
            menu_item_id: Uuid::new_v4(),
            name,
            price,
        }
    }

    pub fn price(&self) -> f64 {
        self.price
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}

