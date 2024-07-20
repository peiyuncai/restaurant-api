use std::sync::{Arc, Mutex};
use dashmap::DashMap;
use uuid::Uuid;
use crate::models::menu::Menu;

pub struct MenuRepo {
    pub menus: Arc<DashMap<Uuid, Arc<Mutex<Menu>>>>,
}

impl MenuRepo {
    pub fn new() -> Self {
        MenuRepo {
            menus: Arc::new(DashMap::new())
        }
    }

    pub fn get(&self, id: Uuid) -> Option<Menu> {
        self.menus.get(&id).map(|menu_arc| {
            let menu = menu_arc.lock().unwrap();
            menu.clone()
        })
    }

    pub fn add(&self, menu: Menu) {
        let menu_id = menu.id();
        let menu_arc = Arc::new(Mutex::new(menu));
        self.menus.insert(menu_id, menu_arc);
    }
}