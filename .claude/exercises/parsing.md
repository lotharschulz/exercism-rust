# Parsing Exercise Patterns

## Core Principles for Parsing Exercises

### Error Handling
- Always return `Result<T, E>` for fallible parsing
- Create custom error types for clarity
- Provide helpful error messages
- Never panic on invalid input

### Input Validation
- Validate input format before processing
- Check boundaries and constraints
- Handle edge cases gracefully
- Consider Unicode and special characters

## Common Parsing Patterns

### Custom Error Types
```rust
#[derive(Debug, PartialEq)]
pub enum ParseError {
    InvalidFormat(String),
    InvalidCharacter(char),
    InvalidLength { expected: usize, actual: usize },
    OutOfRange { value: String, min: i32, max: i32 },
    MissingField(String),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ParseError::InvalidFormat(s) => write!(f, "Invalid format: {}", s),
            ParseError::InvalidCharacter(c) => write!(f, "Invalid character: '{}'", c),
            ParseError::InvalidLength { expected, actual } => {
                write!(f, "Expected length {}, got {}", expected, actual)
            }
            ParseError::OutOfRange { value, min, max } => {
                write!(f, "{} is out of range [{}, {}]", value, min, max)
            }
            ParseError::MissingField(field) => write!(f, "Missing field: {}", field),
        }
    }
}

impl std::error::Error for ParseError {}
```

### String Parsing Utilities
```rust
/// Clean and normalize input
pub fn normalize_input(input: &str) -> String {
    input
        .trim()
        .to_lowercase()
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect()
}

/// Parse with validation
pub fn parse_with_validation<T, F, V>(
    input: &str,
    parser: F,
    validator: V,
) -> Result<T, ParseError>
where
    F: Fn(&str) -> Result<T, ParseError>,
    V: Fn(&T) -> bool,
{
    let parsed = parser(input)?;
    if validator(&parsed) {
        Ok(parsed)
    } else {
        Err(ParseError::InvalidFormat(input.to_string()))
    }
}
```

### Number Parsing
```rust
/// Parse phone number
pub fn parse_phone_number(input: &str) -> Result<String, ParseError> {
    // Remove all non-digit characters
    let digits: String = input
        .chars()
        .filter(|c| c.is_ascii_digit())
        .collect();
    
    // Validate length
    let digits = match digits.len() {
        10 => digits,
        11 if digits.starts_with('1') => digits[1..].to_string(),
        _ => return Err(ParseError::InvalidLength {
            expected: 10,
            actual: digits.len(),
        }),
    };
    
    // Validate area code and exchange code
    let area_code = &digits[0..3];
    let exchange = &digits[3..6];
    
    if area_code.starts_with('0') || area_code.starts_with('1') {
        return Err(ParseError::InvalidFormat("Invalid area code".to_string()));
    }
    
    if exchange.starts_with('0') || exchange.starts_with('1') {
        return Err(ParseError::InvalidFormat("Invalid exchange code".to_string()));
    }
    
    Ok(digits)
}

/// Parse ISBN-10
pub fn parse_isbn(isbn: &str) -> Result<String, ParseError> {
    let cleaned: String = isbn
        .chars()
        .filter(|c| c.is_ascii_digit() || *c == 'X')
        .collect();
    
    if cleaned.len() != 10 {
        return Err(ParseError::InvalidLength {
            expected: 10,
            actual: cleaned.len(),
        });
    }
    
    // X can only be at the end
    if cleaned.chars().take(9).any(|c| c == 'X') {
        return Err(ParseError::InvalidCharacter('X'));
    }
    
    Ok(cleaned)
}
```

### Roman Numerals
```rust
use std::collections::HashMap;

pub struct RomanNumeral {
    value: u32,
}

impl RomanNumeral {
    pub fn from(value: u32) -> Self {
        RomanNumeral { value }
    }
    
    pub fn to_string(&self) -> String {
        let values = [
            (1000, "M"), (900, "CM"), (500, "D"), (400, "CD"),
            (100, "C"), (90, "XC"), (50, "L"), (40, "XL"),
            (10, "X"), (9, "IX"), (5, "V"), (4, "IV"), (1, "I"),
        ];
        
        let mut result = String::new();
        let mut remaining = self.value;
        
        for (value, numeral) in values.iter() {
            while remaining >= *value {
                result.push_str(numeral);
                remaining -= value;
            }
        }
        
        result
    }
    
    pub fn from_string(s: &str) -> Result<Self, ParseError> {
        let mut values: HashMap<char, u32> = HashMap::new();
        values.insert('I', 1);
        values.insert('V', 5);
        values.insert('X', 10);
        values.insert('L', 50);
        values.insert('C', 100);
        values.insert('D', 500);
        values.insert('M', 1000);
        
        let chars: Vec<char> = s.chars().collect();
        
        // Validate characters
        for c in &chars {
            if !values.contains_key(c) {
                return Err(ParseError::InvalidCharacter(*c));
            }
        }
        
        let mut total = 0;
        let mut prev_value = 0;
        
        for c in chars.iter().rev() {
            let value = values[c];
            if value < prev_value {
                total -= value;
            } else {
                total += value;
            }
            prev_value = value;
        }
        
        Ok(RomanNumeral { value: total })
    }
}
```

### Complex Format Parsing
```rust
/// Parse time format (HH:MM:SS)
pub fn parse_time(input: &str) -> Result<(u8, u8, u8), ParseError> {
    let parts: Vec<&str> = input.split(':').collect();
    
    if parts.len() != 3 {
        return Err(ParseError::InvalidFormat("Expected HH:MM:SS".to_string()));
    }
    
    let hours = parts[0].parse::<u8>()
        .map_err(|_| ParseError::InvalidFormat("Invalid hours".to_string()))?;
    let minutes = parts[1].parse::<u8>()
        .map_err(|_| ParseError::InvalidFormat("Invalid minutes".to_string()))?;
    let seconds = parts[2].parse::<u8>()
        .map_err(|_| ParseError::InvalidFormat("Invalid seconds".to_string()))?;
    
    if hours >= 24 {
        return Err(ParseError::OutOfRange {
            value: hours.to_string(),
            min: 0,
            max: 23,
        });
    }
    
    if minutes >= 60 || seconds >= 60 {
        return Err(ParseError::InvalidFormat("Minutes/seconds must be < 60".to_string()));
    }
    
    Ok((hours, minutes, seconds))
}

/// Parse key-value pairs
pub fn parse_config(input: &str) -> Result<HashMap<String, String>, ParseError> {
    let mut config = HashMap::new();
    
    for line in input.lines() {
        let line = line.trim();
        
        // Skip empty lines and comments
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        
        let parts: Vec<&str> = line.splitn(2, '=').collect();
        if parts.len() != 2 {
            return Err(ParseError::InvalidFormat(format!("Invalid line: {}", line)));
        }
        
        let key = parts[0].trim();
        let value = parts[1].trim();
        
        if key.is_empty() {
            return Err(ParseError::MissingField("key".to_string()));
        }
        
        config.insert(key.to_string(), value.to_string());
    }
    
    Ok(config)
}
```

## Exercise-Specific Templates

### phone-number
```rust
pub struct PhoneNumber {
    number: String,
}

impl PhoneNumber {
    pub fn from(input: &str) -> Result<PhoneNumber, String> {
        // Extract digits
        let digits: String = input
            .chars()
            .filter(|c| c.is_ascii_digit())
            .collect();
        
        // Handle country code
        let number = match digits.len() {
            10 => digits,
            11 if digits.starts_with('1') => digits[1..].to_string(),
            _ => return Err("Invalid number of digits".to_string()),
        };
        
        // Validate area and exchange codes
        let area = &number[0..3];
        let exchange = &number[3..6];
        
        if area.starts_with('0') || area.starts_with('1') {
            return Err("Invalid area code".to_string());
        }
        
        if exchange.starts_with('0') || exchange.starts_with('1') {
            return Err("Invalid exchange code".to_string());
        }
        
        Ok(PhoneNumber { number })
    }
    
    pub fn number(&self) -> &str {
        &self.number
    }
    
    pub fn area_code(&self) -> &str {
        &self.number[0..3]
    }
    
    pub fn pretty(&self) -> String {
        format!(
            "({}) {}-{}",
            &self.number[0..3],
            &self.number[3..6],
            &self.number[6..10]
        )
    }
}
```

### isbn-verifier
```rust
pub fn is_valid_isbn(isbn: &str) -> bool {
    let cleaned: Vec<char> = isbn
        .chars()
        .filter(|c| c.is_ascii_digit() || *c == 'X')
        .collect();
    
    if cleaned.len() != 10 {
        return false;
    }
    
    // X can only be at position 9 (last)
    if cleaned.iter().take(9).any(|&c| c == 'X') {
        return false;
    }
    
    let checksum: u32 = cleaned
        .iter()
        .enumerate()
        .map(|(i, &c)| {
            let value = if c == 'X' {
                10
            } else {
                c.to_digit(10).unwrap()
            };
            value * (10 - i as u32)
        })
        .sum();
    
    checksum % 11 == 0
}
```

### allergies
```rust
#[derive(Debug, PartialEq, Clone)]
pub enum Allergen {
    Eggs = 1,
    Peanuts = 2,
    Shellfish = 4,
    Strawberries = 8,
    Tomatoes = 16,
    Chocolate = 32,
    Pollen = 64,
    Cats = 128,
}

pub struct Allergies {
    score: u32,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Allergies { score: score & 255 }  // Only consider first 8 bits
    }
    
    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        let allergen_value = allergen.clone() as u32;
        self.score & allergen_value != 0
    }
    
    pub fn allergies(&self) -> Vec<Allergen> {
        use Allergen::*;
        
        let all_allergens = vec![
            Eggs, Peanuts, Shellfish, Strawberries,
            Tomatoes, Chocolate, Pollen, Cats,
        ];
        
        all_allergens
            .into_iter()
            .filter(|allergen| self.is_allergic_to(allergen))
            .collect()
    }
}
```

## Regular Expression Patterns

```rust
use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref EMAIL_REGEX: Regex = Regex::new(
        r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$"
    ).unwrap();
    
    static ref URL_REGEX: Regex = Regex::new(
        r"^https?://[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}"
    ).unwrap();
    
    static ref HEX_COLOR_REGEX: Regex = Regex::new(
        r"^#[0-9a-fA-F]{6}$"
    ).unwrap();
}

pub fn is_valid_email(email: &str) -> bool {
    EMAIL_REGEX.is_match(email)
}

pub fn is_valid_url(url: &str) -> bool {
    URL_REGEX.is_match(url)
}

pub fn is_valid_hex_color(color: &str) -> bool {
    HEX_COLOR_REGEX.is_match(color)
}
```

## State Machine Parsing

```rust
#[derive(Debug, PartialEq)]
enum State {
    Start,
    InWord,
    InNumber,
    InWhitespace,
}

pub fn tokenize(input: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current_token = String::new();
    let mut state = State::Start;
    
    for ch in input.chars() {
        let new_state = match ch {
            c if c.is_alphabetic() => State::InWord,
            c if c.is_numeric() => State::InNumber,
            c if c.is_whitespace() => State::InWhitespace,
            _ => State::Start,
        };
        
        if new_state != state && state != State::Start {
            if !current_token.is_empty() {
                tokens.push(current_token.clone());
                current_token.clear();
            }
        }
        
        if new_state != State::InWhitespace {
            current_token.push(ch);
        }
        
        state = new_state;
    }
    
    if !current_token.is_empty() {
        tokens.push(current_token);
    }
    
    tokens
}
```

## Testing Parsing Functions

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_input() {
        assert!(parse_function("valid input").is_ok());
    }

    #[test]
    fn test_invalid_format() {
        let result = parse_function("invalid");
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            ParseError::InvalidFormat("...".to_string())
        );
    }

    #[test]
    fn test_empty_input() {
        assert!(parse_function("").is_err());
    }

    #[test]
    fn test_whitespace_handling() {
        assert_eq!(
            parse_function("  input  ").unwrap(),
            parse_function("input").unwrap()
        );
    }

    #[test]
    fn test_unicode_input() {
        assert!(parse_function("🎉 unicode 你好").is_ok());
    }

    #[test]
    fn test_boundary_values() {
        assert!(parse_function("a").is_ok());  // Minimum valid
        assert!(parse_function(&"x".repeat(1000)).is_ok());  // Large input
    }

    #[test]
    fn test_special_characters() {
        let special = "!@#$%^&*()_+-=[]{}|;:',.<>?/";
        for ch in special.chars() {
            let result = parse_function(&ch.to_string());
            // Assert expected behavior for each
        }
    }
}
```

## Error Recovery Strategies

```rust
/// Parse with recovery
pub fn parse_with_recovery(input: &str) -> (Vec<ParsedItem>, Vec<ParseError>) {
    let mut items = Vec::new();
    let mut errors = Vec::new();
    
    for line in input.lines() {
        match parse_line(line) {
            Ok(item) => items.push(item),
            Err(e) => errors.push(e),
        }
    }
    
    (items, errors)
}

/// Best effort parsing
pub fn parse_best_effort(input: &str) -> ParseResult {
    // Try strict parsing first
    if let Ok(result) = parse_strict(input) {
        return ParseResult::Success(result);
    }
    
    // Try with normalization
    let normalized = normalize_input(input);
    if let Ok(result) = parse_strict(&normalized) {
        return ParseResult::SuccessWithWarnings(result, vec!["Input was normalized"]);
    }
    
    // Partial parse
    ParseResult::Partial(parse_what_we_can(input))
}
```

## Performance Considerations

1. **Compile regex once**: Use `lazy_static!` for regex patterns
2. **Avoid repeated allocations**: Reuse buffers when possible
3. **Use iterators**: Process input lazily when appropriate
4. **Short-circuit on errors**: Return early on validation failures
5. **Consider `nom` for complex parsing**: Better performance for complex grammars

## Exercise Checklist

- [ ] Return `Result<T, E>` for all parsing functions
- [ ] Create meaningful error types
- [ ] Handle empty input gracefully
- [ ] Trim and normalize input appropriately
- [ ] Validate all constraints
- [ ] Test with invalid inputs
- [ ] Test with Unicode/special characters
- [ ] Document expected formats
- [ ] Consider using regex for complex patterns
- [ ] Add fuzzing tests for robustness