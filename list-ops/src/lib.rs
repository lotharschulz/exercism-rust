/// Iterator that yields each item of `a` and then each item of `b`.
struct Append<I, J>
where
    I: Iterator,
    J: Iterator<Item = I::Item>,
{
    a: I,
    b: J,
    using_b: bool,
}

impl<I, J> Iterator for Append<I, J>
where
    I: Iterator,
    J: Iterator<Item = I::Item>,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.using_b {
            if let Some(item) = self.a.next() {
                return Some(item);
            }
            self.using_b = true;
        }
        self.b.next()
    }
}

/// Yields each item of a and then each item of b
pub fn append<I, J>(a: I, b: J) -> impl Iterator<Item = I::Item>
where
    I: Iterator,
    J: Iterator<Item = I::Item>,
{
    Append {
        a,
        b,
        using_b: false,
    }
}

/// Iterator that flattens an iterator of iterators.
struct Concat<I>
where
    I: Iterator,
    I::Item: Iterator,
{
    outer: I,
    current_inner: Option<I::Item>,
}

impl<I> Iterator for Concat<I>
where
    I: Iterator,
    I::Item: Iterator,
{
    type Item = <I::Item as Iterator>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(inner) = self.current_inner.as_mut() {
                if let Some(item) = inner.next() {
                    return Some(item);
                }
                self.current_inner = None;
            }

            match self.outer.next() {
                Some(next_inner) => {
                    self.current_inner = Some(next_inner);
                }
                None => return None,
            }
        }
    }
}

/// Combines all items in all nested iterators inside into one flattened iterator
pub fn concat<I>(nested_iter: I) -> impl Iterator<Item = <I::Item as Iterator>::Item>
where
    I: Iterator,
    I::Item: Iterator,
{
    Concat {
        outer: nested_iter,
        current_inner: None,
    }
}

/// Iterator that yields only items matching a predicate.
struct Filter<I, F>
where
    I: Iterator,
    F: Fn(&I::Item) -> bool,
{
    iter: I,
    predicate: F,
}

impl<I, F> Iterator for Filter<I, F>
where
    I: Iterator,
    F: Fn(&I::Item) -> bool,
{
    type Item = I::Item;

    #[allow(clippy::manual_find)]
    fn next(&mut self) -> Option<Self::Item> {
        for item in self.iter.by_ref() {
            if (self.predicate)(&item) {
                return Some(item);
            }
        }
        None
    }
}

/// Returns an iterator of all items in iter for which `predicate(item)` is true
pub fn filter<I, F>(iter: I, predicate: F) -> impl Iterator<Item = I::Item>
where
    I: Iterator,
    F: Fn(&I::Item) -> bool,
{
    Filter { iter, predicate }
}

pub fn length<I: Iterator>(iter: I) -> usize {
    let mut count = 0usize;
    for _item in iter {
        count += 1;
    }
    count
}

/// Iterator that applies a function to each item.
struct Map<I, F> {
    iter: I,
    function: F,
}

impl<I, F, U> Iterator for Map<I, F>
where
    I: Iterator,
    F: Fn(I::Item) -> U,
{
    type Item = U;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|item| (self.function)(item))
    }
}

/// Returns an iterator of the results of applying `function(item)` on all iter items
pub fn map<I, F, U>(iter: I, function: F) -> impl Iterator<Item = U>
where
    I: Iterator,
    F: Fn(I::Item) -> U,
{
    Map { iter, function }
}

pub fn foldl<I, F, U>(iter: I, mut initial: U, function: F) -> U
where
    I: Iterator,
    F: Fn(U, I::Item) -> U,
{
    for item in iter {
        initial = function(initial, item);
    }
    initial
}

pub fn foldr<I, F, U>(mut iter: I, mut initial: U, function: F) -> U
where
    I: DoubleEndedIterator,
    F: Fn(U, I::Item) -> U,
{
    while let Some(item) = iter.next_back() {
        initial = function(initial, item);
    }
    initial
}

/// Iterator that yields items from the back of a double-ended iterator.
struct Reverse<I>
where
    I: DoubleEndedIterator,
{
    iter: I,
}

impl<I> Iterator for Reverse<I>
where
    I: DoubleEndedIterator,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next_back()
    }
}

/// Returns an iterator with all the original items, but in reverse order
pub fn reverse<I: DoubleEndedIterator>(iter: I) -> impl Iterator<Item = I::Item> {
    Reverse { iter }
}
