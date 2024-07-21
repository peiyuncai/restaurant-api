use std::sync::{Arc, Mutex};
use chrono::{DateTime, Utc};
use dashmap::DashMap;
use serde::{Serialize};
use uuid::Uuid;
use crate::models::meal::{MealItem, MealItemStatus};
use crate::models::menu::MenuItem;

#[derive(Copy, Clone, Debug, Serialize)]
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
    // status: OrderStatus,
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
            // status: OrderStatus::Received,
        };
        order.add_meal_items_by_menu_items(menu_items);
        order
    }

    pub fn add_meal_items_by_menu_items(&mut self, menu_items: Vec<MenuItem>) -> bool {
        for menu_item in menu_items.iter() {
            let meal_item = MealItem::create(menu_item.clone());
            self.total_price += meal_item.price();
            self.total_cooking_time_in_min += meal_item.cooking_time_in_min();
            self.meal_items.insert(meal_item.id(), Arc::new(Mutex::new(meal_item)));
        }
        self.update_time = Utc::now();
        true
    }

    pub fn add_meal_items(&mut self, meal_items: Vec<MealItem>) -> bool {
        for meal_item in meal_items.iter() {
            self.total_price += meal_item.price();
            self.total_cooking_time_in_min += meal_item.cooking_time_in_min();
            self.meal_items.insert(meal_item.id(), Arc::new(Mutex::new(meal_item.clone())));
        }
        self.update_time = Utc::now();
        true
    }

    pub fn remove_meal_items(&mut self, meal_item_ids: Vec<Uuid>) -> Vec<Uuid> {
        let mut non_removable_items = Vec::new();
        for meal_item_id in meal_item_ids.iter() {
            if let Some(meal_item) = self.meal_items.get(meal_item_id) {
                let mut meal_item = meal_item.lock().unwrap();
                println!("{} check if item can be removed", meal_item.id());

                match meal_item.get_status() {
                    MealItemStatus::Preparing | MealItemStatus::Completed => {
                        non_removable_items.push(meal_item_id.clone());
                        continue;
                    }
                    _ => {}
                }
                println!("{} is removed", meal_item.id());
                self.total_price -= meal_item.price();
                self.total_cooking_time_in_min -= meal_item.cooking_time_in_min();

                meal_item.remove();
            } else {
                non_removable_items.push(meal_item_id.clone());
            }
        }
        self.update_time = Utc::now();
        // if self.meal_items.iter().filter(|m| !m.lock().unwrap().is_removed()).count() == 0 {
        //     self.status = OrderStatus::Canceled;
        // }
        non_removable_items
    }

    pub fn get_meal_items(&self) -> Vec<Arc<Mutex<MealItem>>> {
        self.meal_items.iter().map(|entry| entry.value().clone()).collect()
    }

    pub fn get_meal_item(&self, meal_item_id: Uuid) -> Option<Arc<Mutex<MealItem>>> {
        self.meal_items.get(&meal_item_id).map(|item| item.clone())
    }

    // pub fn get_status(&self) -> OrderStatus {
    //     self.status
    // }
    //
    // pub fn update_status(&mut self, status: OrderStatus) {
    //     self.status = status
    // }

    pub fn get_table_id(&self) -> u32 {
        self.table_id
    }

    pub fn get_order_id(&self) -> Uuid {
        self.order_id
    }

    pub fn get_total_price(&self) -> f64 {
        self.total_price
    }
}