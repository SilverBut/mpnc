use crate::netargs;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, BufReader, BufWriter},
    net::TcpStream,
};

use std::{convert::TryInto, io::Error};

pub async fn client(addr: &str) -> Result<(), Error> {
    let s1 = format!("{}:3333", addr);
    let s2 = format!("{}:3334", addr);

    let mut stream1_tx = TcpStream::connect(s1.as_str()).await?;
    let mut stream2_tx = TcpStream::connect(s2.as_str()).await?;

    let mut last_write_stream_2 = true;
    let mut input = BufReader::new(tokio::io::stdin());

    loop {
        // let mut buffer = vec![0u8; netargs::BLOCK_SIZE];
        let mut buffer = Vec::with_capacity(netargs::BLOCK_SIZE);
        unsafe {
            buffer.set_len(netargs::BLOCK_SIZE);
        }
        let size = input.read(&mut buffer).await.unwrap();
        if size == 0 {
            eprintln!("EOF");
            break;
        }
        if last_write_stream_2 {
            stream1_tx = go(stream1_tx, size, buffer).await?;
        } else {
            stream2_tx = go(stream2_tx, size, buffer).await?;
        }

        last_write_stream_2 = !last_write_stream_2;
    }
    stream1_tx.flush().await.unwrap();
    stream2_tx.flush().await.unwrap();
    Ok(())
}

async fn go<T>(mut tx: T, size: usize, buffer: Vec<u8>) -> Result<T, Error>
where
    T: tokio::io::AsyncWriteExt + std::marker::Unpin,
{
    tx.write_u64(size.try_into().unwrap()).await?;
    tx.write_all(&buffer[..size]).await?;
    Ok(tx)
}
