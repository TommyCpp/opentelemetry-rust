use std::thread;
// src/main.rs
use std::time::Duration;
use futures_channel::oneshot;

#[tokio::main]
async fn main() {
    // spawn a new tokio background tasks
    let (tx, rx) = oneshot::channel::<()>();

    let (mtx, mut mrx) = tokio::sync::mpsc::channel::<oneshot::Sender<()>>(10);

    thread::spawn(move || {
        // Create a new Runtime to run tasks
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .thread_name("dedicate")
            .worker_threads(1)
            .build()
            .expect("Creating Tokio runtime");

        // Pull task requests off the channel and send them to the executor
        runtime.block_on(async move {
            while let Some(mut tx) = mrx.recv().await {
                println!("send back");
                tx.send(()).unwrap();
            }
        });

    });
    // NB: This spawn is load bearing
    let _ = tokio::spawn(async move {
        // Do some work

        println!("finished trace_provider flush");
        mtx.send(tx).await.expect("send tx to mtx");
        futures_executor::block_on(rx).expect("rx");
        println!("start trace_provider flush");
        // tokio::time::sleep(Duration::from_secs(10)).await;
    })
    .await;
}
