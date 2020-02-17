use std::time::Duration;
use async_std::task as async_task;
use std::{thread, time};
use std::future::Future;

struct SwimClient {
    runtime: tokio::runtime::Runtime,
}

impl SwimClient {
    fn new() -> SwimClient {
        let runtime = tokio::runtime::Runtime::new().unwrap();
        return SwimClient { runtime };
    }

    fn schedule_task<F>(&self, task: F)
        where
            F: Future + Send + 'static,
            F::Output: Send + 'static,
    {
        self.runtime.spawn(task);
    }
}

impl Drop for SwimClient {
    fn drop(&mut self) {
        println!("Stopping Swim client");
    }
}

fn main() {
    let swim_client = SwimClient::new();
    swim_client.schedule_task(count_by_one());
    swim_client.schedule_task(count_by_three());
    swim_client.schedule_task(count_by_five());

    thread::sleep(time::Duration::from_secs(10));
}


async fn count_by_one() {
    for i in 0..100 {
        async_task::sleep(Duration::from_secs(1)).await;
        println!("One({})", i)
    }
}

async fn count_by_three() {
    for i in (0..100).step_by(3) {
        async_task::sleep(Duration::from_secs(3)).await;
        println!("Three({})", i)
    }
}

async fn count_by_five() {
    for i in (0..100).step_by(5) {
        async_task::sleep(Duration::from_secs(5)).await;
        println!("Five({})", i)
    }
}
