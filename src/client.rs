use crate::{connector, netargs};
use futures::SinkExt;

use std::io::{self, Error, Read};

pub async fn client(addr: &str) -> Result<(), Error> {
    let s1 = format!("{}:3333", addr);
    let s2 = format!("{}:3334", addr);

    let mut stream1_tx = connector::Dealer::new(s1.as_str()).await?;
    let mut stream2_tx = connector::Dealer::new(s2.as_str()).await?;

    let mut last_write_stream_2 = true;
    loop {
        let mut buffer = vec![0u8; netargs::BLOCK_SIZE];
        let size = io::stdin().read(&mut buffer[..]).unwrap();
        if size == 0 {
            eprintln!("EOF");
            break;
        }
        let msg = connector::ServerMessage {
            data: buffer[..size].to_vec(),
        };
        if last_write_stream_2 {
            stream1_tx.feed(msg).await?;
        } else {
            stream2_tx.feed(msg).await?;
        }
        last_write_stream_2 = !last_write_stream_2
    }
    stream1_tx.flush().await.unwrap();
    stream2_tx.flush().await.unwrap();
    Ok(())
}
