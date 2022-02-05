use std::io::{self, Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::thread;

fn handle_client(mut stream: TcpStream) {
    let mut data = [0 as u8; 1450]; // using 50 byte buffer
    while match stream.read(&mut data) {
        Ok(size) => {
            // if size is 0, for tcp, we have reached an end
            if size == 0 {
                return;
            }
            io::stdout().write_all(&data[0..size]).unwrap();
            true
        }
        Err(_) => {
            eprintln!(
                "An error occurred, terminating connection with {}",
                stream.peer_addr().unwrap()
            );
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

pub fn server() {
    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
    // accept connections and process them, spawning a new thread for each one
    eprintln!("Server listening on port 3333");
    let mut stream1 = listener.incoming().next().unwrap().unwrap();

    eprintln!("New connection: {}", stream1.peer_addr().unwrap());
    handle_client(stream1);

    // close the socket server
    drop(listener);
}
