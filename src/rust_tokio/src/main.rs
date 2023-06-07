use std::time::Duration;
use futures::{stream::FuturesUnordered, StreamExt};

// #[tokio::main]
#[tokio::main(flavor = "multi_thread", worker_threads = 1)]
async fn main() {
    // For tokio-console
    console_subscriber::init();

    let mut tasks = FuturesUnordered::new();

    // Worker Thread
    let h1 = tokio::spawn(async move {
        println!("[WorKer Thread]: Sleep Now...");
        tokio::time::sleep(Duration::from_secs(1)).await;
        println!("[WorKer Thread]: Wake Up!");
    });
    tasks.push(h1);
    println!("[WorKer Thread]: Start!");

    for i in 1..2000 {
        let h2 = tokio::spawn(async move {
            tokio::time::sleep(Duration::from_secs(1)).await;
            println!("{i} woke up!");
        });
        tasks.push(h2);
        println!("{i} ...");
    }

    while let Some(item) = tasks.next().await {
        let () = item.unwrap();
    }
    println!("Done");
}