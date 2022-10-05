use std::task::Poll;

pub trait QueueFn: Fn() + Sized + Send + Sync + 'static {}

pub struct QueueItem<F> {
    func: F,
    promise: Option<Poll<()>>,
}

impl<F: QueueFn> QueueItem<F> {
    pub fn new(func: F) -> Self {
        Self {
            func,
            promise: None,
        }
    }

    pub fn run(&self) {
        (self.func)();
    }

    pub(crate) fn execution(&self) -> Option<&Poll<()>> {
        self.promise.as_ref()
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
