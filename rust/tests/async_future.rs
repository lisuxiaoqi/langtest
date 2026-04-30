use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use std::time::Duration;

struct Ready;
impl Future for Ready {
    type Output = String;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Ready("hello".to_string())
    }
}

struct ReadyResult;
impl Future for ReadyResult {
    type Output = Result<i32, &'static str>;
    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Ready(Ok(42))
    }
}

#[test]
fn test_basic() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let ret = rt.block_on(Ready);
    println!("ret:{ret}");
}

struct Counter {
    count: u8,
}

impl Future for Counter {
    type Output = u8;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();
        if this.count < 3 {
            this.count += 1;
            cx.waker().wake_by_ref();
            cx.waker().wake_by_ref();
            return Poll::Pending;
        }
        Poll::Ready(this.count)
    }
}

#[test]
fn test_multi_poll() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let ret = rt.block_on(Counter { count: 1 });
    println!("ret:{ret}");
}

#[derive(Default)]
struct TimerFuture {
    shared: Arc<Mutex<SharedState>>,
}

#[derive(Default)]
struct SharedState {
    complete: bool,
    waker: Option<Waker>,
}
impl Future for TimerFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut guard = self.shared.lock().unwrap();
        if guard.complete {
            println!("Poll Ready");
            Poll::Ready(())
        } else {
            match &guard.waker {
                Some(w) if w.will_wake(cx.waker()) => {}
                _ => {
                    guard.waker = Some(cx.waker().clone());
                }
            }
            println!("Poll Pending");
            Poll::Pending
        }
    }
}
impl TimerFuture {
    fn new(d: Duration) -> TimerFuture {
        let t = TimerFuture {
            shared: Default::default(),
        };

        let s_clone = t.shared.clone();
        std::thread::spawn(move || {
            std::thread::sleep(d);
            let mut guard = s_clone.lock().unwrap();
            guard.complete = true;
            if let Some(w) = guard.waker.take() {
                println!("wake");
                w.wake();
            }
        });
        t
    }
}

#[test]
fn test_timer() {
    let f = TimerFuture::new(Duration::from_secs(1));
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(f);
    println!("timer future complete");
}
