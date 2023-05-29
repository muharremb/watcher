// volume_time_bar.rs
use crate::bar::Bar;
use chrono::NaiveDateTime;
use std::fmt;

#[derive(Debug, Clone)]
pub struct VolumeTimeBar {
    pub open_ts: u64,
    pub close_ts: u64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub buy_volume: f64,
    pub sell_volume: f64,
    pub buy_ratio: f64,
    pub total_volume: f64,
}

impl Bar for VolumeTimeBar {
    fn build_bar(open_ts: u64, price: f64, is_buy: bool, quantity: f64) -> Self {
        let volume = price * quantity;
        VolumeTimeBar {
            open_ts,
            close_ts: open_ts,
            open: price,
            high: price,
            low: price,
            close: price,
            buy_volume: if is_buy { volume } else { 0.0 },
            sell_volume: if !is_buy { volume } else { 0.0 },
            buy_ratio: if is_buy { 1.0 } else { 0.0 },
            total_volume: volume,
        }
    }

    fn update_bar(&mut self, price: f64, is_buy: bool, quantity: f64) {
        let volume = price * quantity;
        self.high = self.high.max(price);
        self.low = self.low.min(price);
        self.close = price;
        if is_buy {
            self.buy_volume += volume;
        } else {
            self.sell_volume += volume;
        }
        self.total_volume += volume;
        if self.total_volume != 0.0 {
            self.buy_ratio = self.buy_volume / self.total_volume;
        }
    }
}

impl fmt::Display for VolumeTimeBar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let open_dt = NaiveDateTime::from_timestamp_opt((self.open_ts / 1000) as i64, 0)
            .unwrap_or_else(|| NaiveDateTime::from_timestamp_opt(0, 0).unwrap());
        let close_dt = NaiveDateTime::from_timestamp_opt((self.close_ts / 1000) as i64, 0)
            .unwrap_or_else(|| NaiveDateTime::from_timestamp_opt(0, 0).unwrap());
        write!(f, "VolumeTimeBar {{ open_ts: {}, close_ts: {}, close: {}, buy_ratio: {:.2}, total_volume: {:.0} }}",
            open_dt, close_dt, self.close, self.buy_ratio, self.total_volume)
    }
}
