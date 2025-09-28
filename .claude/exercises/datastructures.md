# Data Structure Exercise Patterns

## Core Principles for Data Structure Exercises

### Ownership and Borrowing
- Design APIs that work well with Rust's ownership model
- Use `Rc` and `RefCell` for shared ownership when needed
- Consider using `Box` for recursive structures
- Use lifetime parameters appropriately

### Safety and Performance
- Prefer safe Rust over unsafe when possible
- Use `Option<T>` for nullable values
- Implement standard traits (`Debug`, `Clone`, `PartialEq`)
- Consider zero-cost abstractions

## Linked List Patterns

### Basic Singly Linked List
```rust
#[derive(Debug, Clone)]
pub struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

#[derive(Debug, Default)]
pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    len: usize,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn len(&self) -> usize {
        self.len
    }
    
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }
    
    pub fn push_front(&mut self, value: T) {
        let new_node = Box::new(Node {
            value,
            next: self.head.take(),
        });
        self.head = Some(new_node);
        self.len += 1;
    }
    
    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            self.len -= 1;
            node.value
        })
    }
    
    pub fn peek_front(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.value)
    }
}

// Iterator implementation
pub struct LinkedListIter<T> {
    current: Option<Box<Node<T>>>,
}

impl<T> Iterator for LinkedListIter<T> {
    type Item = T;
    
    fn next(&mut self) -> Option<Self::Item> {
        self.current.take().map(|node| {
            self.current = node.next;
            node.value
        })
    }
}

impl<T> IntoIterator for LinkedList<T> {
    type Item = T;
    type IntoIter = LinkedListIter<T>;
    
    fn into_iter(self) -> Self::IntoIter {
        LinkedListIter {
            current: self.head,
        }
    }
}
```

### Doubly Linked List with Rc/RefCell
```rust
use std::rc::{Rc, Weak};
use std::cell::RefCell;

pub struct Node<T> {
    value: T,
    next: Option<Rc<RefCell<Node<T>>>>,
    prev: Option<Weak<RefCell<Node<T>>>>,
}

pub struct DoublyLinkedList<T> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
    len: usize,
}

impl<T> DoublyLinkedList<T> {
    pub fn new() -> Self {
        DoublyLinkedList {
            head: None,
            tail: None,
            len: 0,
        }
    }
    
    pub fn push_front(&mut self, value: T) {
        let new_node = Rc::new(RefCell::new(Node {
            value,
            next: self.head.clone(),
            prev: None,
        }));
        
        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(Rc::downgrade(&new_node));
                new_node.borrow_mut().next = Some(old_head);
            }
            None => {
                self.tail = Some(new_node.clone());
            }
        }
        
        self.head = Some(new_node);
        self.len += 1;
    }
    
    pub fn push_back(&mut self, value: T) {
        let new_node = Rc::new(RefCell::new(Node {
            value,
            next: None,
            prev: self.tail.as_ref().map(|t| Rc::downgrade(t)),
        }));
        
        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_node.clone());
            }
            None => {
                self.head = Some(new_node.clone());
            }
        }
        
        self.tail = Some(new_node);
        self.len += 1;
    }
    
    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            match old_head.borrow_mut().next.take() {
                Some(new_head) => {
                    new_head.borrow_mut().prev = None;
                    self.head = Some(new_head);
                }
                None => {
                    self.tail = None;
                }
            }
            self.len -= 1;
            
            // Extract value from Rc<RefCell<Node<T>>>
            Rc::try_unwrap(old_head)
                .ok()
                .map(|cell| cell.into_inner().value)
                .unwrap()
        })
    }
}
```

## Binary Tree Patterns

### Basic Binary Tree
```rust
#[derive(Debug, Clone)]
pub struct BinaryTree<T> {
    value: T,
    left: Option<Box<BinaryTree<T>>>,
    right: Option<Box<BinaryTree<T>>>,
}

impl<T> BinaryTree<T> {
    pub fn new(value: T) -> Self {
        BinaryTree {
            value,
            left: None,
            right: None,
        }
    }
    
    pub fn insert_left(&mut self, value: T) {
        self.left = Some(Box::new(BinaryTree::new(value)));
    }
    
    pub fn insert_right(&mut self, value: T) {
        self.right = Some(Box::new(BinaryTree::new(value)));
    }
}

// Tree traversal
impl<T: Clone> BinaryTree<T> {
    pub fn inorder(&self) -> Vec<T> {
        let mut result = Vec::new();
        self.inorder_helper(&mut result);
        result
    }
    
    fn inorder_helper(&self, result: &mut Vec<T>) {
        if let Some(left) = &self.left {
            left.inorder_helper(result);
        }
        result.push(self.value.clone());
        if let Some(right) = &self.right {
            right.inorder_helper(result);
        }
    }
    
    pub fn preorder(&self) -> Vec<T> {
        let mut result = Vec::new();
        result.push(self.value.clone());
        if let Some(left) = &self.left {
            result.extend(left.preorder());
        }
        if let Some(right) = &self.right {
            result.extend(right.preorder());
        }
        result
    }
    
    pub fn height(&self) -> usize {
        let left_height = self.left.as_ref().map_or(0, |n| n.height());
        let right_height = self.right.as_ref().map_or(0, |n| n.height());
        1 + left_height.max(right_height)
    }
}
```

### Binary Search Tree
```rust
use std::cmp::Ordering;

#[derive(Debug)]
pub struct BST<T: Ord> {
    root: Option<Box<Node<T>>>,
}

#[derive(Debug)]
struct Node<T: Ord> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T: Ord> BST<T> {
    pub fn new() -> Self {
        BST { root: None }
    }
    
    pub fn insert(&mut self, value: T) {
        self.root = Self::insert_node(self.root.take(), value);
    }
    
    fn insert_node(node: Option<Box<Node<T>>>, value: T) -> Option<Box<Node<T>>> {
        match node {
            None => Some(Box::new(Node {
                value,
                left: None,
                right: None,
            })),
            Some(mut n) => {
                match value.cmp(&n.value) {
                    Ordering::Less => n.left = Self::insert_node(n.left.take(), value),
                    Ordering::Greater => n.right = Self::insert_node(n.right.take(), value),
                    Ordering::Equal => {} // No duplicates
                }
                Some(n)
            }
        }
    }
    
    pub fn contains(&self, value: &T) -> bool {
        Self::search_node(&self.root, value)
    }
    
    fn search_node(node: &Option<Box<Node<T>>>, value: &T) -> bool {
        match node {
            None => false,
            Some(n) => match value.cmp(&n.value) {
                Ordering::Equal => true,
                Ordering::Less => Self::search_node(&n.left, value),
                Ordering::Greater => Self::search_node(&n.right, value),
            }
        }
    }
    
    pub fn min(&self) -> Option<&T> {
        self.root.as_ref().map(|node| {
            let mut current = node;
            while let Some(left) = &current.left {
                current = left;
            }
            &current.value
        })
    }
}
```

## Stack and Queue Patterns

### Stack Implementation
```rust
#[derive(Debug, Default)]
pub struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn push(&mut self, item: T) {
        self.items.push(item);
    }
    
    pub fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }
    
    pub fn peek(&self) -> Option<&T> {
        self.items.last()
    }
    
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
    
    pub fn len(&self) -> usize {
        self.items.len()
    }
}

// With capacity management
impl<T> Stack<T> {
    pub fn with_capacity(capacity: usize) -> Self {
        Stack {
            items: Vec::with_capacity(capacity),
        }
    }
    
    pub fn clear(&mut self) {
        self.items.clear();
    }
}
```

### Queue Implementation
```rust
use std::collections::VecDeque;

#[derive(Debug, Default)]
pub struct Queue<T> {
    items: VecDeque<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn enqueue(&mut self, item: T) {
        self.items.push_back(item);
    }
    
    pub fn dequeue(&mut self) -> Option<T> {
        self.items.pop_front()
    }
    
    pub fn peek(&self) -> Option<&T> {
        self.items.front()
    }
    
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
    
    pub fn len(&self) -> usize {
        self.items.len()
    }
}
```

## Circular Buffer Pattern

```rust
pub struct CircularBuffer<T> {
    buffer: Vec<Option<T>>,
    capacity: usize,
    write_pos: usize,
    read_pos: usize,
    len: usize,
}

impl<T: Clone> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        CircularBuffer {
            buffer: vec![None; capacity],
            capacity,
            write_pos: 0,
            read_pos: 0,
            len: 0,
        }
    }
    
    pub fn write(&mut self, item: T) -> Result<(), Error> {
        if self.len == self.capacity {
            return Err(Error::FullBuffer);
        }
        
        self.buffer[self.write_pos] = Some(item);
        self.write_pos = (self.write_pos + 1) % self.capacity;
        self.len += 1;
        Ok(())
    }
    
    pub fn read(&mut self) -> Result<T, Error> {
        if self.len == 0 {
            return Err(Error::EmptyBuffer);
        }
        
        let item = self.buffer[self.read_pos]
            .take()
            .ok_or(Error::EmptyBuffer)?;
        self.read_pos = (self.read_pos + 1) % self.capacity;
        self.len -= 1;
        Ok(item)
    }
    
    pub fn overwrite(&mut self, item: T) {
        if self.len == self.capacity {
            // Overwrite oldest
            self.read_pos = (self.read_pos + 1) % self.capacity;
            self.len -= 1;
        }
        self.write(item).unwrap();
    }
    
    pub fn clear(&mut self) {
        self.buffer = vec![None; self.capacity];
        self.write_pos = 0;
        self.read_pos = 0;
        self.len = 0;
    }
}

#[derive(Debug, PartialEq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}
```

## Graph Patterns

### Adjacency List Graph
```rust
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug)]
pub struct Graph<T: Eq + std::hash::Hash + Clone> {
    adjacency_list: HashMap<T, Vec<T>>,
}

impl<T: Eq + std::hash::Hash + Clone> Graph<T> {
    pub fn new() -> Self {
        Graph {
            adjacency_list: HashMap::new(),
        }
    }
    
    pub fn add_vertex(&mut self, vertex: T) {
        self.adjacency_list.entry(vertex).or_insert(Vec::new());
    }
    
    pub fn add_edge(&mut self, from: T, to: T) {
        self.add_vertex(from.clone());
        self.add_vertex(to.clone());
        
        self.adjacency_list
            .get_mut(&from)
            .unwrap()
            .push(to.clone());
            
        // For undirected graph, add reverse edge
        // self.adjacency_list.get_mut(&to).unwrap().push(from);
    }
    
    pub fn dfs(&self, start: &T) -> Vec<T> {
        let mut visited = HashSet::new();
        let mut result = Vec::new();
        
        if self.adjacency_list.contains_key(start) {
            self.dfs_helper(start, &mut visited, &mut result);
        }
        
        result
    }
    
    fn dfs_helper(&self, vertex: &T, visited: &mut HashSet<T>, result: &mut Vec<T>) {
        if visited.contains(vertex) {
            return;
        }
        
        visited.insert(vertex.clone());
        result.push(vertex.clone());
        
        if let Some(neighbors) = self.adjacency_list.get(vertex) {
            for neighbor in neighbors {
                self.dfs_helper(neighbor, visited, result);
            }
        }
    }
    
    pub fn bfs(&self, start: &T) -> Vec<T> {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let mut result = Vec::new();
        
        if !self.adjacency_list.contains_key(start) {
            return result;
        }
        
        queue.push_back(start.clone());
        visited.insert(start.clone());
        
        while let Some(vertex) = queue.pop_front() {
            result.push(vertex.clone());
            
            if let Some(neighbors) = self.adjacency_list.get(&vertex) {
                for neighbor in neighbors {
                    if !visited.contains(neighbor) {
                        visited.insert(neighbor.clone());
                        queue.push_back(neighbor.clone());
                    }
                }
            }
        }
        
        result
    }
    
    pub fn has_cycle(&self) -> bool {
        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();
        
        for vertex in self.adjacency_list.keys() {
            if self.has_cycle_helper(vertex, &mut visited, &mut rec_stack) {
                return true;
            }
        }
        
        false
    }
    
    fn has_cycle_helper(
        &self,
        vertex: &T,
        visited: &mut HashSet<T>,
        rec_stack: &mut HashSet<T>,
    ) -> bool {
        visited.insert(vertex.clone());
        rec_stack.insert(vertex.clone());
        
        if let Some(neighbors) = self.adjacency_list.get(vertex) {
            for neighbor in neighbors {
                if !visited.contains(neighbor) {
                    if self.has_cycle_helper(neighbor, visited, rec_stack) {
                        return true;
                    }
                } else if rec_stack.contains(neighbor) {
                    return true;
                }
            }
        }
        
        rec_stack.remove(vertex);
        false
    }
}
```

## Exercise-Specific Templates

### linked-list
```rust
// See the doubly linked list implementation above
// Key points:
// - Use Rc<RefCell<>> for shared ownership
// - Use Weak references to prevent cycles
// - Implement push/pop for both ends
// - Handle edge cases (empty list, single element)
```

### binary-search
```rust
pub fn find<T: Ord>(array: &[T], key: T) -> Option<usize> {
    let mut left = 0;
    let mut right = array.len();
    
    while left < right {
        let mid = left + (right - left) / 2;
        
        match key.cmp(&array[mid]) {
            std::cmp::Ordering::Less => right = mid,
            std::cmp::Ordering::Greater => left = mid + 1,
            std::cmp::Ordering::Equal => return Some(mid),
        }
    }
    
    None
}

// Alternative: Recursive implementation
pub fn find_recursive<T: Ord>(array: &[T], key: T) -> Option<usize> {
    find_recursive_helper(array, key, 0, array.len())
}

fn find_recursive_helper<T: Ord>(
    array: &[T],
    key: T,
    left: usize,
    right: usize,
) -> Option<usize> {
    if left >= right {
        return None;
    }
    
    let mid = left + (right - left) / 2;
    
    match key.cmp(&array[mid]) {
        std::cmp::Ordering::Less => find_recursive_helper(array, key, left, mid),
        std::cmp::Ordering::Greater => find_recursive_helper(array, key, mid + 1, right),
        std::cmp::Ordering::Equal => Some(mid),
    }
}
```

### robot-simulator
```rust
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    x: i32,
    y: i32,
    direction: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, direction: Direction) -> Self {
        Robot { x, y, direction }
    }
    
    pub fn turn_right(self) -> Self {
        let direction = match self.direction {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        };
        Robot { direction, ..self }
    }
    
    pub fn turn_left(self) -> Self {
        let direction = match self.direction {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
        };
        Robot { direction, ..self }
    }
    
    pub fn advance(self) -> Self {
        let (x, y) = match self.direction {
            Direction::North => (self.x, self.y + 1),
            Direction::South => (self.x, self.y - 1),
            Direction::East => (self.x + 1, self.y),
            Direction::West => (self.x - 1, self.y),
        };
        Robot { x, y, ..self }
    }
    
    pub fn instructions(self, instructions: &str) -> Self {
        instructions.chars().fold(self, |robot, instruction| {
            match instruction {
                'R' => robot.turn_right(),
                'L' => robot.turn_left(),
                'A' => robot.advance(),
                _ => robot,
            }
        })
    }
    
    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }
    
    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}
```

## Testing Data Structures

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_structure() {
        let ds = DataStructure::new();
        assert!(ds.is_empty());
        assert_eq!(ds.len(), 0);
    }

    #[test]
    fn test_single_element() {
        let mut ds = DataStructure::new();
        ds.insert(42);
        assert_eq!(ds.len(), 1);
        assert!(!ds.is_empty());
    }

    #[test]
    fn test_many_elements() {
        let mut ds = DataStructure::new();
        for i in 0..1000 {
            ds.insert(i);
        }
        assert_eq!(ds.len(), 1000);
    }

    #[test]
    fn test_remove_from_empty() {
        let mut ds = DataStructure::new();
        assert_eq!(ds.remove(), None);
    }

    #[test]
    fn test_iterator() {
        let mut ds = DataStructure::new();
        let values = vec![1, 2, 3, 4, 5];
        
        for &v in &values {
            ds.insert(v);
        }
        
        let collected: Vec<_> = ds.into_iter().collect();
        assert_eq!(collected, values);
    }

    #[test]
    fn test_clone_and_eq() {
        let mut ds1 = DataStructure::new();
        ds1.insert(42);
        
        let ds2 = ds1.clone();
        assert_eq!(ds1, ds2);
    }
    
    #[test]
    fn test_thread_safety() {
        // If implementing thread-safe version
        use std::sync::Arc;
        use std::thread;
        
        let ds = Arc::new(DataStructure::new());
        let mut handles = vec![];
        
        for _ in 0..10 {
            let ds_clone = Arc::clone(&ds);
            let handle = thread::spawn(move || {
                // Thread operations
            });
            handles.push(handle);
        }
        
        for handle in handles {
            handle.join().unwrap();
        }
    }
}
```

## Common Pitfalls and Solutions

1. **Ownership cycles**: Use `Weak` references to break cycles
2. **Iterator invalidation**: Be careful when modifying during iteration
3. **Stack overflow**: Use iterative instead of recursive for deep structures
4. **Memory leaks**: Ensure proper cleanup in drop implementations
5. **Thread safety**: Use `Arc<Mutex<T>>` for shared mutable access

## Performance Tips

1. **Pre-allocate capacity** when size is known
2. **Use `SmallVec` for small collections**
3. **Consider cache locality** in data layout
4. **Use iterators** instead of index-based access
5. **Profile before optimizing** - measure first

## Exercise Checklist

- [ ] Implement all required methods
- [ ] Handle edge cases (empty, single element)
- [ ] Implement standard traits (Debug, Clone, PartialEq)
- [ ] Add iterator support where appropriate
- [ ] Test with large datasets
- [ ] Document complexity for each operation
- [ ] Consider thread safety requirements
- [ ] Avoid unnecessary allocations
- [ ] Prevent memory leaks and cycles
- [ ] Write comprehensive tests