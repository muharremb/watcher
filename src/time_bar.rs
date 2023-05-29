use crate::bar::Bar;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct TimeBar {
    pub open_ts: u64,
    pub close_ts: u64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub buy_volume: f64,
    pub sell_volume: f64,
    pub total_volume: f64,
}

impl Bar for TimeBar {
    fn build_bar(open_ts: u64, price: f64, is_buy: bool, quantity: f64) -> Self {
        TimeBar {
            open_ts,
            close_ts: open_ts,
            open: price,
            high: price,
            low: price,
            close: price,
            buy_volume: if is_buy { price * quantity } else { 0.0 },
            sell_volume: if !is_buy { price * quantity } else { 0.0 },
            total_volume: price * quantity,
        }
    }

    fn update_bar(&mut self, price: f64, is_buy: bool, quantity: f64) {
        self.high = self.high.max(price);
        self.low = self.low.min(price);
        self.close = price;
        if is_buy {
            self.buy_volume += price * quantity;
        } else {
            self.sell_volume += price * quantity;
        }
        self.total_volume += price * quantity;
    }
}
