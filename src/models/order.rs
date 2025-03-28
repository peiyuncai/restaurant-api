use std::fmt;
use std::sync::{Arc, Mutex};
use chrono::{DateTime, Utc};
use dashmap::DashMap;
use serde::Serialize;
use uuid::Uuid;
use crate::models::meal::{MealItem, MealItemStatus};
use crate::models::menu::MenuItem;
use crate::models::price::Price;

#[derive(Copy, Clone, Debug, Serialize, PartialEq)]
pub enum OrderStatus {
    Received,
    Preparing,
    Completed,
    Canceled,
}

impl fmt::Display for OrderStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            OrderStatus::Received => "Received",
            OrderStatus::Preparing => "Preparing",
            OrderStatus::Completed => "Completed",
            OrderStatus::Canceled => "Canceled",
        };
        write!(f, "{}", s)
    }
}

#[derive(Clone, Debug)]
pub struct Order {
    order_id: Uuid,
    table_id: u32,
    meal_items: DashMap<Uuid, Arc<Mutex<MealItem>>>,
    total_cooking_time_in_min: u32,
    total_price: Price,
    creation_time: DateTime<Utc>,
    update_time: DateTime<Utc>,
}

impl Order {
    pub fn new(table_id: u32, menu_items: Vec<MenuItem>) -> Self {
        let mut order = Order {
            order_id: Uuid::new_v4(),
            table_id,
            meal_items: Default::default(),
            total_cooking_time_in_min: 0,
            total_price: Default::default(),
            creation_time: Utc::now(),
            update_time: Utc::now(),
        };
        order.add_meal_items_by_menu_items(menu_items);
        order
    }

    fn add_meal_items_by_menu_items(&mut self, menu_items: Vec<MenuItem>) -> bool {
        for menu_item in menu_items.iter() {
            let meal_item = MealItem::create(menu_item.clone());
            self.total_price.add(meal_item.price());
            self.total_cooking_time_in_min += meal_item.cooking_time_in_min();
            self.meal_items.insert(meal_item.id(), Arc::new(Mutex::new(meal_item)));
        }
        self.update_time = Utc::now();
        true
    }

    pub fn add_meal_items(&mut self, meal_items: Vec<MealItem>) -> bool {
        for meal_item in meal_items.iter() {
            self.total_price.add(meal_item.price());
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

                if meal_item.is_removed() {
                    continue;
                }

                match meal_item.get_status() {
                    MealItemStatus::Preparing | MealItemStatus::Completed => {
                        non_removable_items.push(meal_item_id.clone());
                        continue;
                    }
                    _ => {}
                }

                self.total_price.deduct(meal_item.price());
                self.total_cooking_time_in_min -= meal_item.cooking_time_in_min();

                meal_item.remove();
            } else {
                non_removable_items.push(meal_item_id.clone());
            }
        }
        self.update_time = Utc::now();
        non_removable_items
    }

    pub fn get_meal_items(&self) -> Vec<Arc<Mutex<MealItem>>> {
        self.meal_items.iter().map(|entry| entry.value().clone()).collect()
    }

    pub fn get_meal_item(&self, meal_item_id: Uuid) -> Option<Arc<Mutex<MealItem>>> {
        self.meal_items.get(&meal_item_id).map(|item| item.clone())
    }

    pub fn get_table_id(&self) -> u32 {
        self.table_id
    }

    pub fn get_total_price(&self) -> Price {
        self.total_price
    }

    pub fn is_active(&self) -> bool {
        let order_status = self.get_order_status();
        match order_status {
            OrderStatus::Received | OrderStatus::Preparing => true,
            _ => false,
        }
    }

    // TODO: improve to have and update order_status on write
    pub fn get_order_status(&self) -> OrderStatus {
        let mut has_preparing = false;
        let mut has_received = false;
        let mut all_removed = true;
        for item_arc in self.meal_items.iter() {
            let item = item_arc.lock().unwrap();

            if item.is_removed() {
                continue;
            }

            all_removed = false;

            match item.get_status() {
                MealItemStatus::Received => {
                    has_received = true;
                }
                MealItemStatus::Preparing => {
                    has_preparing = true;
                }
                MealItemStatus::Completed => {}
            }
        }

        if all_removed {
            OrderStatus::Canceled
        } else if has_preparing {
            OrderStatus::Preparing
        } else if has_received {
            OrderStatus::Received
        } else {
            OrderStatus::Completed
        }
    }
}