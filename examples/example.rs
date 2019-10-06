use okex::OKEx;
use okex::rest::RestClient;
use okex::websocket::WSClient;

fn main() {
    let restClient: RestClient = OKEx::new();
    let webSocketClient: WSClient = OKEx::new();

    println!("{:?}", restClient.trading_pairs());
    println!("{:?}", restClient.order_book("BTC-USDT", 5, 0.2));
}