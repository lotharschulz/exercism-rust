/// A circular buffer implementation using a fixed-size vector.
/// Elements are stored in a Vec<Option<T>>, with head and tail indices tracking read and write positions.
pub struct CircularBuffer<T> {
    buffer: Vec<Option<T>>,
    capacity: usize,
    head: usize,
    tail: usize,
    count: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T> CircularBuffer<T> {
    /// Creates a new CircularBuffer with the given capacity.
    pub fn new(capacity: usize) -> Self {
        let mut buffer = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            buffer.push(None);
        }
        CircularBuffer {
            buffer,
            capacity,
            head: 0,
            tail: 0,
            count: 0,
        }
    }

    /// Writes an element to the buffer. Returns FullBuffer error if the buffer is full.
    pub fn write(&mut self, element: T) -> Result<(), Error> {
        if self.count == self.capacity {
            return Err(Error::FullBuffer);
        }
        self.buffer[self.tail] = Some(element);
        self.tail = (self.tail + 1) % self.capacity;
        self.count += 1;
        Ok(())
    }

    /// Reads the oldest element from the buffer. Returns EmptyBuffer error if the buffer is empty.
    pub fn read(&mut self) -> Result<T, Error> {
        if self.count == 0 {
            return Err(Error::EmptyBuffer);
        }
        let element = self.buffer[self.head].take().unwrap();
        self.head = (self.head + 1) % self.capacity;
        self.count -= 1;
        Ok(element)
    }

    /// Clears the buffer, dropping all elements.
    pub fn clear(&mut self) {
        self.buffer.clear();
        for _ in 0..self.capacity {
            self.buffer.push(None);
        }
        self.head = 0;
        self.tail = 0;
        self.count = 0;
    }

    /// Writes an element to the buffer, overwriting the oldest element if the buffer is full.
    pub fn overwrite(&mut self, element: T) {
        if self.count < self.capacity {
            // Not full, just write
            self.buffer[self.tail] = Some(element);
            self.tail = (self.tail + 1) % self.capacity;
            self.count += 1;
        } else {
            // Full, overwrite oldest
            self.buffer[self.head] = Some(element);
            self.head = (self.head + 1) % self.capacity;
            self.tail = (self.tail + 1) % self.capacity;
        }
    }
}
