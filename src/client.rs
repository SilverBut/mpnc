use std::fmt::format;
use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::str::from_utf8;

use crate::netargs;

pub async fn client(addr: &str) {
    let s1 = format!("{}:3333", addr);

    let mut stream1;

    match TcpStream::connect(s1) {
        Ok(mut stream) => {
            stream1 = stream;
        }
        Err(e) => {
            eprintln!("Failed to connect: {}", e);
            return;
        }
    }
    loop {
        let mut buffer = [0u8; netargs::BLOCK_SIZE];
        let size = io::stdin().read(&mut buffer[..]).unwrap();
        if size == 0 {
            eprintln!("EOF");
            break;
        }
        stream1.write_all(&buffer[..size]).unwrap();
    }
    drop(stream1);
}
