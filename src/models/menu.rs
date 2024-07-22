use uuid::Uuid;
use crate::models::price::Price;

#[derive(Debug)]
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

#[derive(Clone, Debug, PartialEq)]
pub struct MenuItem {
    menu_item_id: Uuid,
    name: String,
    price: Price,
}

impl MenuItem {
    pub fn new(name: String, price: String) -> Self {
        MenuItem {
            menu_item_id: Uuid::new_v4(),
            name,
            price: Price::from_string(price),
        }
    }

    pub fn create(id: Uuid, name: String, price: String) -> Self {
        MenuItem {
            menu_item_id: id,
            name,
            price: Price::from_string(price),
        }
    }

    pub fn price(&self) -> Price {
        self.price
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}

