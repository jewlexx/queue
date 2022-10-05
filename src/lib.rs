use std::future::Future;

pub struct QueueItem {
    func: Box<dyn Fn() + Send + Sync + 'static>,
    promise: Option<Box<dyn Future<Output = ()>>>,
}

pub struct Queue {
    queue: Vec<QueueItem>,
}
