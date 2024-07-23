use std::sync::Arc;
use warp::reply::Reply;
use warp::hyper::body::to_bytes;
use warp::http::StatusCode;
use crate::handlers::error::ErrResp;
use crate::handlers::query_order::{QueryOrderHandler, QueryOrderResp};
use crate::models::meal::MealItem;
use crate::models::menu::MenuItem;
use crate::models::order::Order;
use crate::repositories::order::OrderRepo;

#[tokio::test]
async fn test_query_order_handler_handle_success() {
    let order_repo = Arc::new(OrderRepo::new());

    let handler = QueryOrderHandler::new(order_repo.clone());


    let menu_item = MenuItem::new(String::from("fries"), String::from("345"));
    let meal_item = MealItem::create(menu_item);
    let mut order = Order::new(1, vec![]);
    order.add_meal_items(vec![meal_item]);
    let menu_item = MenuItem::new(String::from("burger"), String::from("789"));
    let meal_item = MealItem::create(menu_item);
    order.add_meal_items(vec![meal_item.clone()]);
    order_repo.add(order);
    order_repo.remove_order_meal_items(1, vec![meal_item.id()]);

    let response = handler.handle(1, false).unwrap();

    let response = response.into_response();

    let status = response.status();
    let body = to_bytes(response.into_body()).await.unwrap();
    let body_bytes = body.to_vec();
    let actual_resp: QueryOrderResp = serde_json::from_slice(&*body_bytes).expect("failed to parse");
    println!("{:?}", actual_resp);
    assert_eq!(status, StatusCode::OK);
    assert_eq!("345", actual_resp.data.total_price);
    assert_eq!("Received", actual_resp.data.status);
    assert_eq!(1, actual_resp.data.meal_items.len());
}

#[tokio::test]
async fn test_query_order_handler_handle_not_found() {
    let order_repo = Arc::new(OrderRepo::new());

    let handler = QueryOrderHandler::new(order_repo.clone());

    let response = handler.handle(1, false).unwrap();

    // Convert response into warp::http::Response
    let response = response.into_response();

    // Extract status code and body
    let status = response.status();
    let body = to_bytes(response.into_body()).await.unwrap();
    let body_bytes = body.to_vec();
    let actual_body: ErrResp = serde_json::from_slice(&*body_bytes).expect("failed to parse");

    let expected_body: ErrResp = ErrResp { message: "There are no orders associated with this table".to_string() };

    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(expected_body, actual_body);
}
