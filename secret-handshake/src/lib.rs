pub fn actions(n: u8) -> Vec<&'static str> {
    // The secret handshake consists of the following actions:
    // 1 (1): "wink"
    // 2 (10): "double blink"
    // 4 (100): "close your eyes"
    // 8 (1000): "jump"
    // 16 (10000): reverse the order of the actions

    // Create a vector to hold the actions
    let mut actions = Vec::new();

    // Check each bit and add the corresponding action
    if n & 1 != 0 {
        actions.push("wink");
    }
    if n & 2 != 0 {
        actions.push("double blink");
    }
    if n & 4 != 0 {
        actions.push("close your eyes");
    }
    if n & 8 != 0 {
        actions.push("jump");
    }
    // Check if the reverse bit is set
    if n & 16 != 0 {
        actions.reverse();
    }
    actions   
}
