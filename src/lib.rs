use std::future::Future;

pub trait QueueFn: Fn() + Send + Sync + 'static {}

pub struct QueueItem<F> {
    func: Box<F>,
    promise: Option<Box<dyn Future<Output = ()>>>,
}

impl<F: QueueFn> QueueItem<F> {
    pub fn new(func: F) -> Self {
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
pub struct Queue<F> {
    queue: Vec<QueueItem<F>>,
}

impl<F: QueueFn> Queue<F> {
    pub const fn new() -> Self {
        Self { queue: Vec::new() }
    }

    pub fn add(&mut self, func: F) {
        let item = QueueItem::new(func);
        self.queue.push(item);
    }
}
