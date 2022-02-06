use std::io::{self, Error, Write};

use std::pin::Pin;
use std::task::{Context, Poll};

use futures::{Stream, StreamExt};
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener, TcpStream};
use tokio_serde::{formats::MessagePack, Framed};
use tokio_util::codec::Framed as CodecFramed;
use tokio_util::codec::LengthDelimitedCodec;

use crate::connector::{ClientMessage, ServerMessage};

type ServerFramed = Framed<
    CodecFramed<TcpStream, LengthDelimitedCodec>,
    ClientMessage,
    ServerMessage,
    MessagePack<ClientMessage, ServerMessage>,
>;

struct Seller {
    connection: ServerFramed,
}

impl Seller {
    pub async fn new(tcp_stream: TcpStream) -> Result<Self, Error> {
        let length_delimited = CodecFramed::new(tcp_stream, LengthDelimitedCodec::new());

        let connection = Framed::new(length_delimited, MessagePack::default());

        Ok(Self { connection })
    }
}

impl Stream for Seller {
    type Item = Result<ClientMessage, Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let self_mut = &mut self.as_mut();

        match Pin::new(&mut self_mut.connection).poll_next(cx) {
            Poll::Ready(Some(val)) => Poll::Ready(Some(val.map_err(|err| err.into()))),
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}

async fn handle_client(stream1: TcpStream, stream2: TcpStream) -> Result<(), Error> {
    let mut seller1 = Seller::new(stream1).await?;
    let mut seller2 = Seller::new(stream2).await?;

    let mut last_read_stream_2 = true;
    let mut out = tokio::io::stdout();
    loop {
        let val;
        if last_read_stream_2 {
            val = seller1.next();
        } else {
            val = seller2.next();
        }
        last_read_stream_2 = !last_read_stream_2;
        if let Some(nval) = val.await {
            out.write_all(&nval.unwrap().data).await.unwrap();
        } else {
            break;
        }
    }
    out.flush().await.unwrap();

    Ok(())
}

pub async fn server() -> Result<(), Error> {
    let listener1 = TcpListener::bind("0.0.0.0:3333").await.unwrap();
    eprintln!("Server listening on port 3333");
    let listener2 = TcpListener::bind("0.0.0.0:3334").await.unwrap();
    eprintln!("Server listening on port 3334");
    let (stream1_rx, stream1_addr) = listener1.accept().await.unwrap();
    let (stream2_rx, stream2_addr) = listener2.accept().await.unwrap();

    eprintln!("New connection on A: {}", stream1_addr);
    eprintln!("New connection on B: {}", stream2_addr);
    handle_client(stream1_rx, stream2_rx).await?;

    // close the socket server
    drop(listener1);
    drop(listener2);
    Ok(())
}
