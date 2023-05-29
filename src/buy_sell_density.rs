use crate::volume_time_bar::VolumeTimeBar;
use std::fmt;

#[derive(Debug)]
pub struct BuySellDensity {
    alpha: f64,
    volume_weighted_buy_ratio: f64,
    volume_weighted_total_volume: f64,
}

impl BuySellDensity {
    pub fn new(alpha: f64) -> Self {
        BuySellDensity {
            alpha,
            volume_weighted_buy_ratio: 0.0,
            volume_weighted_total_volume: 0.0,
        }
    }

    pub fn update(&mut self, volume_time_bar: &VolumeTimeBar) {
        self.volume_weighted_buy_ratio = self.alpha
            * (volume_time_bar.buy_ratio * volume_time_bar.total_volume)
            + (1.0 - self.alpha) * self.volume_weighted_buy_ratio;
        self.volume_weighted_total_volume = self.alpha * volume_time_bar.total_volume
            + (1.0 - self.alpha) * self.volume_weighted_total_volume;
    }

    pub fn value(&self) -> f64 {
        self.volume_weighted_buy_ratio / self.volume_weighted_total_volume
    }
}

impl fmt::Display for BuySellDensity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Buy/Sell Density: {:.2}", self.value())
    }
}
