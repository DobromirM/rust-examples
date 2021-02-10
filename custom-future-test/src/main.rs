use async_std::task;
use futures::task::{Context, Poll};
use std::future::Future;
use std::time::Duration;
use tokio::macros::support::Pin;

#[tokio::main]
async fn main() {
    let str = HelloFuture::new(hello()).await;

    println!("{}", str)
}

async fn hello() -> String {
    task::sleep(Duration::from_secs(1)).await;
    return "Hello, world!".to_string();
}

struct HelloFuture<Fut>
where
    Fut: Future<Output = String> + Unpin,
{
    fut: Fut,
}

impl<Fut> HelloFuture<Pin<Box<Fut>>>
where
    Fut: Future<Output = String>,
{
    fn new(fut: Fut) -> HelloFuture<Pin<Box<Fut>>> {
        HelloFuture { fut: Box::pin(fut) }
    }
}

impl<Fut> Future for HelloFuture<Fut>
where
    Fut: Future<Output = String> + Unpin,
{
    type Output = String;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut_self = self.get_mut();
        Pin::new(&mut mut_self.fut).poll(cx)
    }
}
