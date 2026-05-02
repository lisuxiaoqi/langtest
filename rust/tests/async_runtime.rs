use std::future::Future;
use std::pin::Pin;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

struct Task {
    future: Mutex<Pin<Box<dyn Future<Output = ()> + Send + 'static>>>,
    tx: Sender<Arc<Task>>,
}

fn into_task<T>(f: T, sender: Sender<Arc<Task>>) -> Task
where
    T: Future<Output = ()> + Send + 'static,
{
    Task {
        future: Mutex::new(Box::pin(f)),
        tx: sender,
    }
}

fn into_waker(task: Arc<Task>) -> Waker {
    let data = Arc::into_raw(task) as *const ();
    static VTABLE: RawWakerVTable = RawWakerVTable::new(clone, wake, wake_by_ref, drop);

    unsafe fn clone(data: *const ()) -> RawWaker {
        let task = Arc::<Task>::from_raw(data as *const Task);
        let cloned = Arc::clone(&task);
        std::mem::forget(task);
        RawWaker::new(Arc::into_raw(cloned) as *const (), &VTABLE)
    }

    unsafe fn wake(data: *const ()) {
        let task = Arc::<Task>::from_raw(data as *const Task);
        task.tx.send(Arc::clone(&task)).unwrap();
    }

    unsafe fn wake_by_ref(data: *const ()) {
        let task = Arc::<Task>::from_raw(data as *const Task);
        task.tx.send(Arc::clone(&task)).unwrap();
        std::mem::forget(task);
    }
    unsafe fn drop(data: *const ()) {
        let _ = Arc::<Task>::from_raw(data as *const Task);
    }

    let raw_waker = RawWaker::new(data, &VTABLE);
    unsafe { Waker::from_raw(raw_waker) }
}

struct MiniRuntime<T = Arc<Task>> {
    tx: Sender<T>,
    rx: Receiver<T>,
}

impl MiniRuntime {
    fn new() -> Self {
        let (tx, rx) = mpsc::channel();
        Self { tx, rx }
    }

    fn spawn(&self, f: impl Future<Output = ()> + Send + 'static) {
        let task = into_task(f, Sender::clone(&self.tx));
        let arc_task = Arc::new(task);
        self.tx.send(arc_task).unwrap();
    }

    fn run_once(&self) {
        while let Ok(t) = self.rx.recv() {
            let mut guard = t.future.lock().unwrap();
            let waker = into_waker(Arc::clone(&t));
            let mut cx = Context::from_waker(&waker);
            match guard.as_mut().poll(&mut cx) {
                Poll::Ready(_) => return,
                Poll::Pending => (),
            }
        }
    }
}

fn block_on<T: Future + Send + 'static>(f: T) -> T::Output
where
    T::Output: Send,
{
    let (tx, rx) = mpsc::channel();
    let wrapped = async move {
        let out = f.await;
        tx.send(out).unwrap();
    };
    let runtime = MiniRuntime::new();
    runtime.spawn(wrapped);
    runtime.run_once();
    rx.recv().unwrap()
}

struct Simple;
impl Future for Simple {
    type Output = String;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Ready(String::from("hello"))
    }
}

struct Counter {
    count: i32,
}
impl Future for Counter {
    type Output = i32;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.count < 3 {
            self.get_mut().count += 1;
            cx.waker().wake_by_ref();
            Poll::Pending
        } else {
            Poll::Ready(self.count)
        }
    }
}

#[test]
fn test_runtime() {
    let s = Simple;
    println!("Simple Future:{}", block_on(s));
    let counter = Counter { count: 0 };
    println!("Counter Future:{}", block_on(counter));
}
