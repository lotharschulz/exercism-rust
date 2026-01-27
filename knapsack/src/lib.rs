#[derive(Debug)]
pub struct Item {
    pub weight: u32,
    pub value: u32,
}

pub fn maximum_value(max_weight: u32, items: &[Item]) -> u32 {
    // Space-optimized dynamic programming: O(W) space instead of O(n×W)
    let mut dp = vec![0; (max_weight + 1) as usize];
    for item in items {
        // Iterate backwards to avoid using updated values in same iteration
        for w in (item.weight..=max_weight).rev() {
            dp[w as usize] = dp[w as usize].max(dp[(w - item.weight) as usize] + item.value);
        }
    }
    dp[max_weight as usize]
    // Recursive solution: O(2^n) time
    // items.iter().enumerate().map(|(index, item)| {
    //     if item.weight <= max_weight {
    //         maximum_value(max_weight - item.weight, &items[index+1..]) + item.value
    //     } else {
    //         0
    //     }
    // }).max().unwrap_or(0)

    // Dynamic programming solution: O(n×W) time and space
    // let n = items.len();
    // let mut dp = vec![vec![0; (max_weight + 1) as usize]; n + 1];
    // for i in 1..=n {
    //     for w in 0..=max_weight {
    //         if items[i - 1].weight <= w {
    //             dp[i][w as usize] = dp[i - 1][(w - items[i - 1].weight) as usize]
    //                 .max(dp[i - 1][w as usize] + items[i - 1].value);
    //         } else {
    //             dp[i][w as usize] = dp[i - 1][w as usize];
    //         }
    //     }
    // }
    // dp[n][max_weight as usize]
}
