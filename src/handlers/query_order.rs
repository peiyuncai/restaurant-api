use std::sync::{Arc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use warp::http::StatusCode;
use crate::handlers::error::{ErrResp, MESSAGE_ORDER_NOT_FOUND};
use crate::models::meal::MealItemStatus;
use crate::models::order::{Order, OrderStatus};
use crate::repositories::order::OrderRepo;

#[derive(Serialize, Deserialize, Debug)]
pub struct MealItemResp {
    meal_item_id: Uuid,
    name: String,
    price: String,
    status: String,
    cooking_time_in_min: u32,
    is_remove: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OrderResp {
    pub remaining_cooking_time_upper_bound_in_min: u32,
    pub total_price: String,
    pub status: String,
    pub meal_items: Vec<MealItemResp>,
}

impl OrderResp {
    pub fn new(order: Order, include_removed_items: bool) -> Self {
        let mut order_resp = OrderResp {
            total_price: order.get_total_price().to_string(),
            remaining_cooking_time_upper_bound_in_min: 0,
            status: OrderStatus::Received.to_string(),
            meal_items: vec![],
        };

        let mut has_preparing = false;
        let mut has_received = false;
        let mut all_removed = true;

        for item_arc in order.get_meal_items().iter() {
            let item = item_arc.lock().unwrap();

            if !item.is_removed() || include_removed_items {
                let item_resp = MealItemResp {
                    meal_item_id: item.id(),
                    name: item.get_name(),
                    price: item.price().to_string(),
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
                    order_resp.remaining_cooking_time_upper_bound_in_min += item.cooking_time_in_min();
                }
                MealItemStatus::Preparing => {
                    has_preparing = true;
                    order_resp.remaining_cooking_time_upper_bound_in_min += item.cooking_time_in_min();
                }
                MealItemStatus::Completed => {}
            }
        }

        if all_removed {
            order_resp.status = OrderStatus::Canceled.to_string();
        } else if has_preparing {
            order_resp.status = OrderStatus::Preparing.to_string();
        } else if has_received {
            order_resp.status = OrderStatus::Received.to_string();
        } else {
            order_resp.status = OrderStatus::Completed.to_string();
        }

        order_resp
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QueryOrderResp {
    pub data: OrderResp,
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
                data: OrderResp::new(order, include_removed_items),
            };
            Ok(warp::reply::with_status(
                warp::reply::json(&resp),
                StatusCode::OK,
            ))
        } else {
            let resp = ErrResp {
                error_message: MESSAGE_ORDER_NOT_FOUND.to_string(),
            };
            Ok(warp::reply::with_status(
                warp::reply::json(&resp),
                StatusCode::NOT_FOUND,
            ))
        }
    }
}