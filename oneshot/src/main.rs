use futures::channel::oneshot;
use std::collections::HashMap;

struct ConnectionPool {
    connections: HashMap<String, i32>
}

impl ConnectionPool {
    fn new() -> ConnectionPool {
        return ConnectionPool { connections: HashMap::new() };
    }

    fn get_connection(&mut self, host: &str, sender: futures::channel::oneshot::Sender<i32>) {
        if !self.connections.contains_key(host) {
            self.connections.insert(host.to_string(), 32);
        }

        sender.send(self.connections.get(host).unwrap().clone());
    }
}


fn main() {
    let (sender, mut receiver) = oneshot::channel::<i32>();
    let mut connection_pool = ConnectionPool::new();

    connection_pool.get_connection("123", sender);

    println!("{:?}", receiver.try_recv().unwrap().unwrap());
}
