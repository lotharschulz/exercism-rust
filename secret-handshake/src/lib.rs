pub fn actions(n: u8) -> Vec<&'static str> {
    // The secret handshake consists of the following actions:
    // 1 (1): "wink"
    // 2 (10): "double blink"
    // 4 (100): "close your eyes"
    // 8 (1000): "jump"
    // 16 (10000): reverse the order of the actions
    // Convert the number to binary and determine which actions to include
    let binary = format!("{:b}", n);
    // Create a vector to hold the actions
    let mut actions = Vec::new();
    // Check each bit and add the corresponding action
    for (bit_pos, action) in [(0, "wink"), (1, "double blink"), (2, "close your eyes"), (3, "jump")] {
        match (n >> bit_pos) & 1 {
            1 => actions.push(action),
            _ => {}
        }
    }
    // Check if the reverse bit is set
    match (n >> 4) & 1 {
        1 => actions.reverse(),
        _ => {}
    }
    actions   
}
