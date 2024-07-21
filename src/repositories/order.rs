use std::sync::{Arc, Mutex};
use dashmap::DashMap;
use uuid::Uuid;
use crate::models::meal::{MealItem, MealItemStatus};
use crate::models::order::{Order};

pub struct OrderRepo {
    pub orders: Arc<DashMap<u32, Arc<Mutex<Order>>>>,
}

impl OrderRepo {
    pub fn new() -> Self {
        OrderRepo {
            orders: Arc::new(DashMap::new())
        }
    }

    pub fn add(&self, order: Order) {
        let table_id = order.get_table_id();
        let order_arc = Arc::new(Mutex::new(order));
        self.orders.insert(table_id, order_arc);
    }

    pub fn get_order_by_table_id(&self, id: u32) -> Option<Arc<Mutex<Order>>> {
        self.orders.get(&id).map(|order_arc| order_arc.clone())
    }

    pub fn get_order_meal_item(&self, table_id: u32, meal_item_id: Uuid) -> Option<Arc<Mutex<MealItem>>> {
        if let Some(order_arc) = self.orders.get(&table_id) {
            let order = order_arc.lock().unwrap();
            return order.get_meal_item(meal_item_id);
        }
        None
    }

    pub fn update_order_meal_item_status(&self, table_id: u32, meal_item_id: Uuid, meal_item_status: MealItemStatus) -> bool {
        if let Some(order_arc) = self.orders.get(&table_id) {
            let order = order_arc.lock().unwrap();
            // match order.get_status() {
            //     OrderStatus::Received => order.update_status(OrderStatus::Preparing),
            //     _ => {}
            // }

            if let Some(meal_item_arc) = order.get_meal_item(meal_item_id) {
                let mut meal_item = meal_item_arc.lock().unwrap();
                meal_item.update_state(meal_item_status);
            }
            true
        } else {
            false
        }
    }

    pub fn add_order_meal_items(&self, table_id: u32, meal_items: Vec<MealItem>) -> bool {
        if let Some(order_arc) = self.orders.get(&table_id) {
            let mut order = order_arc.lock().unwrap();
            order.add_meal_items(meal_items);
            true
        } else {
            false
        }
    }
}