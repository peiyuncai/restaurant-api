use std::sync::{Arc};
use serde::{Deserialize, Serialize};
use warp::http::StatusCode;
use crate::usecases::models::error::{ErrResp, MESSAGE_ORDER_NOT_FOUND};
use crate::repositories::order::OrderRepo;
use crate::usecases::models::order_resp::OrderResp;

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