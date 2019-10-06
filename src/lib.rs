pub mod rest;
pub mod websocket;

pub trait OKEx : Sized {
    fn new() -> Self;
    fn authenticate(&mut self, api_key: &str, private: &str);
}