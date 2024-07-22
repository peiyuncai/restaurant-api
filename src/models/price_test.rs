#[cfg(test)]
mod price_test {
    use crate::models::price::Price;

    #[test]
    fn test_from_string() {
        let price = Price::from_string("345".to_string());
        assert_eq!("345", price.to_string());
    }

    #[test]
    fn test_to_string() {
        let price = Price::from_string("345".to_string());
        assert_eq!("345", price.to_string());
    }

    #[test]
    fn test_add() {
        let mut price = Price::from_string("345".to_string());
        price.add(Price::from_string("123".to_string()));
        assert_eq!("468", price.to_string());
    }

    #[test]
    fn test_deduct() {
        let mut price = Price::from_string("345".to_string());
        price.deduct(Price::from_string("123".to_string()));
        assert_eq!("222", price.to_string());
    }
}