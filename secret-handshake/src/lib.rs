pub fn actions(n: u8) -> Vec<&'static str> {
    // Define the possible actions with their corresponding bit values
    const ACTIONS: [(u8, &str); 4] = [
        (1, "wink"),
        (2, "double blink"),
        (4, "close your eyes"),
        (8, "jump"),
    ];

    // Collect the actions based on the bits set in n
    //
    // iterate through the ACTIONS array using iter() that creates an iterator
    // filter the actions accordingly using filter() that returns an iterator
    // map to extract the action strings using map() that also returns an iterator
    // materialize the iterator into a vector using collect()
    // Without collect(), the result would remain an iterator and not a concrete collection aka Vec
    // Finally, check if the reverse bit is set and reverse the order if needed
    let mut result: Vec<&str> = ACTIONS
        .iter()
        .filter(|(bit, _)| n & bit != 0)
        .map(|(_, action)| *action)
        .collect();

    if n & 16 != 0 {
        result.reverse();
    }

    result
}
