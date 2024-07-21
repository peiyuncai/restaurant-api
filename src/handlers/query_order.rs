use std::sync::{Arc};
use serde::Serialize;
use uuid::Uuid;
use warp::http::StatusCode;
use crate::handlers::add_meal_items::{ErrResp, MESSAGE_ORDER_NOT_FOUND};
use crate::models::meal::MealItemStatus;
use crate::models::order::{Order, OrderStatus};
use crate::repositories::order::OrderRepo;

#[derive(Serialize)]
struct MealItemResp {
    meal_item_id: Uuid,
    name: String,
    price: String,
    status: String,
    cooking_time_in_min: u32,
    is_remove: bool,
}

#[derive(Serialize)]
pub struct OrderResp {
    remaining_cooking_time_upper_bound_in_min: u32,
    total_price: String,
    status: OrderStatus,
    meal_items: Vec<MealItemResp>,
}

impl OrderResp {
    pub fn new(order: Order, include_removed_items: bool) -> Self {
        let mut order_resp = OrderResp {
            total_price: "".to_string(),
            remaining_cooking_time_upper_bound_in_min: 0,
            status: OrderStatus::Received,
            meal_items: vec![],
        };

        order_resp.total_price = convert_price(order.get_total_price());

        let mut has_preparing = false;
        let mut has_received = false;
        let mut has_completed = true;
        let mut all_removed = true;

        for item_arc in order.get_meal_items().iter() {
            let item = item_arc.lock().unwrap();

            if !item.is_removed() || include_removed_items {
                let item_resp = MealItemResp {
                    meal_item_id: item.id(),
                    name: item.get_name(),
                    price: convert_price(item.price()),
                    cooking_time_in_min: item.cooking_time_in_min(),
                    status: item.get_status().to_string(),
                    is_remove: item.is_removed(),
                };
                order_resp.meal_items.push(item_resp);
            }

            if item.is_removed() {
                continue;
            }

            all_removed = false;

            match item.get_status() {
                MealItemStatus::Received => {
                    has_received = true;
                    has_completed = false;
                    order_resp.remaining_cooking_time_upper_bound_in_min += item.cooking_time_in_min();
                }
                MealItemStatus::Preparing => {
                    has_preparing = true;
                    has_completed = false;
                    order_resp.remaining_cooking_time_upper_bound_in_min += item.cooking_time_in_min();
                }
                MealItemStatus::Completed => {}
            }
        }

        if all_removed {
            order_resp.status = OrderStatus::Canceled;
        } else if has_preparing {
            order_resp.status = OrderStatus::Preparing;
        } else if has_received {
            order_resp.status = OrderStatus::Received;
        } else if has_completed {
            order_resp.status = OrderStatus::Completed;
        }

        order_resp
    }
}

pub fn convert_price(price: f64) -> String {
    let price_in_cents: u64 = (price * 100.0) as u64;
    price_in_cents.to_string()
}

#[derive(Serialize)]
struct QueryOrderResp {
    order: OrderResp,
}

pub struct QueryOrderHandler {
    order_repo: Arc<OrderRepo>,
}

impl QueryOrderHandler {
    pub fn new(order_repo: Arc<OrderRepo>) -> Self {
        QueryOrderHandler {
            order_repo,
        }
    }

    pub fn handle(&self, table_id: u32, include_removed_items: bool) -> Result<impl warp::Reply, warp::Rejection> {
        if let Some(order_arc) = self.order_repo.get_order_by_table_id(table_id) {
            let order = order_arc.lock().unwrap().clone();
            let resp = QueryOrderResp {
                order: OrderResp::new(order, include_removed_items),
            };
            Ok(warp::reply::with_status(
                warp::reply::json(&resp),
                StatusCode::OK, // or StatusCode::NOT_FOUND depending on your logic
            ))
        } else {
            let resp = ErrResp {
                message: MESSAGE_ORDER_NOT_FOUND.to_string(),
            };
            Ok(warp::reply::with_status(
                warp::reply::json(&resp),
                StatusCode::NOT_FOUND,
            ))
        }
    }
}