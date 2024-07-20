use chrono::{DateTime, Utc};
use uuid::Uuid;
use rand::{Rng};
use crate::models::Menu::MenuItem;

#[derive(Clone, Debug)]
pub struct MealItem {
    meal_item_id: Uuid,
    menu_item: MenuItem,
    creation_time: DateTime<Utc>,
    update_time: DateTime<Utc>,
    cooking_time_in_min: u32,
    is_removed: bool,
}

impl MealItem {
    pub fn create_meal_item(menu_item: MenuItem) -> MealItem {
        MealItem {
            meal_item_id: Uuid::new_v4(),
            menu_item,
            creation_time: Utc::now(),
            update_time: Utc::now(),
            cooking_time_in_min: rand::thread_rng().gen_range(5..=15),
            is_removed: false,
        }
    }

    pub fn remove_meal_item(&mut self) {
        self.update_time = Utc::now();
        self.is_removed = true;
    }
}