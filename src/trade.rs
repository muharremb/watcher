use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Trade {
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(deserialize_with = "from_str_to_f64", rename = "p")]
    pub price: f64,
    #[serde(deserialize_with = "from_str_to_f64", rename = "q")]
    pub quantity: f64,
    #[serde(deserialize_with = "from_bool_to_opposite", rename = "m")]
    pub is_buy: bool,
}

fn from_str_to_f64<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: String = serde::Deserialize::deserialize(deserializer)?;
    s.parse::<f64>().map_err(serde::de::Error::custom)
}

fn from_bool_to_opposite<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let b: bool = serde::Deserialize::deserialize(deserializer)?;
    Ok(!b)
}
