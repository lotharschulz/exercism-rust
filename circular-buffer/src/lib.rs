/// A circular buffer implementation using a fixed-size vector.
/// Elements are stored in a Vec<Option<T>>, with head and tail indices tracking read and write positions.
pub struct CircularBuffer<T> {
    /// The underlying buffer storage. Uses Option<T> to represent empty slots.
    buffer: Vec<Option<T>>,
    /// The maximum number of elements the buffer can hold.
    capacity: usize,
    /// Index of the oldest element (read position).
    head: usize,
    /// Index where the next element will be written (write position).
    tail: usize,
    /// Current number of elements in the buffer.
    count: usize,
}

#[derive(Debug, PartialEq, Eq)]
/// Errors that can occur when operating on the circular buffer.
pub enum Error {
    /// Attempted to read from an empty buffer.
    EmptyBuffer,
    /// Attempted to write to a full buffer without overwriting.
    FullBuffer,
}

impl<T> CircularBuffer<T> {
    /// Creates a new CircularBuffer with the given capacity.
    pub fn new(capacity: usize) -> Self {
        let mut buffer = Vec::with_capacity(capacity);
        // Initialize all slots as None (empty)
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
        // Place the element at the tail position
        self.buffer[self.tail] = Some(element);
        // Move tail forward, wrapping around if necessary
        self.tail = (self.tail + 1) % self.capacity;
        self.count += 1;
        Ok(())
    }

    /// Reads the oldest element from the buffer. Returns EmptyBuffer error if the buffer is empty.
    pub fn read(&mut self) -> Result<T, Error> {
        if self.count == 0 {
            return Err(Error::EmptyBuffer);
        }
        // Take the element at head, leaving None in its place
        let element = self.buffer[self.head].take().unwrap();
        // Move head forward, wrapping around if necessary
        self.head = (self.head + 1) % self.capacity;
        self.count -= 1;
        Ok(element)
    }

    /// Clears the buffer, dropping all elements.
    pub fn clear(&mut self) {
        // Set all slots to None to drop the elements
        for item in &mut self.buffer {
            *item = None;
        }
        // Reset indices and count
        self.head = 0;
        self.tail = 0;
        self.count = 0;
    }

    /// Writes an element to the buffer, overwriting the oldest element if the buffer is full.
    pub fn overwrite(&mut self, element: T) {
        if self.count < self.capacity {
            // Buffer not full, perform normal write
            self.buffer[self.tail] = Some(element);
            self.tail = (self.tail + 1) % self.capacity;
            self.count += 1;
        } else {
            // Buffer full, overwrite the oldest element at head
            self.buffer[self.head] = Some(element);
            // Move both head and tail forward
            self.head = (self.head + 1) % self.capacity;
            self.tail = (self.tail + 1) % self.capacity;
            // Count remains the same
        }
    }
}
