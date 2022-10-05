use std::future::Future;

pub type QueueFn = dyn Fn() + Send + Sync + 'static;

pub struct QueueItem {
    func: Box<QueueFn>,
    promise: Option<Box<dyn Future<Output = ()>>>,
}

impl QueueItem {
    pub fn new(func: &'static QueueFn) -> Self {
        Self {
            func: Box::new(func),
            promise: None,
        }
    }

    pub fn run(&self) {
        (self.func)();
    }

    pub(crate) fn execution(&self) -> Option<&dyn Future<Output = ()>> {
        self.promise.as_ref().map(|p| p.as_ref())
    }
}

#[derive(Default)]
pub struct Queue {
    queue: Vec<QueueItem>,
}

impl Queue {
    pub const fn new() -> Self {
        Self { queue: Vec::new() }
    }

    pub fn add(&mut self, func: &QueueFn) {
        let item = QueueItem::new(func);
        self.queue.push(item);
    }
}
