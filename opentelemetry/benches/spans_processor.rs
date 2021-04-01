use criterion::{criterion_group, criterion_main, Criterion};
use std::time::Duration;
use futures::{FutureExt, Future, SinkExt, StreamExt};

fn criterion_benchmark(c: &mut Criterion) {
    let iteration = 1000;
    let batch_size = 30;
    let max_queue_size = 100;
    let delay = Duration::from_secs(20);
    let export_time = Duration::from_millis(2);

    let mut group = c.benchmark_group("batch processor");
    group.sample_size(10);

    group.bench_function("not wait for batch", |b| {
        let runtime = tokio::runtime::Builder::new_multi_thread().enable_all().build().unwrap();
        b.iter(|| {
            runtime.block_on(async move {
                let (mut app_sender, app_receiver) = futures::channel::mpsc::channel::<i32>(max_queue_size);
                let (mut collect_sender, mut collect_receiver) = futures::channel::mpsc::channel(max_queue_size);

                // app task
                let app_task = tokio::task::spawn(async move {
                    for _ in 0i32..iteration {
                        app_sender.try_send(1);
                        tokio::time::sleep(Duration::from_millis(10));
                    }
                    let _ = app_sender.send(2).await;
                });

                // collect task
                let collect_task = tokio::task::spawn(Box::pin(async move {
                    let mut fused_receiver = app_receiver.fuse();
                    let mut batch = Vec::with_capacity(batch_size);
                    loop {
                        let mut countdown = Box::pin(tokio::time::sleep(delay).fuse());
                        let next_step = futures::select! {
                            data = fused_receiver.next() => {
                                if data == Some(2) {
                                    Some(vec![Some(2)])
                                } else {
                                    batch.push(data);
                                    if batch.len() == batch_size {
                                        Some(std::mem::replace(&mut batch,
                                                          Vec::with_capacity(batch_size)))
                                    } else {
                                        None
                                    }
                                }
                            },
                            _ = countdown =>{
                                None
                            }
                        };
                        if let Some(data) = next_step {
                            if data.len() == 1 && *data.get(0).unwrap() == Some(2) {
                                return;
                            } else {
                                collect_sender.try_send(data);
                            }
                        }
                    }
                }));

                let export_task = tokio::task::spawn(Box::pin(async move {
                    while let Some(message) = collect_receiver.next().await {
                        tokio::time::sleep(export_time).await;
                    }
                }));

                futures::join!(app_task, collect_task, export_task);
            });
        })
    });

    group.bench_function("wait for batch", |b| {
        let runtime = tokio::runtime::Builder::new_multi_thread().enable_all().build().unwrap();
        b.iter(|| {
            runtime.block_on(async move {
                let (mut app_sender, app_receiver) = futures::channel::mpsc::channel(batch_size * max_queue_size);

                // app task
                let app_task = tokio::task::spawn(async move {
                    for _ in 0i32..iteration {
                        app_sender.try_send(1);
                        tokio::time::sleep(Duration::from_millis(10));
                    }
                    app_sender.try_send(2)
                });

                // collect task
                let collect_task = tokio::task::spawn(Box::pin(async move {
                    let mut fused_receiver = app_receiver.fuse();
                    let mut batch = Vec::with_capacity(batch_size);
                    for _ in 0i32..iteration {
                        let mut countdown = Box::pin(tokio::time::sleep(delay).fuse());
                        let next_step = futures::select! {
                            data = fused_receiver.next() => {
                                if data == Some(2) {
                                    Some(vec![Some(2)])
                                } else {
                                    batch.push(data);
                                    if batch.len() == batch_size {
                                        Some(std::mem::replace(&mut batch,
                                                          Vec::with_capacity(batch_size)))
                                    } else {
                                        None
                                    }
                                }
                            },
                            _ = countdown =>{
                                None
                            }
                        };
                        if let Some(data) = next_step {
                            if data.len() == 1 && *data.get(0).unwrap() == Some(2) {
                                return;
                            } else {
                                tokio::time::sleep(export_time).await;
                            }
                        }
                    }
                }));

                futures::join!(app_task, collect_task);
            });
        })
    });

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);