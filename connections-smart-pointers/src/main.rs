use futures_util::stream::{SplitSink, SplitStream};
use futures::{StreamExt, SinkExt};
use tokio_tungstenite::{connect_async, WebSocketStream};
use tokio_tungstenite::tungstenite::protocol::Message;
use tokio::net::TcpStream;
use std::{thread, time};
use url;
use std::sync::Arc;
use tokio::sync::Mutex;
use std::time::Duration;
use async_std::task;

struct Connection {
    read_stream: Mutex<SplitStream<WebSocketStream<TcpStream>>>,
    write_stream: Mutex<SplitSink<WebSocketStream<TcpStream>, Message>>,
}

impl Connection {
    async fn new(host: &str) -> Connection {
        let url = url::Url::parse(&host).unwrap();
        let (ws_stream, _) = connect_async(&url).await.expect("Failed to connect");
        let (write_stream, read_stream) = ws_stream.split();
        return Connection { read_stream: Mutex::new(read_stream), write_stream: Mutex::new(write_stream) };
    }
    async fn receive_messages(&self) {
        while let Some(message) = self.read_stream.lock().await.next().await {
            let message = message.unwrap();
            println!("{}", message);
        }
    }

    async fn send_message(&self) {
        loop {
            self.write_stream.lock().await.send(Message::Text((r#"@sync(node:"/unit/foo",lane:info)"#).to_string())).await.unwrap();
            task::sleep(Duration::from_secs(2)).await;
        }
    }
}


fn main() {
    let mut runtime = tokio::runtime::Runtime::new().unwrap();

    let connection = Arc::new(runtime.block_on(Connection::new("ws://127.0.0.1:9001")));

    let cloned_connection = connection.clone();
    runtime.spawn(async move { cloned_connection.send_message().await; });
    let cloned_connection = connection.clone();
    runtime.spawn(async move { cloned_connection.receive_messages().await; });

    thread::sleep(time::Duration::from_secs(10));
}
