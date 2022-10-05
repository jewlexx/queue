use std::future::Future;

pub struct QueueItem(
    Box<dyn Fn() + Send + Sync + 'static>,
    dyn Future<Output = ()>,
);

pub struct Queue {
    queue: Vec<QueueItem>,
}
