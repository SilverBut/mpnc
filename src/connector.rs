use futures::{Sink};
use serde::{Deserialize, Serialize};
use std::io::Error;
use std::pin::Pin;
use std::task::{Context, Poll};

use tokio::net::TcpStream;
use tokio_serde::{formats::MessagePack, Framed};
use tokio_util::codec::Framed as CodecFramed;
use tokio_util::codec::LengthDelimitedCodec;

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct ClientMessage {
    pub data: Vec<u8>,
}

pub type ServerMessage = ClientMessage;

type ClientFramed = Framed<
    CodecFramed<TcpStream, LengthDelimitedCodec>,
    ClientMessage,
    ServerMessage,
    MessagePack<ClientMessage, ServerMessage>,
>;

pub struct Dealer {
    connection: ClientFramed,
}

impl Dealer {
    pub async fn new(addr: &str) -> Result<Self, Error> {
        let tcp_stream = TcpStream::connect(addr).await?;

        let length_delimited = CodecFramed::new(tcp_stream, LengthDelimitedCodec::new());

        let connection = Framed::new(length_delimited, MessagePack::default());

        Ok(Self { connection })
    }
}

impl Sink<ServerMessage> for Dealer {
    type Error = Error;

    fn poll_ready(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        let self_mut = &mut self.as_mut();
        match Pin::new(&mut self_mut.connection).poll_ready(cx) {
            Poll::Ready(val) => Poll::Ready(val.map_err(|err| err.into())),
            Poll::Pending => Poll::Pending,
        }
    }

    fn start_send(mut self: Pin<&mut Self>, item: ServerMessage) -> Result<(), Self::Error> {
        let self_mut = &mut self.as_mut();
        Ok(Pin::new(&mut self_mut.connection).start_send(item)?)
    }
    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        let self_mut = &mut self.as_mut();
        match Pin::new(&mut self_mut.connection).poll_flush(cx) {
            Poll::Ready(val) => Poll::Ready(val.map_err(|err| err.into())),
            Poll::Pending => Poll::Pending,
        }
    }
    fn poll_close(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        let self_mut = &mut self.as_mut();
        match Pin::new(&mut self_mut.connection).poll_close(cx) {
            Poll::Ready(val) => Poll::Ready(val.map_err(|err| err.into())),
            Poll::Pending => Poll::Pending,
        }
    }
}
