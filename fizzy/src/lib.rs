use std::fmt::Display;
use std::ops::Rem;

/// A Matcher represents a single FizzBuzz rule: a predicate function and its substitution word.
/// When the predicate returns true for a given value, the substitution is included in the output.
pub struct Matcher<T> {
    // Boxed trait object to store the matcher function dynamically
    matcher: Box<dyn Fn(T) -> bool>,
    // The substitution string to use when the matcher returns true
    subs: String,
}

impl<T> Matcher<T> {
    /// Creates a new Matcher with a predicate function and substitution string.
    ///
    /// # Arguments
    /// * `matcher` - A function that tests if a value should trigger this substitution
    /// * `subs` - The string to substitute when the matcher returns true
    pub fn new<F, S>(matcher: F, subs: S) -> Matcher<T>
    where
        F: Fn(T) -> bool + 'static,
        S: Into<String>,
    {
        Matcher {
            // Box the function for dynamic dispatch
            matcher: Box::new(matcher),
            // Convert the substitution to a String
            subs: subs.into(),
        }
    }
}

/// A Fizzy is a collection of Matchers that can be applied to an iterator.
/// It processes each element, applying all matchers and concatenating their substitutions.
pub struct Fizzy<T> {
    // Vector of matchers to apply in order
    matchers: Vec<Matcher<T>>,
}

impl<T> Fizzy<T> {
    /// Creates a new empty Fizzy with no matchers.
    pub fn new() -> Self {
        Fizzy {
            matchers: Vec::new(),
        }
    }

    /// Adds a matcher to this Fizzy and returns the updated Fizzy (builder pattern).
    /// The matcher will be applied in the order it was added.
    #[must_use]
    pub fn add_matcher(mut self, matcher: Matcher<T>) -> Self {
        self.matchers.push(matcher);
        self
    }

    /// Applies all matchers to each element of an iterator, returning a new iterator of Strings.
    ///
    /// For each element:
    /// - All matching substitutions are concatenated in order
    /// - If no matchers match, the element is converted to a string using Display
    pub fn apply<I>(self, iter: I) -> impl Iterator<Item = String>
    where
        I: Iterator<Item = T>,
        T: Display + Copy,
    {
        // Move matchers into the closure to avoid lifetime issues
        let matchers = self.matchers;
        iter.map(move |item| {
            let mut result = String::new();
            // Apply each matcher in sequence
            for matcher in &matchers {
                if (matcher.matcher)(item) {
                    // Concatenate all matching substitutions
                    result.push_str(&matcher.subs);
                }
            }
            // If no matchers matched, return the number itself
            if result.is_empty() {
                item.to_string()
            } else {
                result
            }
        })
    }
}

impl<T> Default for Fizzy<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Convenience function that returns a Fizzy with standard FizzBuzz rules:
/// - "fizz" for multiples of 3
/// - "buzz" for multiples of 5
/// - "fizzbuzz" for multiples of both (15)
pub fn fizz_buzz<T>() -> Fizzy<T>
where
    T: Rem<Output = T> + From<u8> + PartialEq + Copy + Display + 'static,
{
    Fizzy::new()
        // Add fizz matcher first (divisible by 3)
        .add_matcher(Matcher::new(|n: T| n % 3.into() == 0.into(), "fizz"))
        // Add buzz matcher second (divisible by 5)
        .add_matcher(Matcher::new(|n: T| n % 5.into() == 0.into(), "buzz"))
}
