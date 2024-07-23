use std::sync::{Arc};
use serde::{Deserialize, Serialize};
use warp::http::{StatusCode};
use crate::handlers::error::{ErrResp, MESSAGE_ORDER_NOT_FOUND, MESSAGE_ORDER_REMOVAL_FAILED};
use crate::repositories::order::OrderRepo;

#[derive(Serialize, Deserialize, Debug)]
pub struct RemoveOrderResp {
    pub table_id: u32,
    pub message: String,
}

pub struct RemoveOrderHandler {
    order_repo: Arc<OrderRepo>,
}

impl RemoveOrderHandler {
    pub fn new(order_repo: Arc<OrderRepo>) -> Self {
        RemoveOrderHandler {
            order_repo,
        }
    }

    pub fn handle(&self, table_id: u32) -> Result<impl warp::Reply, warp::Rejection> {
        let (result, existed) = self.order_repo.remove_order(table_id);
        if !existed {
            let resp = ErrResp {
                message: MESSAGE_ORDER_NOT_FOUND.to_string(),
            };
            return Ok(warp::reply::with_status(
                warp::reply::json(&resp),
                StatusCode::NOT_FOUND,
            ));
        }

        if result {
            let resp = RemoveOrderResp {
                table_id,
                message: "Success".to_string(),
            };
            Ok(warp::reply::with_status(
                warp::reply::json(&resp), // You can use a static string for success
                StatusCode::OK,
            ))
        } else {
            let resp = RemoveOrderResp {
                table_id,
                message: MESSAGE_ORDER_REMOVAL_FAILED.to_string(),
            };
            Ok(warp::reply::with_status(
                warp::reply::json(&resp),
                StatusCode::CONFLICT,
            ))
        }
    }
}