struct ConnectionPool<T: Producer> {
    producer: T,
}

impl<T: Producer> ConnectionPool<T> {
    fn new(producer: T) -> ConnectionPool<T> {
        ConnectionPool { producer }
    }

    fn create_connection(self) {
        self.producer.create_new().print();
    }
}

trait Producer {
    type T: Connectable;
    fn create_new(self) -> Self::T;
}

struct FooProducer {}

impl Producer for FooProducer {
    type T = FooConnection;

    fn create_new(self) -> FooConnection {
        FooConnection::new()
    }
}

struct BarProducer {}

impl Producer for BarProducer {
    type T = BarConnection;

    fn create_new(self) -> BarConnection {
        BarConnection::new()
    }
}

trait Connectable {
    fn new() -> Self;
    fn print(self);
}

struct FooConnection {}

impl Connectable for FooConnection {
    fn new() -> FooConnection {
        FooConnection {}
    }

    fn print(self) {
        println!("Foo")
    }
}

struct BarConnection {}

impl Connectable for BarConnection {
    fn new() -> BarConnection {
        BarConnection {}
    }

    fn print(self) {
        println!("Bar")
    }
}

fn main() {
    let foo_pool = ConnectionPool::new(FooProducer {});
    let bar_pool = ConnectionPool::new(BarProducer {});

    foo_pool.create_connection();
    bar_pool.create_connection();
}
