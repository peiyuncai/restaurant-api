use std::collections::HashMap;
use chrono::{Date, DateTime, Utc};
use uuid::Uuid;
use crate::models::meal::MealItem;
use crate::models::menu::MenuItem;

enum OrderStatus {
    InProgress,
    Completed,
    Canceled,
}

pub struct Order {
    order_id: Uuid,
    table_id: u32,
    meal_items: HashMap<Uuid, MealItem>,
    total_cooking_time_in_min: u32,
    total_price: f64,
    creation_time: DateTime<Utc>,
    update_time: DateTime<Utc>,
    status: OrderStatus,
}

impl Order {
    pub fn add_meal_items(&mut self, menu_items: Vec<MenuItem>) -> bool {
        for menu_item in menu_items.iter() {
            let meal_item = MealItem::create(menu_item.clone());
            self.total_price = self.total_price + menu_item.price();
            self.total_cooking_time_in_min = self.total_cooking_time_in_min + meal_item.cooking_time_in_min();
            self.meal_items.insert(meal_item.id(), meal_item);
        }
        self.update_time = Utc::now();
        true
    }

    pub fn remove_meal_items(&mut self, meal_item_ids: Vec<Uuid>) -> bool {
        for meal_item_id in meal_item_ids.iter() {
            if let Some(meal_item) = self.meal_items.get_mut(meal_item_id) {
                self.total_price = self.total_price - meal_item.price();
                self.total_cooking_time_in_min = self.total_cooking_time_in_min - meal_item.cooking_time_in_min();
                meal_item.remove();
            }
        }
        self.update_time = Utc::now();
        if self.meal_items.values().filter(|m| !m.is_removed()).count() == 0 {
            self.status = OrderStatus::Canceled;
        }
        true
    }

    pub fn get_meal_item(&self, meal_item_id: Uuid) -> Option<&MealItem> {
        self.meal_items.get(&meal_item_id)
    }
}