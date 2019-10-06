use crate::OKEx;

pub struct WSClient {

}

impl OKEx for WSClient {
    fn new() -> Self {
        WSClient {

        }
    }

    fn authenticate(&mut self, api_key: &str, private: &str) {
        
    }
}