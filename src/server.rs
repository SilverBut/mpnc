use std::{convert::TryInto, io::Error};

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
};

use crate::netargs;

async fn handle_client<T>(mut stream1: T, mut stream2: T) -> Result<(), Error>
where
    T: AsyncReadExt + std::marker::Unpin,
{
    let mut last_read_stream_2 = true;
    let mut out = tokio::io::stdout();
    loop {
        if last_read_stream_2 {
            if let Ok(size) = stream1.read_u64().await {
                let mut buffer = Vec::with_capacity(size.try_into().unwrap());
                unsafe {
                    buffer.set_len(size.try_into().unwrap());
                }
                stream1.read_exact(&mut buffer).await?;
                out.write_all(&buffer).await.unwrap();
            } else {
                eprintln!("die at A");
                break;
            }
        } else {
            if let Ok(size) = stream2.read_u64().await {
                let mut buffer = Vec::with_capacity(size.try_into().unwrap());
                unsafe {
                    buffer.set_len(size.try_into().unwrap());
                }
                stream2.read_exact(&mut buffer).await?;
                out.write_all(&buffer).await.unwrap();
            } else {
                eprintln!("die at B");
                break;
            }
        }
        last_read_stream_2 = !last_read_stream_2;
    }
    out.flush().await.unwrap();

    Ok(())
}

pub async fn server() -> Result<(), Error> {
    let s1 = format!("0.0.0.0:3333");
    let s2 = format!("0.0.0.0:3335");

    let listener1 = TcpListener::bind(s1).await.unwrap();
    eprintln!("Server listening on port 3333");
    let listener2 = TcpListener::bind(s2).await.unwrap();
    eprintln!("Server listening on port 3335");
    let (stream1_rx, stream1_addr) = listener1.accept().await.unwrap();
    let (stream2_rx, stream2_addr) = listener2.accept().await.unwrap();

    eprintln!("New connection on A: {}", stream1_addr);
    eprintln!("New connection on B: {}", stream2_addr);
    handle_client(
        BufReader::with_capacity(4 * netargs::BLOCK_SIZE, stream1_rx),
        BufReader::with_capacity(4 * netargs::BLOCK_SIZE, stream2_rx),
    )
    .await?;

    // close the socket server
    drop(listener1);
    drop(listener2);
    Ok(())
}
