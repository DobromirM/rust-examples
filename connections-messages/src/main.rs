use futures_util::stream::SplitStream;
use futures::StreamExt;
use tokio_tungstenite::{connect_async, WebSocketStream};
use tokio_tungstenite::tungstenite::protocol::Message;
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use std::{thread, time};
use std::error::Error;
use url;
use std::future::Future;
use futures::future::FutureExt;
use std::fmt;

struct Connection {
    url: url::Url,
    rx: mpsc::Receiver<Message>,
}

impl Connection {
    fn new(host: &str) -> Result<(Connection, mpsc::Sender<Message>), ConnectionError> {
        let url = url::Url::parse(&host)?;
        let (tx, rx) = mpsc::channel(5);

        return Ok((Connection { url, rx }, tx));
    }

    async fn open(self) -> Result<(), ConnectionError> {
        let (ws_stream, _) = tokio::spawn(connect_async(self.url)).await??;
        let (write_stream, read_stream) = ws_stream.split();

        let receive = Connection::receive_messages(read_stream);
        let send = self.rx.map(Ok).forward(write_stream);

        tokio::spawn(send);
        tokio::spawn(receive);
        return Ok(());
    }

    async fn receive_messages(mut read_stream: SplitStream<WebSocketStream<TcpStream>>) {
        while let Some(message) = read_stream.next().await {
            if let Ok(m) = message {
                println!("{}", m);
            }
        }
    }

    async fn send_message(mut tx: mpsc::Sender<Message>, message: String) -> Result<(), ConnectionError> {
        tx.send(Message::text(message)).await?;
        return Ok(());
    }
}

#[derive(Debug, Clone)]
pub enum ConnectionError
{
    ParseError,
    ConnectError,
    SendMessageError,

}

impl Error for ConnectionError {}

impl fmt::Display for ConnectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConnectionError::ParseError => write!(f, "Parse error!"),
            ConnectionError::ConnectError => write!(f, "Connect error!"),
            ConnectionError::SendMessageError => write!(f, "Send message error!")
        }
    }
}

impl From<url::ParseError, > for ConnectionError {
    fn from(_: url::ParseError) -> Self {
        ConnectionError::ParseError
    }
}

impl From<tokio::task::JoinError, > for ConnectionError {
    fn from(_: tokio::task::JoinError) -> Self {
        ConnectionError::ConnectError
    }
}

impl From<tungstenite::error::Error, > for ConnectionError {
    fn from(_: tungstenite::error::Error) -> Self {
        ConnectionError::ConnectError
    }
}

impl From<tokio::sync::mpsc::error::SendError<Message>, > for ConnectionError {
    fn from(_: tokio::sync::mpsc::error::SendError<Message>) -> Self {
        ConnectionError::SendMessageError
    }
}


struct Client {
    rt: tokio::runtime::Runtime,
}


impl Client {
    fn new() -> Client {
        let rt = tokio::runtime::Runtime::new().unwrap();
        return Client { rt };
    }

    fn schedule_task<F>(&self, task: impl Future<Output=Result<F, ConnectionError>> + Send + 'static)
        where F: Send + 'static {
        &self.rt.spawn(async move { task.await }.inspect(|response| {
            match response {
                Err(e) => println!("{}", e),
                Ok(_) => ()
            }
        }));
    }
}


fn main() {
    let client = Client::new();
    let (connection, tx) = Connection::new("ws://127.0.0.1:9001").unwrap();

    client.schedule_task(connection.open());
    let tx_clone = mpsc::Sender::clone(&tx);
    client.schedule_task(Connection::send_message(tx_clone, String::from(r#"@sync(node:"/unit/foo",lane:info)"#)));
    thread::sleep(time::Duration::from_secs(2));

    let tx_clone = mpsc::Sender::clone(&tx);
    client.schedule_task(Connection::send_message(tx_clone, String::from(r#"@sync(node:"/unit/foo",lane:info)"#)));
    thread::sleep(time::Duration::from_secs(1))
}
