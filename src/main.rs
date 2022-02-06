mod client;
mod netargs;
mod server;

#[tokio::main]
async fn main() {
    let ve = match std::env::args().nth(1).expect("no mode given").as_str() {
        "server" => server::server().await,
        "client" => client::client(std::env::args().nth(2).expect("no ip given").as_str()).await,
        _ => panic!("server or client?"),
    };
    ve.unwrap();
}
