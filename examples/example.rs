use okex::OKEx;
use okex::rest::RestClient;
use okex::websocket::WSClient;

fn main() {
    let restClient: RestClient = OKEx::new();
    let webSocketClient: WSClient = OKEx::new();

    println!("Calling trading pairs!");
    println!("Got {:?}", restClient.trading_pairs());
    println!("Called trading pairs!");
}