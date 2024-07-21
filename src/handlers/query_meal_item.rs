use std::sync::{Arc};
use serde::Serialize;
use uuid::Uuid;
use crate::handlers::query_order::convertPrice;
use crate::models::meal::{MealItem};
use crate::repositories::order::OrderRepo;

#[derive(Serialize)]
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
            price: convertPrice(item.price()),
            cooking_time_in_min: item.cooking_time_in_min(),
            is_removed: item.is_removed(),
            status: item.get_status().to_string(),
        }
    }
}

#[derive(Serialize)]
struct QueryMealItemResp {
    meal_item: MealItemResp,
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
                meal_item: MealItemResp::new(item),
            };
            Ok(warp::reply::json(&resp))
        } else {
            Err(warp::reject::not_found())
        }
    }
}