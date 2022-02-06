use crate::{connector, netargs};
use futures::SinkExt;
use tokio::io::AsyncReadExt;

use std::io::{self, Error, Read};

pub async fn client(addr: &str) -> Result<(), Error> {
    let s1 = format!("{}:3333", addr);
    let s2 = format!("{}:3334", addr);

    let mut stream1_tx = connector::Dealer::new(s1.as_str()).await?;
    let mut stream2_tx = connector::Dealer::new(s2.as_str()).await?;

    let mut last_write_stream_2 = true;
    let mut input = tokio::io::stdin();
    loop {
        let mut buffer = vec![0u8; netargs::BLOCK_SIZE];
        let size = input.read(&mut buffer).await.unwrap();
        if size == 0 {
            eprintln!("EOF");
            break;
        }
        buffer.truncate(size);
        let msg = connector::ServerMessage { data: buffer };
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
