use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    let mut triplets = HashSet::new();

    for a in 1..sum {
        for b in a..(sum - a) {
            let c = sum - a - b;
            if c > 0 && a * a + b * b == c * c {
                let mut triplet = [a, b, c];
                triplet.sort_unstable();
                triplets.insert(triplet);
            }
        }
    }

    triplets
}
