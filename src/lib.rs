use std::task::Poll;

pub trait QueueFn: Fn() + Sized + Send + Sync + 'static {}

pub struct QueueItem<F: QueueFn> {
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

pub struct Queue<F: QueueFn> {
    queue: Vec<QueueItem<F>>,
    max_threads: usize,
    used_threads: usize,
}

impl<F: QueueFn> Default for Queue<F> {
    fn default() -> Self {
        Self::new(1)
    }
}

impl<F: QueueFn> Queue<F> {
    pub const fn new(max_threads: usize) -> Self {
        Self {
            queue: Vec::new(),
            max_threads,
            used_threads: 0,
        }
    }

    pub fn add(&mut self, func: F) {
        let item = QueueItem::new(func);
        self.queue.push(item);
    }
}
