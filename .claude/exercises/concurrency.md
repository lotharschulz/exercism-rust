# Concurrency Exercise Patterns

## Core Principles for Concurrency Exercises

### Thread Safety
- Use `Arc` for shared ownership across threads
- Use `Mutex` or `RwLock` for shared mutable state
- Prefer message passing over shared state when possible
- Use `Send` and `Sync` traits appropriately

### Synchronization
- Use channels for communication between threads
- Consider `atomic` types for simple shared values
- Use `Barrier` for thread synchronization points
- Apply `Condvar` for conditional waiting

## Basic Thread Patterns

### Thread Creation and Joining
```rust
use std::thread;
use std::time::Duration;

pub fn basic_threading() {
    // Simple thread spawn
    let handle = thread::spawn(|| {
        println!("Hello from thread!");
        42
    });
    
    // Get result from thread
    let result = handle.join().unwrap();
    assert_eq!(result, 42);
    
    // Multiple threads
    let mut handles = vec![];
    
    for i in 0..10 {
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(100));
            i * 2
        });
        handles.push(handle);
    }
    
    let results: Vec<i32> = handles
        .into_iter()
        .map(|h| h.join().unwrap())
        .collect();
}

// Thread with shared state
use std::sync::{Arc, Mutex};

pub fn shared_counter() -> i32 {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    *counter.lock().unwrap()
}
```

### Channel Communication
```rust
use std::sync::mpsc;
use std::thread;

pub fn channel_example() {
    // Simple channel
    let (tx, rx) = mpsc::channel();
    
    thread::spawn(move || {
        tx.send("Hello from thread").unwrap();
    });
    
    let msg = rx.recv().unwrap();
    println!("Received: {}", msg);
}

// Multiple producers
pub fn multiple_producers() {
    let (tx, rx) = mpsc::channel();
    let mut handles = vec![];
    
    for i in 0..10 {
        let tx = tx.clone();
        let handle = thread::spawn(move || {
            tx.send(i).unwrap();
        });
        handles.push(handle);
    }
    
    // Drop original sender
    drop(tx);
    
    // Receive all messages
    let mut results = vec![];
    for received in rx {
        results.push(received);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
}

// Bidirectional communication
pub fn bidirectional_channels() {
    let (tx1, rx1) = mpsc::channel();
    let (tx2, rx2) = mpsc::channel();
    
    thread::spawn(move || {
        let msg = rx1.recv().unwrap();
        println!("Thread received: {}", msg);
        tx2.send("Response from thread").unwrap();
    });
    
    tx1.send("Hello thread").unwrap();
    let response = rx2.recv().unwrap();
    println!("Main received: {}", response);
}
```

### Atomic Operations
```rust
use std::sync::atomic::{AtomicUsize, AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;

pub struct AtomicCounter {
    count: Arc<AtomicUsize>,
}

impl AtomicCounter {
    pub fn new() -> Self {
        AtomicCounter {
            count: Arc::new(AtomicUsize::new(0)),
        }
    }
    
    pub fn increment(&self) {
        self.count.fetch_add(1, Ordering::SeqCst);
    }
    
    pub fn get(&self) -> usize {
        self.count.load(Ordering::SeqCst)
    }
}

// Spinlock using atomics
pub struct SpinLock {
    locked: AtomicBool,
}

impl SpinLock {
    pub fn new() -> Self {
        SpinLock {
            locked: AtomicBool::new(false),
        }
    }
    
    pub fn lock(&self) {
        while self.locked.compare_exchange(
            false,
            true,
            Ordering::Acquire,
            Ordering::Relaxed,
        ).is_err() {
            // Spin
            std::hint::spin_loop();
        }
    }
    
    pub fn unlock(&self) {
        self.locked.store(false, Ordering::Release);
    }
}
```

### Read-Write Locks
```rust
use std::sync::{Arc, RwLock};
use std::thread;

pub struct SharedData {
    data: Arc<RwLock<Vec<i32>>>,
}

impl SharedData {
    pub fn new() -> Self {
        SharedData {
            data: Arc::new(RwLock::new(Vec::new())),
        }
    }
    
    pub fn read(&self) -> Vec<i32> {
        let data = self.data.read().unwrap();
        data.clone()
    }
    
    pub fn write(&self, value: i32) {
        let mut data = self.data.write().unwrap();
        data.push(value);
    }
    
    pub fn concurrent_reads(&self) {
        let mut handles = vec![];
        
        // Multiple readers can access simultaneously
        for i in 0..10 {
            let data = Arc::clone(&self.data);
            let handle = thread::spawn(move || {
                let data = data.read().unwrap();
                println!("Thread {} read {} items", i, data.len());
            });
            handles.push(handle);
        }
        
        for handle in handles {
            handle.join().unwrap();
        }
    }
}
```

## Advanced Concurrency Patterns

### Thread Pool
```rust
use std::sync::{Arc, Mutex, mpsc};
use std::thread;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        
        let mut workers = Vec::with_capacity(size);
        
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        
        ThreadPool { workers, sender }
    }
    
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }
        
        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();
            
            match message {
                Message::NewJob(job) => {
                    println!("Worker {} got a job", id);
                    job();
                }
                Message::Terminate => {
                    println!("Worker {} terminating", id);
                    break;
                }
            }
        });
        
        Worker {
            id,
            thread: Some(thread),
        }
    }
}
```

### Barrier Synchronization
```rust
use std::sync::{Arc, Barrier};
use std::thread;

pub fn barrier_example() {
    let n = 10;
    let mut handles = vec![];
    let barrier = Arc::new(Barrier::new(n));
    
    for i in 0..n {
        let b = Arc::clone(&barrier);
        let handle = thread::spawn(move || {
            println!("Thread {} before barrier", i);
            
            b.wait();
            
            println!("Thread {} after barrier", i);
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
}
```

### Condition Variables
```rust
use std::sync::{Arc, Mutex, Condvar};
use std::thread;

pub struct Queue {
    items: Arc<(Mutex<Vec<i32>>, Condvar)>,
}

impl Queue {
    pub fn new() -> Self {
        Queue {
            items: Arc::new((Mutex::new(Vec::new()), Condvar::new())),
        }
    }
    
    pub fn producer(&self) {
        let (lock, cvar) = &*self.items;
        
        for i in 0..10 {
            thread::sleep(std::time::Duration::from_millis(100));
            
            let mut items = lock.lock().unwrap();
            items.push(i);
            println!("Produced: {}", i);
            
            cvar.notify_one();
        }
    }
    
    pub fn consumer(&self) {
        let (lock, cvar) = &*self.items;
        
        loop {
            let mut items = lock.lock().unwrap();
            
            while items.is_empty() {
                items = cvar.wait(items).unwrap();
            }
            
            if let Some(item) = items.pop() {
                println!("Consumed: {}", item);
                
                if item == 9 {
                    break;
                }
            }
        }
    }
}
```

## Exercise-Specific Templates

### parallel-letter-frequency
```rust
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    if input.is_empty() || worker_count == 0 {
        return HashMap::new();
    }
    
    let counter = Arc::new(Mutex::new(HashMap::new()));
    let chunks = input.chunks(input.len().max(1) / worker_count.max(1) + 1);
    let mut handles = vec![];
    
    for chunk in chunks {
        let counter = Arc::clone(&counter);
        let chunk = chunk.to_vec();
        
        let handle = thread::spawn(move || {
            let mut local_count = HashMap::new();
            
            for text in chunk {
                for ch in text.chars() {
                    if ch.is_alphabetic() {
                        let ch = ch.to_lowercase().next().unwrap();
                        *local_count.entry(ch).or_insert(0) += 1;
                    }
                }
            }
            
            let mut global = counter.lock().unwrap();
            for (ch, count) in local_count {
                *global.entry(ch).or_insert(0) += count;
            }
        });
        
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    Arc::try_unwrap(counter)
        .unwrap()
        .into_inner()
        .unwrap()
}

// Alternative: Using channels
pub fn frequency_with_channels(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    use std::sync::mpsc;
    
    let (tx, rx) = mpsc::channel();
    let chunks = input.chunks(input.len().max(1) / worker_count.max(1) + 1);
    
    for chunk in chunks {
        let tx = tx.clone();
        let chunk = chunk.to_vec();
        
        thread::spawn(move || {
            let mut local_count = HashMap::new();
            
            for text in chunk {
                for ch in text.chars().filter(|c| c.is_alphabetic()) {
                    let ch = ch.to_lowercase().next().unwrap();
                    *local_count.entry(ch).or_insert(0) += 1;
                }
            }
            
            tx.send(local_count).unwrap();
        });
    }
    
    drop(tx);
    
    let mut result = HashMap::new();
    for local_count in rx {
        for (ch, count) in local_count {
            *result.entry(ch).or_insert(0) += count;
        }
    }
    
    result
}
```

### bank-account
```rust
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct BankAccount {
    balance: Arc<Mutex<Option<i32>>>,
}

impl BankAccount {
    pub fn new() -> BankAccount {
        BankAccount {
            balance: Arc::new(Mutex::new(None)),
        }
    }
    
    pub fn open(&mut self) {
        let mut balance = self.balance.lock().unwrap();
        *balance = Some(0);
    }
    
    pub fn close(&mut self) {
        let mut balance = self.balance.lock().unwrap();
        *balance = None;
    }
    
    pub fn balance(&self) -> Option<i32> {
        let balance = self.balance.lock().unwrap();
        *balance
    }
    
    pub fn deposit(&mut self, amount: i32) -> Option<i32> {
        if amount < 0 {
            return None;
        }
        
        let mut balance = self.balance.lock().unwrap();
        
        balance.as_mut().map(|b| {
            *b += amount;
            *b
        })
    }
    
    pub fn withdraw(&mut self, amount: i32) -> Option<i32> {
        if amount < 0 {
            return None;
        }
        
        let mut balance = self.balance.lock().unwrap();
        
        balance.as_mut().and_then(|b| {
            if *b >= amount {
                *b -= amount;
                Some(*b)
            } else {
                None
            }
        })
    }
}

// Thread-safe operations test
#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    
    #[test]
    fn test_concurrent_deposits() {
        let mut account = BankAccount::new();
        account.open();
        
        let mut handles = vec![];
        
        for _ in 0..10 {
            let mut acc = account.clone();
            let handle = thread::spawn(move || {
                acc.deposit(10);
            });
            handles.push(handle);
        }
        
        for handle in handles {
            handle.join().unwrap();
        }
        
        assert_eq!(account.balance(), Some(100));
    }
}
```

## Deadlock Prevention

```rust
use std::sync::{Arc, Mutex};
use std::thread;

// Deadlock example (DON'T DO THIS)
pub fn deadlock_example() {
    let resource1 = Arc::new(Mutex::new(0));
    let resource2 = Arc::new(Mutex::new(0));
    
    let r1 = Arc::clone(&resource1);
    let r2 = Arc::clone(&resource2);
    
    let handle1 = thread::spawn(move || {
        let _lock1 = r1.lock().unwrap();
        thread::sleep(std::time::Duration::from_millis(10));
        let _lock2 = r2.lock().unwrap(); // Deadlock!
    });
    
    let r1 = Arc::clone(&resource1);
    let r2 = Arc::clone(&resource2);
    
    let handle2 = thread::spawn(move || {
        let _lock2 = r2.lock().unwrap();
        thread::sleep(std::time::Duration::from_millis(10));
        let _lock1 = r1.lock().unwrap(); // Deadlock!
    });
}

// Solution: Always acquire locks in the same order
pub fn no_deadlock_example() {
    let resource1 = Arc::new(Mutex::new(0));
    let resource2 = Arc::new(Mutex::new(0));
    
    // Always lock resource1 first, then resource2
    let r1 = Arc::clone(&resource1);
    let r2 = Arc::clone(&resource2);
    
    let handle1 = thread::spawn(move || {
        let _lock1 = r1.lock().unwrap();
        let _lock2 = r2.lock().unwrap();
        // Do work
    });
    
    let r1 = Arc::clone(&resource1);
    let r2 = Arc::clone(&resource2);
    
    let handle2 = thread::spawn(move || {
        let _lock1 = r1.lock().unwrap(); // Same order!
        let _lock2 = r2.lock().unwrap();
        // Do work
    });
    
    handle1.join().unwrap();
    handle2.join().unwrap();
}
```

## Performance Patterns

### Lock-Free Data Structures
```rust
use std::sync::atomic::{AtomicPtr, Ordering};
use std::ptr;

pub struct Stack<T> {
    head: AtomicPtr<Node<T>>,
}

struct Node<T> {
    data: T,
    next: *mut Node<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack {
            head: AtomicPtr::new(ptr::null_mut()),
        }
    }
    
    pub fn push(&self, data: T) {
        let new_node = Box::into_raw(Box::new(Node {
            data,
            next: ptr::null_mut(),
        }));
        
        loop {
            let head = self.head.load(Ordering::Relaxed);
            unsafe {
                (*new_node).next = head;
            }
            
            if self.head.compare_exchange(
                head,
                new_node,
                Ordering::Release,
                Ordering::Relaxed,
            ).is_ok() {
                break;
            }
        }
    }
    
    pub fn pop(&self) -> Option<T> {
        loop {
            let head = self.head.load(Ordering::Acquire);
            
            if head.is_null() {
                return None;
            }
            
            let next = unsafe { (*head).next };
            
            if self.head.compare_exchange(
                head,
                next,
                Ordering::Release,
                Ordering::Relaxed,
            ).is_ok() {
                let node = unsafe { Box::from_raw(head) };
                return Some(node.data);
            }
        }
    }
}
```

## Testing Concurrent Code

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_thread_safety() {
        let data = Arc::new(Mutex::new(Vec::new()));
        let counter = Arc::new(AtomicUsize::new(0));
        let mut handles = vec![];
        
        for i in 0..100 {
            let data = Arc::clone(&data);
            let counter = Arc::clone(&counter);
            
            let handle = thread::spawn(move || {
                let mut data = data.lock().unwrap();
                data.push(i);
                counter.fetch_add(1, Ordering::SeqCst);
            });
            
            handles.push(handle);
        }
        
        for handle in handles {
            handle.join().unwrap();
        }
        
        assert_eq!(counter.load(Ordering::SeqCst), 100);
        assert_eq!(data.lock().unwrap().len(), 100);
    }

    #[test]
    fn test_no_data_races() {
        // Use thread sanitizer: RUSTFLAGS="-Z sanitizer=thread" cargo test
        let shared = Arc::new(AtomicUsize::new(0));
        let mut handles = vec![];
        
        for _ in 0..10 {
            let shared = Arc::clone(&shared);
            handles.push(thread::spawn(move || {
                for _ in 0..1000 {
                    shared.fetch_add(1, Ordering::Relaxed);
                }
            }));
        }
        
        for handle in handles {
            handle.join().unwrap();
        }
        
        assert_eq!(shared.load(Ordering::SeqCst), 10000);
    }

    #[test]
    fn test_timeout() {
        let (tx, rx) = mpsc::channel();
        
        thread::spawn(move || {
            thread::sleep(Duration::from_secs(2));
            tx.send(42).unwrap();
        });
        
        // Use timeout to avoid hanging tests
        match rx.recv_timeout(Duration::from_secs(1)) {
            Ok(_) => panic!("Should timeout"),
            Err(mpsc::RecvTimeoutError::Timeout) => {},
            Err(e) => panic!("Unexpected error: {:?}", e),
        }
    }
}
```

## Common Pitfalls and Solutions

1. **Data races**: Use proper synchronization (Mutex, RwLock)
2. **Deadlocks**: Acquire locks in consistent order
3. **Panic in thread**: Use `catch_unwind` or handle Results properly
4. **Resource leaks**: Ensure proper cleanup in Drop
5. **Starvation**: Use fair locks or careful scheduling

## Best Practices

1. **Prefer message passing** over shared state
2. **Use `crossbeam` crate** for advanced concurrent data structures
3. **Keep critical sections small** to reduce contention
4. **Use `rayon` for data parallelism**
5. **Profile with tools** like `perf` or `flamegraph`
6. **Test with `--test-threads=1`** for deterministic behavior
7. **Use thread sanitizer** to detect races

## Exercise Checklist

- [ ] Identify shared state requirements
- [ ] Choose appropriate synchronization primitives
- [ ] Implement Send and Sync correctly
- [ ] Handle thread panics gracefully
- [ ] Test with multiple threads
- [ ] Check for deadlocks and races
- [ ] Benchmark parallel vs sequential
- [ ] Document thread safety guarantees
- [ ] Use atomic operations where appropriate
- [ ] Consider using established crates (tokio, rayon)