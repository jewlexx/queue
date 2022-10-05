use std::task::Poll;

pub trait QueueFn: Fn() + Sized + Send + Sync + 'static {}

pub struct QueueItem<F: QueueFn> {
    func: F,
    thread: Option<std::thread::JoinHandle<()>>,
}

impl<F: QueueFn> QueueItem<F> {
    pub fn new(func: F) -> Self {
        Self { func, thread: None }
    }

    pub fn run(&self) {
        (self.func)();
    }

    pub fn is_finished(&self) -> bool {
        if let Some(ref thread) = self.thread {
            thread.is_finished()
        } else {
            false
        }
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

    pub async fn execute(&mut self) {
        loop {
            self.queue.retain(|item| {
                if item.is_finished() {
                    self.used_threads -= 1;
                    false
                } else {
                    true
                }
            });

            let unstarted = self
                .queue
                .iter()
                .enumerate()
                .filter_map(|(index, thread)| thread.thread.map(|_| (index, thread)));

            for (i, item) in unstarted {
                let func = Box::new(&item.func);
            }
        }
    }
}
