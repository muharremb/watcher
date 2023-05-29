use colored::*;
use serde_json::from_str;
use tungstenite::connect;
use url::Url;
mod bar;
mod buy_sell_density;
mod time_bar;
mod trade;
mod volume_time_bar;

use bar::Bar;
use buy_sell_density::BuySellDensity;
use time_bar::TimeBar;
use trade::Trade;
use volume_time_bar::VolumeTimeBar;

fn main() {
    let (mut socket, response) =
        connect(Url::parse("wss://stream.binance.com:9443/ws/btcusdt@trade").unwrap())
            .expect("Failed to connect");

    println!("Connected to the server.");
    println!("HTTP status code: {}", response.status());
    println!("Response contains the following headers:");
    for (ref header, value) in response.headers() {
        println!(" header: {} - value: {:?}", header, value);
    }

    let mut time_bars: Vec<TimeBar> = Vec::new(); // Vector to store TimeBars
    let mut volume_time_bars: Vec<VolumeTimeBar> = Vec::new(); // Vector to store VolumeTimeBars
    let mut last_five_time_bars: Vec<TimeBar> = Vec::new(); // Vector to store the last 5 TimeBars
    let mut current_time_bar: Option<TimeBar> = None; // Current TimeBar
    let mut current_volume_time_bar: Option<VolumeTimeBar> = None; // Current VolumeTimeBar
    let time_bar_duration: u64 = 10 * 1000; // TimeBar duration in miliseconds
    let alpha = (-0.5f64).exp();
    let mut buy_sell_density = BuySellDensity::new(alpha);

    loop {
        let msg = socket.read_message().expect("Error reading message");

        // Check if the message is a Text message
        if let tungstenite::Message::Text(text) = msg {
            // Try to parse the message as a Trade
            match from_str::<Trade>(&text) {
                Ok(trade) => {
                    // If there's no current TimeBar or the trade's timestamp is outside the current TimeBar's time window
                    if current_time_bar.is_none()
                        || trade.event_time
                            >= current_time_bar.as_ref().unwrap().open_ts + time_bar_duration
                    {
                        // Finalize the current TimeBar and add it to the vector
                        if let Some(time_bar) = current_time_bar.take() {
                            time_bars.push(time_bar.clone());
                            println!(
                                "TimeBar finalized total_volume: {:.0}",
                                time_bar.total_volume
                            );

                            last_five_time_bars.push(time_bar.clone());
                            if last_five_time_bars.len() > 5 {
                                last_five_time_bars.remove(0);
                            }
                        }

                        // Start a new TimeBar
                        current_time_bar = Some(TimeBar::build_bar(
                            trade.event_time,
                            trade.price,
                            trade.is_buy,
                            trade.quantity,
                        ));
                    } else {
                        // Update the current TimeBar with the trade
                        let time_bar = current_time_bar.as_mut().unwrap();
                        time_bar.update_bar(trade.price, trade.is_buy, trade.quantity);
                    }

                    // If there's no current VolumeTimeBar or the trade's volume is outside the current VolumeTimeBar's volume window
                    if current_volume_time_bar.is_none()
                        || (last_five_time_bars.len() == 5
                            && (current_volume_time_bar.as_ref().unwrap().total_volume
                                > last_five_time_bars
                                    .iter()
                                    .map(|bar| bar.total_volume)
                                    .sum::<f64>()
                                    / 5.0))
                    {
                        // Finalize the current VolumeTimeBar and add it to the vector
                        if let Some(volume_time_bar) = current_volume_time_bar.take() {
                            volume_time_bars.push(volume_time_bar.clone());
                            println!("VTB: {}", volume_time_bar);
                            // Buy sell density calculation
                            // Update the BuySellDensity
                            buy_sell_density.update(&volume_time_bar);
                            println!("Buy/Sell Density: {:.2}", buy_sell_density.value());
                        }

                        // Start a new VolumeTimeBar
                        current_volume_time_bar = Some(VolumeTimeBar::build_bar(
                            trade.event_time,
                            trade.price,
                            trade.is_buy,
                            trade.quantity,
                        ));
                    } else {
                        // Update the current VolumeTimeBar with the trade
                        let volume_time_bar = current_volume_time_bar.as_mut().unwrap();
                        volume_time_bar.update_bar(trade.price, trade.is_buy, trade.quantity);
                    }
                }
                Err(e) => {
                    println!("Error parsing trade: {}", e);
                    // Handle the error...
                }
            }
        } else {
            // Handle other types of messages...
            println!("Received non-text message: {:?}", msg);
        }
    }
}
