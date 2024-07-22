#[derive(Clone, Debug, Copy)]
pub struct Price(f64);

impl Price {
    pub fn from_string(price: String) -> Self {
        let price_in_cents = price.parse().unwrap_or(0.0);
        let price: f64 = price_in_cents / 100.0;
        Price(price)
    }

    pub fn to_string(self) -> String {
        let price_in_cents: u64 = (self.0 * 100.0) as u64;
        price_in_cents.to_string()
    }

    pub fn add(&mut self, other: Price) {
        self.0 += other.0;
    }

    pub fn deduct(&mut self, other: Price) {
        self.0 -= other.0;
    }
}

impl Default for Price {
    fn default() -> Self {
        Price(0.0)
    }
}