use futures_util::stream::SplitStream;
use futures::StreamExt;
use tokio_tungstenite::{connect_async, WebSocketStream};
use tokio_tungstenite::tungstenite::protocol::Message;
use tokio::net::TcpStream;
use std::{thread, time};
use tokio::sync::mpsc;
use url;

struct Connection {}

impl Connection {
    async fn new(host: &str) -> mpsc::Sender<Message> {
        let url = url::Url::parse(&host).unwrap();
        let (ws_stream, _) = connect_async(&url).await.expect("Failed to connect");

        let (write_stream, read_stream) = ws_stream.split();
        let (tx, rx) = mpsc::channel(5);

        let receive = Connection::receive_messages(read_stream);
        let send = rx.map(Ok).forward(write_stream);

        tokio::spawn(send);
        tokio::spawn(receive);

        return tx;
    }

    async fn receive_messages(mut read_stream: SplitStream<WebSocketStream<TcpStream>>) {
        while let Some(message) = read_stream.next().await {
            if let Ok(m) = message {
                println!("{}", m);
            }
        }
    }

    async fn send_message(mut tx: mpsc::Sender<Message>, message: String) {
        tx.send(Message::text(message)).await.unwrap();
    }
}

#[tokio::main]
async fn main() {
    let tx = Connection::new("ws://127.0.0.1:9001").await;

    let tx_clone = mpsc::Sender::clone(&tx);
    tokio::spawn(async move { Connection::send_message(tx_clone, String::from(r#"@sync(node:"/unit/foo",lane:info)"#)).await });
    thread::sleep(time::Duration::from_secs(2));

    let tx_clone = mpsc::Sender::clone(&tx);
    tokio::spawn(async move { Connection::send_message(tx_clone, String::from(r#"@sync(node:"/unit/foo",lane:info)"#)).await });
    thread::sleep(time::Duration::from_secs(10))
}
