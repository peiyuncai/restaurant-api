use std::sync::{Arc, Mutex};
use chrono::{DateTime, Utc};
use dashmap::DashMap;
use uuid::Uuid;
use crate::models::meal::{MealItem, MealItemStatus};
use crate::models::menu::MenuItem;

#[derive(Copy, Clone, Debug)]
pub enum OrderStatus {
    Received,
    Preparing,
    Completed,
    Canceled,
}

#[derive(Clone, Debug)]
pub struct Order {
    order_id: Uuid,
    table_id: u32,
    meal_items: DashMap<Uuid, Arc<Mutex<MealItem>>>,
    total_cooking_time_in_min: u32,
    total_price: f64,
    creation_time: DateTime<Utc>,
    update_time: DateTime<Utc>,
    status: OrderStatus,
}

impl Order {
    pub fn new(table_id: u32, menu_items: Vec<MenuItem>) -> Self {
        let mut order = Order {
            order_id: Uuid::new_v4(),
            table_id,
            meal_items: Default::default(),
            total_cooking_time_in_min: 0,
            total_price: 0.0,
            creation_time: Utc::now(),
            update_time: Utc::now(),
            status: OrderStatus::Received,
        };
        order.add_meal_items(menu_items);
        order
    }

    pub fn add_meal_items(&mut self, menu_items: Vec<MenuItem>) -> bool {
        for menu_item in menu_items.iter() {
            let meal_item = MealItem::create(menu_item.clone());
            self.total_price += meal_item.price();
            self.total_cooking_time_in_min += meal_item.cooking_time_in_min();
            self.meal_items.insert(meal_item.id(), Arc::new(Mutex::new(meal_item)));
        }
        self.update_time = Utc::now();
        true
    }

    pub fn remove_meal_items(&mut self, meal_item_ids: Vec<Uuid>) -> bool {
        for meal_item_id in meal_item_ids.iter() {
            if let Some(meal_item) = self.meal_items.get(meal_item_id) {
                let mut meal_item = meal_item.lock().unwrap();
                match meal_item.get_status() {
                    MealItemStatus::Preparing | MealItemStatus::Completed => return false,
                    _ => {}
                }
                self.total_price -= meal_item.price();
                self.total_cooking_time_in_min -= meal_item.cooking_time_in_min();
                meal_item.remove();
            }
        }
        self.update_time = Utc::now();
        if self.meal_items.iter().filter(|m| !m.lock().unwrap().is_removed()).count() == 0 {
            self.status = OrderStatus::Canceled;
        }
        true
    }

    pub fn get_meal_items(&self) -> Vec<Arc<Mutex<MealItem>>> {
        self.meal_items.iter().map(|entry| entry.value().clone()).collect()
    }

    pub fn get_meal_item(&self, meal_item_id: Uuid) -> Option<Arc<Mutex<MealItem>>> {
        self.meal_items.get(&meal_item_id).map(|item| item.clone())
    }

    pub fn get_status(&self) -> OrderStatus {
        self.status
    }

    pub fn update_status(&mut self, status: OrderStatus) {
        self.status = status
    }

    pub fn get_table_id(&self) -> u32 {
        self.table_id
    }

    pub fn get_order_id(&self) -> Uuid {
        self.order_id
    }
}