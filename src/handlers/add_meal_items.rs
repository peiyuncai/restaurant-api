use std::sync::{Arc};
use std::thread::{sleep};
use std::time::Duration;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use warp::http::StatusCode;
use crate::handlers::error::{ErrResp, MESSAGE_ORDER_NOT_FOUND};
use crate::handlers::query_order::{OrderResp};
use crate::libraries::thread_pool::{ThreadPoolDyn};
use crate::models::meal::{MealItem, MealItemStatus};
use crate::models::menu::MenuItem;
use crate::repositories::order::OrderRepo;

#[derive(Deserialize)]
pub struct MenuItemReq {
    pub menu_item_id: Uuid,
    pub name: String,
    pub price: String,
}

#[derive(Deserialize)]
pub struct AddMealItemsReq {
    pub table_id: u32,
    pub menu_items: Vec<MenuItemReq>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AddMealItemsResp {
    pub order: OrderResp,
}

pub struct AddMealItemsHandler {
    order_repo: Arc<OrderRepo>,
    thread_pool: Arc<dyn ThreadPoolDyn>,
}

impl AddMealItemsHandler {
    pub fn new(order_repo: Arc<OrderRepo>, thread_pool: Arc<dyn ThreadPoolDyn>) -> Self {
        AddMealItemsHandler {
            order_repo,
            thread_pool,
        }
    }

    pub fn handle(&self, req: AddMealItemsReq) -> Result<impl warp::Reply, warp::Rejection> {
        let mut meal_items = Vec::with_capacity(req.menu_items.len());
        for menu_item_req in req.menu_items {
            let menu_item = MenuItem::create(
                menu_item_req.menu_item_id,
                menu_item_req.name,
                menu_item_req.price,
            );

            meal_items.push(MealItem::create(menu_item));
        }

        let existed = self.order_repo.add_order_meal_items(req.table_id, meal_items.clone());
        if !existed {
            let resp = ErrResp {
                message: MESSAGE_ORDER_NOT_FOUND.to_string(),
            };
            return Ok(warp::reply::with_status(
                warp::reply::json(&resp),
                StatusCode::NOT_FOUND,
            ));
        }

        for meal_item in meal_items.iter() {
            let meal_item_id = meal_item.id();
            let table_id = req.table_id;
            let order_repo_arc = Arc::clone(&self.order_repo);

            self.thread_pool.execute(Box::new(move || {
                if let Some(meal_item_arc) = order_repo_arc.get_order_meal_item(table_id, meal_item_id) {
                    let meal_item = meal_item_arc.lock().unwrap();
                    if meal_item.is_removed() { return; }

                    let cooking_time_in_min = meal_item.cooking_time_in_min();
                    drop(meal_item);

                    println!("start preparing {}", meal_item_id);

                    let existed = order_repo_arc.update_order_meal_item_status(table_id, meal_item_id, MealItemStatus::Preparing);
                    if !existed { return; }

                    // Simulates cooking time by putting the thread to sleep, blocking it from accepting new meals until the current meal is prepared.
                    sleep(Duration::from_secs(cooking_time_in_min as u64));

                    order_repo_arc.update_order_meal_item_status(table_id, meal_item_id, MealItemStatus::Completed);
                    println!("completed {}", meal_item_id);
                }
            }))
        }

        if let Some(order) = self.order_repo.get_order_by_table_id(req.table_id) {
            let resp = AddMealItemsResp {
                order: OrderResp::new(order.lock().unwrap().clone(), false),
            };
            return Ok(warp::reply::with_status(
                warp::reply::json(&resp),
                StatusCode::OK,
            ));
        }

        let resp = ErrResp {
            message: StatusCode::INTERNAL_SERVER_ERROR.to_string()
        };
        Ok(warp::reply::with_status(
            warp::reply::json(&resp),
            StatusCode::INTERNAL_SERVER_ERROR,
        ))
    }
}