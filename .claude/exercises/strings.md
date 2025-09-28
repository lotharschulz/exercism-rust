# String Exercise Patterns

## Common Requirements for String Exercises

### Unicode Handling
- Always use `.chars()` for character iteration (Unicode-aware)
- Use `.char_indices()` when you need both character and position
- Consider grapheme clusters for exercises involving visual characters
- Remember that `.len()` returns bytes, not character count

### Memory Efficiency
- Prefer `&str` parameters over `String` when ownership isn't needed
- Use `String::with_capacity()` when final size is known
- Consider `Cow<'_, str>` for conditional modifications
- Avoid unnecessary allocations in loops

## Common Patterns

### Basic String Reversal
```rust
/// Unicode-aware string reversal
pub fn reverse(input: &str) -> String {
    input.chars().rev().collect()
}

/// With grapheme clusters (for emoji support)
use unicode_segmentation::UnicodeSegmentation;

pub fn reverse_graphemes(input: &str) -> String {
    input.graphemes(true).rev().collect()
}
```

### Character Filtering
```rust
/// Filter alphabetic characters only
pub fn only_letters(input: &str) -> String {
    input.chars()
        .filter(|c| c.is_alphabetic())
        .collect()
}

/// Remove whitespace
pub fn remove_whitespace(input: &str) -> String {
    input.chars()
        .filter(|c| !c.is_whitespace())
        .collect()
}
```

### Case Transformations
```rust
/// Convert to acronym (uppercase first letters)
pub fn acronym(phrase: &str) -> String {
    phrase
        .split(|c: char| c.is_whitespace() || c == '-' || c == '_')
        .filter(|word| !word.is_empty())
        .flat_map(|word| {
            word.chars().take(1).chain(
                word.chars()
                    .skip_while(|c| c.is_uppercase())
                    .filter(|c| c.is_uppercase())
            )
        })
        .collect::<String>()
        .to_uppercase()
}
```

### String Building
```rust
/// Efficient string building with known capacity
pub fn build_string(parts: &[&str]) -> String {
    let capacity = parts.iter().map(|s| s.len()).sum();
    let mut result = String::with_capacity(capacity);
    
    for part in parts {
        result.push_str(part);
    }
    
    result
}

/// Using fold for string accumulation
pub fn join_with_separator(items: &[&str], separator: &str) -> String {
    items.iter()
        .fold(String::new(), |mut acc, item| {
            if !acc.is_empty() {
                acc.push_str(separator);
            }
            acc.push_str(item);
            acc
        })
}
```

### Pattern Matching in Strings
```rust
/// Check if string matches pattern
pub fn is_question(input: &str) -> bool {
    input.trim().ends_with('?')
}

/// Extract patterns
pub fn extract_numbers(input: &str) -> Vec<u32> {
    input
        .split(|c: char| !c.is_numeric())
        .filter(|s| !s.is_empty())
        .filter_map(|s| s.parse().ok())
        .collect()
}
```

### String Validation
```rust
/// Validate string format
pub fn is_valid_identifier(s: &str) -> bool {
    !s.is_empty()
        && s.chars()
            .all(|c| c.is_alphanumeric() || c == '_')
        && s.chars()
            .next()
            .map_or(false, |c| c.is_alphabetic() || c == '_')
}
```

### Substring Operations
```rust
/// Find all occurrences of substring
pub fn find_all(haystack: &str, needle: &str) -> Vec<usize> {
    haystack
        .match_indices(needle)
        .map(|(i, _)| i)
        .collect()
}

/// Replace with limit
pub fn replace_n(input: &str, from: &str, to: &str, n: usize) -> String {
    let mut result = String::with_capacity(input.len());
    let mut last_end = 0;
    let mut count = 0;
    
    for (start, part) in input.match_indices(from) {
        if count >= n {
            break;
        }
        result.push_str(&input[last_end..start]);
        result.push_str(to);
        last_end = start + part.len();
        count += 1;
    }
    
    result.push_str(&input[last_end..]);
    result
}
```

## Exercise-Specific Templates

### reverse-string
```rust
pub fn reverse(input: &str) -> String {
    // Simple: input.chars().rev().collect()
    // For grapheme clusters:
    // use unicode_segmentation::UnicodeSegmentation;
    // input.graphemes(true).rev().collect()
    input.chars().rev().collect()
}
```

### bob (Teenager Response)
```rust
pub fn reply(message: &str) -> &str {
    let message = message.trim();
    
    if message.is_empty() {
        return "Fine. Be that way!";
    }
    
    let is_question = message.ends_with('?');
    let is_yelling = message.chars().any(|c| c.is_alphabetic()) 
        && message.chars()
            .filter(|c| c.is_alphabetic())
            .all(|c| c.is_uppercase());
    
    match (is_question, is_yelling) {
        (true, true) => "Calm down, I know what I'm doing!",
        (true, false) => "Sure.",
        (false, true) => "Whoa, chill out!",
        (false, false) => "Whatever.",
    }
}
```

### acronym
```rust
pub fn abbreviate(phrase: &str) -> String {
    phrase
        .split(|c: char| c.is_whitespace() || c == '-' || c == '_')
        .filter(|word| !word.is_empty())
        .flat_map(|word| {
            std::iter::once(word.chars().next().unwrap())
                .chain(
                    word.chars()
                        .skip(1)
                        .zip(word.chars())
                        .filter(|(curr, prev)| {
                            curr.is_uppercase() && !prev.is_uppercase()
                        })
                        .map(|(c, _)| c)
                )
        })
        .collect::<String>()
        .to_uppercase()
}
```

### raindrops
```rust
pub fn raindrops(n: u32) -> String {
    let mut result = String::new();
    
    if n % 3 == 0 { result.push_str("Pling"); }
    if n % 5 == 0 { result.push_str("Plang"); }
    if n % 7 == 0 { result.push_str("Plong"); }
    
    if result.is_empty() {
        result = n.to_string();
    }
    
    result
}

// Alternative functional approach
pub fn raindrops_functional(n: u32) -> String {
    let sounds = [(3, "Pling"), (5, "Plang"), (7, "Plong")];
    
    let result: String = sounds
        .iter()
        .filter(|(divisor, _)| n % divisor == 0)
        .map(|(_, sound)| *sound)
        .collect();
    
    if result.is_empty() {
        n.to_string()
    } else {
        result
    }
}
```

## Common Edge Cases

### Always Test
1. **Empty strings**: `""`
2. **Single character**: `"a"`
3. **Unicode characters**: `"你好"`, `"🎉"`
4. **Whitespace only**: `"   "`
5. **Mixed whitespace**: `" \t\n "`
6. **Special characters**: `"!@#$%"`
7. **Very long strings**: Test with 10,000+ characters
8. **Grapheme clusters**: `"👨‍👩‍👧‍👦"` (family emoji)

## Performance Tips

### Allocation Optimization
```rust
// Bad: Multiple allocations
let mut s = String::new();
for c in chars {
    s.push(c);  // May reallocate multiple times
}

// Good: Pre-allocate
let mut s = String::with_capacity(chars.len());
for c in chars {
    s.push(c);  // No reallocation needed
}

// Best: Collect directly
let s: String = chars.iter().collect();
```

### Avoid Repeated Work
```rust
// Bad: Repeated trimming
if input.trim().is_empty() || input.trim().len() < 5 {
    // trim() called twice
}

// Good: Store trimmed version
let trimmed = input.trim();
if trimmed.is_empty() || trimmed.len() < 5 {
    // trim() called once
}
```

## Common Pitfalls and Solutions

### Character Counting
```rust
// Wrong: Counts bytes, not characters
let count = s.len();

// Right: Counts Unicode characters
let count = s.chars().count();

// For grapheme clusters (visual characters)
use unicode_segmentation::UnicodeSegmentation;
let count = s.graphemes(true).count();
```

### String Indexing
```rust
// Wrong: Can't index strings directly
// let first = s[0];  // Won't compile

// Right: Use chars iterator
let first = s.chars().next();

// Or nth for specific position
let third = s.chars().nth(2);
```

### Modifying Strings
```rust
// Inefficient: Creating new string each time
let mut s = "hello".to_string();
s = s + " world";  // Creates new String

// Efficient: Modify in place
let mut s = "hello".to_string();
s.push_str(" world");  // Modifies existing String

// Most efficient for multiple additions
let parts = ["hello", " ", "world"];
let s = parts.concat();  // Single allocation
```

## Testing String Functions

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_string() {
        assert_eq!(function(""), "");
    }

    #[test]
    fn test_unicode() {
        assert_eq!(function("😀🎉"), expected);
    }

    #[test]
    fn test_whitespace() {
        assert_eq!(function("  \t\n  "), expected);
    }
    
    #[test]
    fn test_grapheme_clusters() {
        // Family emoji is one grapheme but multiple chars
        let family = "👨‍👩‍👧‍👦";
        assert_eq!(function(family), expected);
    }
}