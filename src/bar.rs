
pub trait Bar {
    fn build_bar(open_ts: u64, price: f64, is_buy: bool, quantity: f64) -> Self;
    fn update_bar(&mut self, price: f64, is_buy: bool, quantity: f64);
}
