use std::sync::{Arc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use warp::http::StatusCode;
use crate::handlers::error::{ErrResp, MESSAGE_ITEM_NOT_FOUND};
use crate::models::meal::{MealItem};
use crate::repositories::order::OrderRepo;

#[derive(Serialize, Deserialize, Debug)]
struct MealItemResp {
    meal_item_id: Uuid,
    name: String,
    price: String,
    cooking_time_in_min: u32,
    is_removed: bool,
    status: String,
}

impl MealItemResp {
    pub fn new(item: MealItem) -> Self {
        MealItemResp {
            meal_item_id: item.id(),
            name: item.get_name(),
            price: item.price().to_string(),
            cooking_time_in_min: item.cooking_time_in_min(),
            is_removed: item.is_removed(),
            status: item.get_status().to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct QueryMealItemResp {
    data: MealItemResp,
}

pub struct QueryMealItemHandler {
    order_repo: Arc<OrderRepo>,
}

impl QueryMealItemHandler {
    pub fn new(order_repo: Arc<OrderRepo>) -> Self {
        QueryMealItemHandler {
            order_repo,
        }
    }

    pub fn handle(&self, table_id: u32, meal_item_id: Uuid) -> Result<impl warp::Reply, warp::Rejection> {
        if let Some(item_arc) = self.order_repo.get_order_meal_item(table_id, meal_item_id) {
            let item = item_arc.lock().unwrap().clone();
            let resp = QueryMealItemResp {
                data: MealItemResp::new(item),
            };
            Ok(warp::reply::with_status(
                warp::reply::json(&resp),
                StatusCode::OK, // or StatusCode::NOT_FOUND depending on your logic
            ))
        } else {
            let resp = ErrResp {
                message: MESSAGE_ITEM_NOT_FOUND.to_string(),
            };
            Ok(warp::reply::with_status(
                warp::reply::json(&resp),
                StatusCode::NOT_FOUND,
            ))
        }
    }
}