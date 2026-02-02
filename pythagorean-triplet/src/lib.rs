use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    (1..=sum / 3)
        .flat_map(|a| {
            (a..=(sum - a) / 2).map(move |b| {
                let c = sum - a - b;
                if a * a + b * b == c * c {
                    Some([a, b, c])
                } else {
                    None
                }
            })
        })
        .flatten()
        .collect()
}
