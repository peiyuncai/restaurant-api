#[cfg(test)]
mod order_test {
    use crate::models::meal::MealItem;
    use crate::models::menu::MenuItem;
    use crate::models::order::Order;
    use crate::repositories::order::OrderRepo;

    #[test]
    fn test_add() {
        let repo = OrderRepo::new();

        let menu_item = MenuItem::new(String::from("fries"), String::from("34.5"));
        let order = Order::new(1, vec![menu_item]);

        repo.add(order.clone());

        if let Some(order_arc) = repo.get_order_by_table_id(1) {
            let fetched_order = order_arc.lock().unwrap();
            let fetched_meal_item = fetched_order.get_meal_items().get(0).unwrap().lock().unwrap().clone();
            let meal_item = order.get_meal_items().get(0).unwrap().lock().unwrap().clone();
            assert_eq!(order.get_table_id(), fetched_order.get_table_id());
            assert_eq!(order.get_total_price(), fetched_order.get_total_price());
            assert_eq!(meal_item, fetched_meal_item);
        } else {
            panic!("Order not found");
        }
    }

    #[test]
    fn test_get_order_by_table_id() {
        let repo = OrderRepo::new();

        assert!(repo.get_order_by_table_id(1).is_none());
        assert!(repo.get_order_by_table_id(2).is_none());

        repo.add(Order::new(1, vec![]));

        assert!(repo.get_order_by_table_id(1).is_some());
        assert!(repo.get_order_by_table_id(2).is_none());
    }

    #[test]
    fn test_get_order_meal_item() {
        let repo = OrderRepo::new();

        let menu_item = MenuItem::new(String::from("fries"), String::from("34.5"));
        let meal_item = MealItem::create(menu_item);
        let mut order = Order::new(1, vec![]);
        order.add_meal_items(vec![meal_item.clone()]);

        if let Some(meal_item_arc) = order.get_meal_item(meal_item.id()) {
            let fetched_meal_item = meal_item_arc.lock().unwrap().clone();
            assert_eq!(meal_item, fetched_meal_item);
        } else {
            panic!("meal item not found")
        }
    }

    #[test]
    fn test_update_order_meal_item_status() {
        let repo = OrderRepo::new();

        let menu_item = MenuItem::new(String::from("fries"), String::from("34.5"));
        let meal_item = MealItem::create(menu_item);
        let mut order = Order::new(1, vec![]);
        order.add_meal_items(vec![meal_item.clone()]);

        if let Some(meal_item_arc) = order.get_meal_item(meal_item.id()) {
            let fetched_meal_item = meal_item_arc.lock().unwrap().clone();
            assert_eq!(meal_item, fetched_meal_item);
        } else {
            panic!("meal item not found")
        }
    }
}
