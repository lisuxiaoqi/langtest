use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Mutex, RwLock,
    },
    thread,
};

#[test]
fn test_thread() {
    let mut handles = vec![];
    let counter = Arc::new(Mutex::new(0));
    for _ in 0..10 {
        let cloned = Arc::clone(&counter);

        handles.push(thread::spawn(move || {
            let mut guard = cloned.lock().unwrap();
            *guard += 1;
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("result:{}", counter.lock().unwrap());
}

#[test]
fn test_atomic() {
    let mut handles = vec![];
    let counter = Arc::new(AtomicUsize::new(0));
    for _ in 0..10 {
        let cloned = Arc::clone(&counter);

        handles.push(thread::spawn(move || {
            cloned.fetch_add(1, Ordering::SeqCst);
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
    println!("result:{}", counter.load(Ordering::Relaxed));
}

#[test]
fn test_logger() {
    struct Logger(Mutex<Vec<String>>);

    impl Logger {
        fn log(&self, msg: String) {
            let mut guard = self.0.lock().unwrap();
            guard.push(msg);
        }

        fn dump(&self) {
            let guard = self.0.lock().unwrap();
            guard.iter().for_each(|i| {
                println!("{i}");
            });
        }
    }

    let logger = Arc::new(Logger(Mutex::new(Vec::new())));

    thread::scope(|s| {
        for _ in 0..5 {
            let clone = Arc::clone(&logger);
            s.spawn(move || {
                clone.log(format!("msg from:{:?}", thread::current().id()));
            });
        }
    });

    logger.dump();
}

#[test]
fn test_cache() {
    struct Cache {
        inner: RwLock<HashMap<String, String>>,
    }

    impl Cache {
        fn get(&self, key: &String) -> Option<String> {
            let guard = self.inner.read().unwrap();
            guard.get(key).cloned()
        }

        fn set(&self, key: String, value: String) {
            let mut guard = self.inner.write().unwrap();
            guard.insert(key, value);
        }
    }

    let cache = Arc::new(Cache {
        inner: RwLock::new(HashMap::new()),
    });

    thread::scope(|s| {
        for i in 0..5 {
            let clone = Arc::clone(&cache);
            s.spawn(move || {
                let key = format!("k{}", i);
                clone.set(key.clone(), format!("v{}", i));
                println!("set then get k:{}, v:{:?}", key, clone.get(&key));
            });
        }
    });

    if let Ok(v) = cache.inner.read() {
        v.iter().for_each(|(key, val)| {
            println!("k:{}, v:{}", key, val);
        });
    };
}
