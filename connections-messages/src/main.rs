use futures_util::stream::SplitStream;
use futures::StreamExt;
use tokio_tungstenite::{connect_async, WebSocketStream};
use tokio_tungstenite::tungstenite::protocol::Message;
use tokio::net::TcpStream;
use std::{thread, time};
use tokio::sync::mpsc;
use url;
use std::future::Future;

struct Connection {
    url: url::Url,
    rx: mpsc::Receiver<Message>,
}

impl Connection {
    fn new(host: &str) -> (Connection, mpsc::Sender<Message>) {
        let url = url::Url::parse(&host).unwrap();
        let (tx, rx) = mpsc::channel(5);

        return (Connection { url, rx }, tx);
    }

    async fn open(self) {
        let (ws_stream, _) = tokio::spawn(connect_async(self.url)).await.unwrap().unwrap();
        let (write_stream, read_stream) = ws_stream.split();

        let receive = Connection::receive_messages(read_stream);
        let send = self.rx.map(Ok).forward(write_stream);

        tokio::spawn(send);
        tokio::spawn(receive);
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

struct Runtime {
    rt: tokio::runtime::Runtime,
}

impl Runtime {
    fn new() -> Runtime {
        let rt = tokio::runtime::Runtime::new().unwrap();
        return Runtime { rt };
    }

    fn schedule_task<F>(&self, task: impl Future<Output=F> + std::marker::Send + 'static)
        where F: std::marker::Send + 'static {
        &self.rt.spawn(async move { task.await });
    }
}


fn main() {
    let runtime = Runtime::new();
    let (connection, tx) = Connection::new("ws://127.0.0.1:9001");

    runtime.schedule_task(connection.open());
    let tx_clone = mpsc::Sender::clone(&tx);
    runtime.schedule_task(Connection::send_message(tx_clone, String::from(r#"@sync(node:"/unit/foo",lane:info)"#)));
    thread::sleep(time::Duration::from_secs(2));

    let tx_clone = mpsc::Sender::clone(&tx);
    runtime.schedule_task(Connection::send_message(tx_clone, String::from(r#"@sync(node:"/unit/foo",lane:info)"#)));
    thread::sleep(time::Duration::from_secs(10))
}
