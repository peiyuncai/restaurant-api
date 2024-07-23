use std::sync::Arc;
use warp::reply::Reply;
use warp::hyper::body::to_bytes;
use warp::http::StatusCode;
use crate::handlers::error::{ErrResp, MESSAGE_ORDER_NOT_FOUND, MESSAGE_ORDER_REMOVAL_FAILED};
use crate::handlers::remove_order::RemoveOrderHandler;
use crate::models::meal::{MealItem, MealItemStatus};
use crate::models::menu::MenuItem;
use crate::models::order::Order;
use crate::repositories::order::OrderRepo;

#[tokio::test]
async fn test_remove_order_handler_handle_success() {
    let order_repo = Arc::new(OrderRepo::new());

    let handler = RemoveOrderHandler::new(order_repo.clone());

    let order = Order::new(1, vec![]);
    order_repo.add(order);

    let response = handler.handle(1).unwrap();

    let response = response.into_response();

    let status = response.status();
    assert_eq!(status, StatusCode::NO_CONTENT);
}

#[tokio::test]
async fn test_remove_order_handler_handle_not_found() {
    let order_repo = Arc::new(OrderRepo::new());

    let handler = RemoveOrderHandler::new(order_repo.clone());

    let response = handler.handle(1).unwrap();

    let response = response.into_response();

    let status = response.status();
    let body = to_bytes(response.into_body()).await.unwrap();
    let body_bytes = body.to_vec();
    let actual_body: ErrResp = serde_json::from_slice(&*body_bytes).expect("failed to parse");

    let expected_body: ErrResp = ErrResp { error_message: MESSAGE_ORDER_NOT_FOUND.to_string() };

    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(expected_body, actual_body);
}

#[tokio::test]
async fn test_remove_order_handler_handle_conflict() {
    let order_repo = Arc::new(OrderRepo::new());

    let handler = RemoveOrderHandler::new(order_repo.clone());

    let menu_item = MenuItem::new(String::from("fries"), String::from("345"));
    let mut meal_item = MealItem::create(menu_item);
    meal_item.update_state(MealItemStatus::Preparing);
    let mut order = Order::new(1, vec![]);
    order.add_meal_items(vec![meal_item]);
    order_repo.add(order);

    let response = handler.handle(1).unwrap();

    let response = response.into_response();

    let status = response.status();
    let body = to_bytes(response.into_body()).await.unwrap();
    let body_bytes = body.to_vec();
    let actual_body: ErrResp = serde_json::from_slice(&*body_bytes).expect("failed to parse");

    let expected_body: ErrResp = ErrResp { error_message: MESSAGE_ORDER_REMOVAL_FAILED.to_string() };

    assert_eq!(status, StatusCode::CONFLICT);
    assert_eq!(expected_body, actual_body);
}
