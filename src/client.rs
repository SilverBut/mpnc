use std::fmt::format;
use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::str::from_utf8;

pub fn client(addr: &str) {
    let s1 = format!("{}:3333", addr);

    let mut stream1;

    match TcpStream::connect(s1) {
        Ok(mut stream) => {
            stream1 = stream;
            // println!("Successfully connected to server in port 3333");

            // let msg = b"Hello!";

            // stream.write(msg).unwrap();
            // println!("Sent Hello, awaiting reply...");

            // let mut data = [0 as u8; 6]; // using 6 byte buffer
            // match stream.read_exact(&mut data) {
            //     Ok(_) => {
            //         if &data == msg {
            //             println!("Reply is ok!");
            //         } else {
            //             let text = from_utf8(&data).unwrap();
            //             println!("Unexpected reply: {}", text);
            //         }
            //     }
            //     Err(e) => {
            //         println!("Failed to receive data: {}", e);
            //     }
            // }
        }
        Err(e) => {
            eprintln!("Failed to connect: {}", e);
            return;
        }
    }
    loop {
        let mut buffer = [0u8; 1450];
        let size = io::stdin().read(&mut buffer[..]).unwrap();
        if size == 0 {
            eprintln!("EOF");
            return;
        }
        stream1.write(&buffer[..size]).unwrap();
    }
}
