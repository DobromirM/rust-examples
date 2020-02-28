use std::collections::HashMap;
use tokio::sync::mpsc;
use futures::StreamExt;
use tokio_tungstenite::{connect_async, WebSocketStream};
use tokio_tungstenite::tungstenite::protocol::Message;
use url;
use std::{thread, time};
use futures_util::stream::SplitStream;
use tokio::net::TcpStream;

struct ConnectionPool {
    connections: HashMap<String, ConnectionHandler>,
    rx: mpsc::Receiver<ConnectionPoolMessage>,
}

type ConnectionPoolMessage = (String, String);

impl ConnectionPool {
    fn new() -> (ConnectionPool, ConnectionPoolHandler) {
        let (tx, rx) = mpsc::channel(5);

        return (ConnectionPool { connections: HashMap::new(), rx }, ConnectionPoolHandler { tx });
    }

    fn open(mut self) {
        let handle_messages = async move {
            loop {
                let response = self.rx.recv().await;

                match response {
                    Some((host, message)) => {
                        let handler = self.get_connection(&host).await;
                        handler.send_message(&message).await;
                    }
                    None => ()
                }
            }
        };

        tokio::spawn(handle_messages);
    }

    async fn get_connection(&mut self, host: &str) -> &mut ConnectionHandler {
        if !self.connections.contains_key(host) {
            let (connection, connection_handler) = Connection::new(host);
            connection.open().await;
            self.connections.insert(host.to_string(), connection_handler);
        }
        return self.connections.get_mut(host).unwrap();
    }

    async fn receive_message(host: &str, message: Message) {
        println!("Host: {:?}", host);
        println!("Message: {:?}", message.to_text().unwrap());
        // TODO this will call the `receive_message` of the Router
    }
}

struct ConnectionPoolHandler {
    tx: mpsc::Sender<ConnectionPoolMessage>
}

impl ConnectionPoolHandler {
    fn send_message(&mut self, host: &str, message: &str) {
        self.tx.try_send((host.to_string(), message.to_string())).unwrap();
    }
}

struct Connection {
    url: url::Url,
    rx: mpsc::Receiver<Message>,
}

impl Connection {
    fn new(host: &str) -> (Connection, ConnectionHandler) {
        let url = url::Url::parse(&host).unwrap();
        let (tx, rx) = mpsc::channel(5);

        return (Connection { url, rx }, ConnectionHandler { tx });
    }

    async fn open(self) {
        let (ws_stream, _) = connect_async(&self.url).await.unwrap();
        let (write_stream, read_stream) = ws_stream.split();

        let receive = Connection::receive_messages(read_stream, self.url.to_string().to_owned());
        let send = self.rx.map(Ok).forward(write_stream);

        tokio::spawn(receive);
        tokio::spawn(send);
    }

    async fn receive_messages(read_stream: SplitStream<WebSocketStream<TcpStream>>, host: String) {
        read_stream.for_each(|response| async {
            if let Ok(message) = response {
                ConnectionPool::receive_message(&host, message).await;
            }
        }).await;
    }
}

#[derive(Debug, Clone)]
struct ConnectionHandler {
    tx: mpsc::Sender<Message>,
}

impl ConnectionHandler {
    async fn send_message(&mut self, message: &str) {
        self.tx.send(Message::text(message)).await.unwrap();
    }
}

#[tokio::main]
async fn main() {
    let (connection_pool, mut handler) = ConnectionPool::new();
    connection_pool.open();

    handler.send_message("ws://127.0.0.1:9001", "@sync(node:\"/unit/foo\", lane:\"info\")");
    handler.send_message("ws://127.0.0.1:9001", "@sync(node:\"/unit/foo\", lane:\"info\")");
    thread::sleep(time::Duration::from_secs(10));
}
