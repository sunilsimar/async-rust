use tokio::task::JoinSet;

async fn ticker() {
    for i in 0..10 {
        println!("tick {i}");
    }
}

async fn tocker() {
    for i in 0..10 {
        println!("tock {i}");
    }
}

async fn double(n: i32) -> i32 {
    n * 2
}

async fn hello() {
    println!("Hello from async");

    // Use the tokio::! macro
    let result = tokio::join!(double(2), double(3));
    println!("{result:?}");

    // You can still use futures join_all
    let futures = vec![double(2), double(3)];
    let result = futures::future::join_all(futures).await;
    println!("{result:?}");

    // Using Tokio JoinSet
    let mut set = JoinSet::new();
    for i in 0..10 {
        set.spawn(double(i));
    }

    while let Some(res) = set.join_next().await {
        println!("{res:?}");
    }
}

#[tokio::main]
// #[tokio::main(flavor = "current_thread")]
async fn main() {
    // hello().await;
    let _ = tokio::join!(
        tokio::spawn(hello()),
        tokio::spawn(ticker()),
    );
    println!("Finished");
}