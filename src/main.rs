mod client;
mod server;

fn main() {
    match std::env::args().nth(1).expect("no mode given").as_str() {
        "server" => server::server(),
        "client" => client::client(std::env::args().nth(2).expect("no ip given").as_str()),
        _ => panic!("server or client?"),
    }
}
