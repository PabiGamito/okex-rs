use crate::OKEx;
use serde::{Deserialize, Deserializer, Serialize};
use std::str::FromStr;
use std::fmt::Display;

pub struct RestClient {
    client: reqwest::Client,
    api_key: String,
    secret: String,
}

impl OKEx for RestClient {
    fn new() -> Self {
        let timeout = std::time::Duration::from_secs(5);
        // TODO: Figure out What happens if there is an err and it tries to unwrap?
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
    // Spot Trading
    pub fn trading_pairs(&self) -> reqwest::Result<()> {
        let request_url = format!("https://www.okex.com/api/spot/v3/{}", "instruments");
        
        let mut response = self.client.get(&request_url).send()?;

        let instruments: Vec<Instrument> = response.json()?;

        println!("Reponse :: {:?}", instruments[0]);

        Ok(())
    }

    pub fn order_book() {

    }

    pub fn trading_pair_information() {

    }

    pub fn filled_orders() {

    }

    pub fn market_data() {

    }
}

#[derive(Deserialize, Debug)]
struct Instrument {
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

fn from_str<'de, T, D>(deserializer: D) -> Result<T, D::Error>
    where T: FromStr,
          T::Err: Display,
          D: Deserializer<'de>
{
    let s = String::deserialize(deserializer)?;
    T::from_str(&s).map_err(serde::de::Error::custom)
}