use futures::{future, StreamExt, SinkExt};
use tokio_tungstenite::{connect_async, WebSocketStream};
use tokio_tungstenite::tungstenite::protocol::Message;
use futures_util::stream::{SplitSink, SplitStream};
use tokio::net::TcpStream;
use async_std::task;
use std::time::Duration;
use tungstenite::error::Error;


#[tokio::main]
async fn main() {
    let host = "ws://127.0.0.1:9001".to_string();
    let url = url::Url::parse(&host).unwrap();
    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");

    println!("WebSocket handshake has been successfully completed");
    let (write_stream, read_stream) = ws_stream.split();

    let ws_read = read_message(read_stream);
    let ws_write = write_message(write_stream);

    let result = future::join(ws_read, ws_write).await;

    match result {
        (Ok(_), Err(_)) => println!("Write Error"),
        (Err(_), Ok(_)) => println!("Read Error"),
        (Err(_), Err(_)) => println!("Read and Write Error"),
        (Ok(_), Ok(_)) => (),
    }
}

async fn read_message(mut read_stream: SplitStream<WebSocketStream<TcpStream>>) -> Result<(), Error> {
    while let Some(message) = read_stream.next().await {
        let message = message?;
        println!("{}", message);
    }

    return Ok(());
}

async fn write_message(mut write_stream: SplitSink<WebSocketStream<TcpStream>, Message>) -> Result<(), Error> {
    loop {
        write_stream.send(Message::Text(format!(r#"@sync(node:"/unit/foo",lane:info)"#))).await?;
        task::sleep(Duration::from_secs(10)).await;
    }
}
