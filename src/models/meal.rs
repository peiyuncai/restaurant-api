use chrono::{DateTime, Utc};
use uuid::Uuid;
use rand::{Rng};
use crate::models::menu::MenuItem;

#[derive(Clone, Debug)]
pub enum MealItemStatus {
    Received,
    Preparing,
    Completed,
}

#[derive(Clone, Debug)]
pub struct MealItem {
    meal_item_id: Uuid,
    menu_item: MenuItem,
    creation_time: DateTime<Utc>,
    update_time: DateTime<Utc>,
    cooking_time_in_min: u32,
    is_removed: bool,
    status: MealItemStatus,
}

impl MealItem {
    pub fn create(menu_item: MenuItem) -> MealItem {
        MealItem {
            meal_item_id: Uuid::new_v4(),
            menu_item,
            creation_time: Utc::now(),
            update_time: Utc::now(),
            cooking_time_in_min: rand::thread_rng().gen_range(5..=15),
            is_removed: false,
            status: MealItemStatus::Received,
        }
    }

    pub fn remove(&mut self) {
        self.update_time = Utc::now();
        self.is_removed = true;
    }

    pub fn id(&self) -> Uuid {
        self.meal_item_id
    }

    pub fn cooking_time_in_min(&self) -> u32 {
        self.cooking_time_in_min
    }

    pub fn price(&self) -> f64 {
        self.menu_item.price()
    }

    pub fn is_removed(&self) -> bool {
        self.is_removed
    }

    pub fn update_state(&mut self, status: MealItemStatus) {
        self.status = status;
        self.update_time = Utc::now();
    }
}