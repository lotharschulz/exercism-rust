# Collection Exercise Patterns

## Core Principles for Collection Exercises

### Iterator Philosophy
- Prefer iterator chains over manual loops
- Use lazy evaluation when possible
- Avoid collecting intermediate results unnecessarily
- Chain operations for efficiency

### Memory Management
- Pre-allocate with `Vec::with_capacity()` when size is known
- Use `&[T]` for read-only access instead of `&Vec<T>`
- Consider `SmallVec` for collections typically < 32 elements
- Return iterators when the caller might not need all elements

## Common Iterator Patterns

### Map Implementation
```rust
/// Generic map function (accumulate exercise)
pub fn map<T, F, U>(input: Vec<T>, mut function: F) -> Vec<U>
where
    F: FnMut(T) -> U,
{
    input.into_iter().map(function).collect()
}

/// With borrowing to avoid consuming input
pub fn map_borrow<T, F, U>(input: &[T], mut function: F) -> Vec<U>
where
    F: FnMut(&T) -> U,
{
    input.iter().map(function).collect()
}
```

### Filter Operations
```rust
/// Filter with predicate
pub fn filter_by<T, F>(items: Vec<T>, mut predicate: F) -> Vec<T>
where
    F: FnMut(&T) -> bool,
{
    items.into_iter()
        .filter(|item| predicate(item))
        .collect()
}

/// Filter map combination
pub fn filter_map_example<T, U>(items: Vec<T>) -> Vec<U>
where
    T: TryInto<U>,
{
    items.into_iter()
        .filter_map(|item| item.try_into().ok())
        .collect()
}
```

### Fold/Reduce Patterns
```rust
/// Sum with fold
pub fn sum_with_initial<T>(items: &[T], initial: T) -> T
where
    T: std::ops::Add<Output = T> + Copy,
{
    items.iter().fold(initial, |acc, &item| acc + item)
}

/// Complex accumulation
pub fn accumulate_groups<T>(items: Vec<T>) -> Vec<Vec<T>>
where
    T: PartialEq,
{
    items.into_iter().fold(Vec::new(), |mut groups, item| {
        match groups.last_mut() {
            Some(group) if group.first() == Some(&item) => {
                group.push(item);
            }
            _ => {
                groups.push(vec![item]);
            }
        }
        groups
    })
}
```

### Flatten Operations
```rust
/// Flatten nested arrays (with Option handling)
pub fn flatten<T>(nested: Vec<Option<Vec<Option<T>>>>) -> Vec<T> {
    nested
        .into_iter()
        .filter_map(|outer| outer)
        .flat_map(|inner| inner.into_iter())
        .filter_map(|item| item)
        .collect()
}

/// Recursive flattening for arbitrary nesting
#[derive(Debug, Clone, PartialEq)]
pub enum NestedList<T> {
    Elem(T),
    List(Vec<NestedList<T>>),
}

pub fn flatten_recursive<T>(list: &NestedList<T>) -> Vec<T>
where
    T: Clone,
{
    match list {
        NestedList::Elem(val) => vec![val.clone()],
        NestedList::List(items) => {
            items.iter()
                .flat_map(|item| flatten_recursive(item))
                .collect()
        }
    }
}
```

### Grouping and Partitioning
```rust
use std::collections::HashMap;

/// Group items by key
pub fn group_by<T, K, F>(items: Vec<T>, mut key_fn: F) -> HashMap<K, Vec<T>>
where
    K: Eq + std::hash::Hash,
    F: FnMut(&T) -> K,
{
    let mut groups = HashMap::new();
    for item in items {
        let key = key_fn(&item);
        groups.entry(key).or_insert_with(Vec::new).push(item);
    }
    groups
}

/// Partition into two collections
pub fn partition_even_odd(numbers: Vec<i32>) -> (Vec<i32>, Vec<i32>) {
    numbers.into_iter()
        .partition(|&n| n % 2 == 0)
}
```

### Zip and Combination Patterns
```rust
/// Zip with index
pub fn enumerate_items<T>(items: Vec<T>) -> Vec<(usize, T)> {
    items.into_iter().enumerate().collect()
}

/// Combine multiple collections
pub fn zip_three<A, B, C>(a: Vec<A>, b: Vec<B>, c: Vec<C>) -> Vec<(A, B, C)> {
    a.into_iter()
        .zip(b.into_iter())
        .zip(c.into_iter())
        .map(|((a, b), c)| (a, b, c))
        .collect()
}

/// Cartesian product
pub fn cartesian_product<T: Clone>(a: &[T], b: &[T]) -> Vec<(T, T)> {
    a.iter()
        .flat_map(|x| b.iter().map(move |y| (x.clone(), y.clone())))
        .collect()
}
```

## Exercise-Specific Templates

### accumulate
```rust
pub fn map<T, F, U>(input: Vec<T>, mut function: F) -> Vec<U>
where
    F: FnMut(T) -> U,
{
    // Simplest solution
    input.into_iter().map(function).collect()
    
    // Alternative with capacity optimization
    // let mut result = Vec::with_capacity(input.len());
    // for item in input {
    //     result.push(function(item));
    // }
    // result
}
```

### sublist
```rust
#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(a: &[T], b: &[T]) -> Comparison {
    use Comparison::*;
    
    match (a.len(), b.len()) {
        (0, 0) => Equal,
        (0, _) => Sublist,
        (_, 0) => Superlist,
        (a_len, b_len) if a_len > b_len => {
            if a.windows(b_len).any(|window| window == b) {
                Superlist
            } else {
                Unequal
            }
        }
        (a_len, b_len) if a_len < b_len => {
            if b.windows(a_len).any(|window| window == a) {
                Sublist
            } else {
                Unequal
            }
        }
        _ => {
            if a == b { Equal } else { Unequal }
        }
    }
}
```

### grade-school
```rust
use std::collections::{HashMap, BTreeMap};

#[derive(Default)]
pub struct School {
    grades: HashMap<u32, Vec<String>>,
}

impl School {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        self.grades
            .entry(grade)
            .or_insert_with(Vec::new)
            .push(student.to_string());
        
        // Keep sorted
        if let Some(students) = self.grades.get_mut(&grade) {
            students.sort();
        }
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut grades: Vec<_> = self.grades.keys().copied().collect();
        grades.sort_unstable();
        grades
    }

    pub fn grade(&self, grade: u32) -> Vec<String> {
        self.grades
            .get(&grade)
            .cloned()
            .unwrap_or_default()
    }
}
```

### flatten-array
```rust
pub fn flatten<T>(list: Vec<Option<Box<dyn NestedItem<T>>>>) -> Vec<T> {
    list.into_iter()
        .filter_map(|item| item)
        .flat_map(|item| item.flatten())
        .collect()
}

// Or with explicit enum
#[derive(Clone, Debug, PartialEq)]
pub enum FlattenItem<T> {
    Item(T),
    List(Vec<FlattenItem<T>>),
    Null,
}

impl<T: Clone> FlattenItem<T> {
    pub fn flatten(&self) -> Vec<T> {
        match self {
            FlattenItem::Item(val) => vec![val.clone()],
            FlattenItem::List(items) => {
                items.iter()
                    .flat_map(|item| item.flatten())
                    .collect()
            }
            FlattenItem::Null => vec![],
        }
    }
}
```

## Collection Type Selection

### When to Use Each Type
```rust
// Vec<T> - Dynamic arrays, sequential access
let mut items: Vec<i32> = Vec::new();
items.push(42);

// VecDeque<T> - Double-ended queue operations
use std::collections::VecDeque;
let mut deque: VecDeque<i32> = VecDeque::new();
deque.push_front(1);
deque.push_back(2);

// HashMap<K, V> - Fast lookups by key
use std::collections::HashMap;
let mut map: HashMap<String, i32> = HashMap::new();
map.insert("key".to_string(), 42);

// BTreeMap<K, V> - Sorted keys, range queries
use std::collections::BTreeMap;
let mut btree: BTreeMap<i32, String> = BTreeMap::new();
btree.insert(1, "one".to_string());

// HashSet<T> - Unique values, fast membership testing
use std::collections::HashSet;
let mut set: HashSet<i32> = HashSet::new();
set.insert(42);

// BinaryHeap<T> - Priority queue
use std::collections::BinaryHeap;
let mut heap: BinaryHeap<i32> = BinaryHeap::new();
heap.push(42);
```

## Performance Optimizations

### Pre-allocation
```rust
// Know the size? Pre-allocate!
pub fn transform_all<T, U, F>(items: &[T], f: F) -> Vec<U>
where
    F: Fn(&T) -> U,
{
    let mut result = Vec::with_capacity(items.len());
    for item in items {
        result.push(f(item));
    }
    result
}
```

### Avoid Cloning
```rust
// Bad: Unnecessary cloning
pub fn process_bad<T: Clone>(items: Vec<T>) -> Vec<T> {
    items.iter().cloned().filter(|_| true).collect()
}

// Good: Move or borrow
pub fn process_good<T>(items: Vec<T>) -> Vec<T> {
    items.into_iter().filter(|_| true).collect()
}

// Better: Return iterator
pub fn process_better<T>(items: Vec<T>) -> impl Iterator<Item = T> {
    items.into_iter().filter(|_| true)
}
```

### Batch Operations
```rust
// Process in chunks for cache efficiency
pub fn sum_chunks(numbers: &[i32]) -> i32 {
    numbers
        .chunks(1024)  // Process in cache-friendly chunks
        .map(|chunk| chunk.iter().sum::<i32>())
        .sum()
}
```

## Common Pitfalls and Solutions

### Iterator Consumption
```rust
// Wrong: Can't use iterator twice
let iter = vec![1, 2, 3].into_iter();
let sum: i32 = iter.sum();  // Consumes iterator
// let count = iter.count();  // Error! Iterator already consumed

// Right: Clone or collect first
let vec = vec![1, 2, 3];
let sum: i32 = vec.iter().sum();
let count = vec.len();  // Vec still available
```

### Lifetime Issues
```rust
// Problem: Returning iterator with lifetime issues
// fn get_evens(nums: &Vec<i32>) -> impl Iterator<Item = &i32> {
//     nums.iter().filter(|&&n| n % 2 == 0)  // Lifetime issue
// }

// Solution: Specify lifetime
fn get_evens<'a>(nums: &'a [i32]) -> impl Iterator<Item = &'a i32> {
    nums.iter().filter(|&&n| n % 2 == 0)
}
```

## Testing Collection Functions

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_collection() {
        let empty: Vec<i32> = vec![];
        assert_eq!(function(empty), expected);
    }

    #[test]
    fn test_single_element() {
        assert_eq!(function(vec![42]), expected);
    }

    #[test]
    fn test_duplicates() {
        assert_eq!(function(vec![1, 1, 2, 2, 3, 3]), expected);
    }

    #[test]
    fn test_large_collection() {
        let large: Vec<_> = (0..10_000).collect();
        // Test performance doesn't degrade
        let start = std::time::Instant::now();
        let result = function(large);
        assert!(start.elapsed().as_millis() < 100);
    }
    
    #[test]
    fn test_iterator_laziness() {
        let mut calls = 0;
        let iter = (0..1000).inspect(|_| calls += 1);
        let result: Vec<_> = iter.take(5).collect();
        assert_eq!(calls, 5, "Iterator should be lazy");
    }
}
```

## Exercise Checklist

When implementing collection exercises:
- [ ] Use appropriate collection type (Vec, HashMap, etc.)
- [ ] Prefer iterators over loops
- [ ] Pre-allocate when size is known
- [ ] Avoid unnecessary cloning
- [ ] Handle empty collections
- [ ] Test with large inputs
- [ ] Consider memory efficiency
- [ ] Use standard library methods when available
- [ ] Return iterators when appropriate
- [ ] Document complexity (O(n), O(n²), etc.)