use std::time::Duration;

async fn hello_delay(task: u64, time: u64) {
    println!("Task {task} has started");

    // Block the thread
    // std::thread::sleep(Duration::from_millis(time));
    
    // Async sleep not blocking thread
    tokio::time::sleep(Duration::from_millis(time)).await;
    println!("Task {task} is done.");
}

#[tokio::main]
async fn main() {
    let mut futures = Vec::new();
    for i in 0..5 {
        futures.push(hello_delay(i, 500 * i));
    };
    futures::future::join_all(futures).await;
}