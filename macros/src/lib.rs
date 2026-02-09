#[macro_export]
macro_rules! hashmap {
    // Rule 1: Handle empty hashmap - hashmap!()
    () => {
        {
            let mut hm = ::std::collections::HashMap::new();
            hm
        }
    };
    // Rule 2: Handle one or more key-value pairs without trailing comma
    // Pattern: ($(...),+) means one or more comma-separated items
    ($($key:expr => $value:expr),+) => {
        {
            let mut hm = ::std::collections::HashMap::new();
            // $(...)*  repeats the inner code for each matched key-value pair
            $(
                hm.insert($key, $value);
            )*
            hm
        }
    };
    // Rule 3: Handle one or more key-value pairs with trailing comma
    // This allows: hashmap!(k => v, k => v,) but rejects: hashmap!(k => v, ,)
    ($($key:expr => $value:expr),+,) => {
        {
            let mut hm = ::std::collections::HashMap::new();
            $(
                hm.insert($key, $value);
            )*
            hm
        }
    };
}

/// This module contains doctests, which allows writing tests where a code
/// snippet is supposed to fail to compile. These tests also have "ignore"
/// attributes, makes sure to remove them when solving this exercise locally.
pub mod compile_fail_tests;
