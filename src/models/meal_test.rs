#[cfg(test)]
mod meal_test {
    use crate::models::meal::{MealItem, MealItemStatus};
    use crate::models::menu::MenuItem;

    #[test]
    fn test_remove() {
        let menu_item = MenuItem::new(String::from("fries"), String::from("345"));
        let mut meal_item = MealItem::create(menu_item);
        assert!(!meal_item.is_removed());

        meal_item.remove();

        assert!(meal_item.is_removed());
    }

    #[test]
    fn test_update_status() {
        let menu_item = MenuItem::new(String::from("fries"), String::from("345"));
        let mut meal_item = MealItem::create(menu_item);
        assert_eq!(MealItemStatus::Received, meal_item.get_status());

        meal_item.update_state(MealItemStatus::Completed);

        assert_eq!(MealItemStatus::Completed, meal_item.get_status());
    }
}