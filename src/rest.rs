use crate::OKEx;
use serde::{Deserialize, Deserializer, Serialize};
use std::str::FromStr;
use std::fmt::Display;
use std::collections::HashMap;

const API_HOST: &'static str = "https://www.okex.com/api/spot/v3/";

pub struct RestClient {
    client: reqwest::Client,
    api_key: String,
    secret: String,
}

impl OKEx for RestClient {
    fn new() -> Self {
        let timeout = std::time::Duration::from_secs(5);
        // TODO: Will this just panic if there is an error building the client?
        // Is that alright? Or should the use be able to handle to error.
        let client = reqwest::ClientBuilder::new()
            .timeout(timeout).build().unwrap();
        
        RestClient {
            client: client,
            api_key: String::new(),
            secret: String::new(),
        }
    }

    fn authenticate(&mut self, api_key: &str, secret: &str) {
        self.api_key.push_str(api_key);
        self.secret.push_str(secret);
    }
}

impl RestClient {
    fn get<T>(&self, endpoint: &str, params: &[(String, String)]) -> reqwest::Result<T> where for<'de> T : Deserialize<'de> {
        let request_url = format!("{}{}", API_HOST, endpoint);

        let mut response = self.client
            .get(&request_url)
            .query(params)
            .send()?;

        response.json()
    }

    // Spot Trading
    pub fn trading_pairs(&self) -> reqwest::Result<TradingPairs> {
        self.get("instruments", &[])
    }

    pub fn order_book(&self, instrument_id: &str, size: u64, depth: f64) 
    -> reqwest::Result<OrderBook> {
        let endpoint = format!("instruments/{}/book", instrument_id);
        let params = [
            (String::from("size"), size.to_string()), 
            (String::from("depth"), depth.to_string())
        ];

        self.get(&endpoint, &params)
    }

    pub fn trading_pair_information() {

    }

    pub fn filled_orders() {

    }

    pub fn market_data() {

    }
}

type TradingPairs = Vec<Instrument>;

#[derive(Deserialize, Debug)]
pub struct Instrument {
    base_currency: String,
    instrument_id: String,
    #[serde(deserialize_with = "from_str")]
    min_size: f64,
    quote_currency: String,
    #[serde(deserialize_with = "from_str")]
    size_increment: f64,
    #[serde(deserialize_with = "from_str")]
    tick_size: f64,
}

#[derive(Deserialize, Debug)]
pub struct OrderBook {
    asks: Vec<Order>,
    bids: Vec<Order>
}

#[derive(Deserialize, Debug)]
pub struct Order {
    #[serde(deserialize_with = "from_str")]
    price: f64,
    #[serde(deserialize_with = "from_str")]
    size: f64,
    #[serde(deserialize_with = "from_str")]
    num_orders: u64,
}

fn from_str<'de, T, D>(deserializer: D) -> Result<T, D::Error>
    where T: FromStr,
          T::Err: Display,
          D: Deserializer<'de>
{
    let s = String::deserialize(deserializer)?;
    T::from_str(&s).map_err(serde::de::Error::custom)
}