use std::sync::{Arc, Mutex};
use dashmap::DashMap;
use uuid::Uuid;
use crate::models::meal::MealItemStatus;
use crate::models::order::{Order, OrderStatus};

pub struct OrderRepo {
    pub orders: Arc<DashMap<u32, Arc<Mutex<Order>>>>,
}

impl OrderRepo {
    pub fn new() -> Self {
        OrderRepo {
            orders: Arc::new(DashMap::new())
        }
    }

    fn get_order_by_table_id(&self, id: u32) -> Option<Arc<Mutex<Order>>> {
        self.orders.get(&id).map(|entry| entry.value().clone())
    }

    fn update_order_meal_item_status(&self, table_id: u32, meal_item_id: Uuid, meal_item_status: MealItemStatus) -> bool {
        if let Some(order_arc) = self.orders.get(&table_id) {
            let mut order = order_arc.lock().unwrap();
            if order.get_status() == OrderStatus::Received {
                order.update_status(OrderStatus::Preparing);
            }
            if let Some(meal_item) = order.get_meal_item_mut(meal_item_id) {
                meal_item.update_state(meal_item_status);
            }
            true
        } else {
            false
        }
    }
}