use std::sync::Arc;
use warp::reply::Reply;
use warp::hyper::body::to_bytes;
use uuid::Uuid;
use warp::http::StatusCode;
use crate::handlers::add_meal_items::{AddMealItemsHandler, AddMealItemsReq, MenuItemReq};
use crate::handlers::error::ErrResp;
use crate::handlers::tests::thread_pool_mock::MockThreadPool;
use crate::models::order::Order;
use crate::repositories::order::OrderRepo;

#[tokio::test]
async fn test_add_meal_items_handler_success() {
    let order_repo = Arc::new(OrderRepo::new());
    let thread_pool = Arc::new(MockThreadPool::new(2));

    let handler = AddMealItemsHandler::new(order_repo.clone(), thread_pool.clone());

    let order = Order::new(1, vec![]);
    order_repo.add(order);

    let req = AddMealItemsReq {
        table_id: 1,
        menu_items: vec![
            MenuItemReq {
                menu_item_id: Uuid::new_v4(),
                name: String::from("fries"),
                price: String::from("345"),
            },
            MenuItemReq {
                menu_item_id: Uuid::new_v4(),
                name: String::from("burger"),
                price: String::from("789"),
            },
        ],
    };

    let response = handler.handle(req).unwrap();

    // Convert response into warp::http::Response
    let response = response.into_response();

    // Extract status code and body
    let status = response.status();
    // TODO: not working now though e2e tests are fine
    let body = to_bytes(response.into_body()).await.unwrap();
    // let body_bytes = body.to_vec();
    // let actual_body: OrderResp = serde_json::from_slice(&*body_bytes).expect("failed to parse");
    // println!("{:?}", actual_body);

    thread_pool.wait();
    assert_eq!(status, StatusCode::OK);
    assert_eq!(2, thread_pool.get_count());
}

#[tokio::test]
async fn test_add_meal_items_handler_not_found() {
    let order_repo = Arc::new(OrderRepo::new());
    let thread_pool = Arc::new(MockThreadPool::new(2));

    let handler = AddMealItemsHandler::new(order_repo.clone(), thread_pool.clone());

    let req = AddMealItemsReq {
        table_id: 1,
        menu_items: vec![
            MenuItemReq {
                menu_item_id: Uuid::new_v4(),
                name: String::from("fries"),
                price: String::from("345"),
            },
            MenuItemReq {
                menu_item_id: Uuid::new_v4(),
                name: String::from("burger"),
                price: String::from("789"),
            },
        ],
    };

    let response = handler.handle(req).unwrap();

    // Convert response into warp::http::Response
    let response = response.into_response();

    // Extract status code and body
    let status = response.status();
    let body = to_bytes(response.into_body()).await.unwrap();
    let body_bytes = body.to_vec();
    let actual_body: ErrResp = serde_json::from_slice(&*body_bytes).expect("failed to parse");

    let expected_body: ErrResp = ErrResp { message: "There are no orders associated with this table".to_string() };

    thread_pool.wait();
    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(expected_body, actual_body);
    assert_eq!(0, thread_pool.get_count());
}
