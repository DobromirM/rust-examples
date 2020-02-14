use futures::executor;
use async_std::task;
use std::time::Duration;
use futures;
use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        executor::block_on(async_main());
    });

    println!("Stopping client..");
    handle.join().unwrap();
}


async fn async_main() {
    futures::join!(count_by_one(), count_by_three(), count_by_five());
}


async fn count_by_one() {
    for i in 0..100 {
        task::sleep(Duration::from_secs(1)).await;
        println!("One({})", i)
    }
}

async fn count_by_three() {
    for i in (0..100).step_by(3) {
        task::sleep(Duration::from_secs(3)).await;
        println!("Three({})", i)
    }
}

async fn count_by_five() {
    for i in (0..100).step_by(5) {
        task::sleep(Duration::from_secs(5)).await;
        println!("Five({})", i)
    }
}
