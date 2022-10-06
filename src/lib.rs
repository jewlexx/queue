use parking_lot::Mutex;

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
    queue: Mutex<Vec<QueueItem<F>>>,
    max_threads: Mutex<usize>,
    used_threads: Mutex<usize>,
}

impl<F: QueueFn> Default for Queue<F> {
    fn default() -> Self {
        Self::new(1)
    }
}

impl<F: QueueFn> Queue<F> {
    pub const fn new(max_threads: usize) -> Self {
        Self {
            queue: Mutex::new(Vec::new()),
            max_threads: Mutex::new(max_threads),
            used_threads: Mutex::new(0),
        }
    }

    pub fn add(&self, func: F) {
        let item = QueueItem::new(func);
        self.queue.lock().push(item);
    }

    pub async fn execute(&self) {
        loop {
            if let Some(queue) = self.queue.try_lock() {
                queue.retain(|item| !item.is_finished());

                for mut item in queue.iter() {
                    let func = Box::new(&item.func);

                    let thread = std::thread::spawn(func);
                }
            };
        }
    }
}
