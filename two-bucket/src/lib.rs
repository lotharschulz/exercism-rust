#[derive(PartialEq, Eq, Debug)]
pub enum Bucket {
    One,
    Two,
}

#[derive(PartialEq, Eq, Debug)]
pub struct BucketStats {
    pub moves: u8,
    pub goal_bucket: Bucket,
    pub other_bucket: u8,
}

pub fn solve(
    capacity_1: u8,
    capacity_2: u8,
    goal: u8,
    start_bucket: &Bucket,
) -> Option<BucketStats> {
    if (capacity_2 - capacity_1) % goal != 0 && capacity_1 != goal && capacity_2 != goal {
        return None;
    }

    let mut c1_bucket = Bucket::One;
    let mut c2_bucket = Bucket::Two;
    let (mut c1_max, mut c2_max) = (capacity_1, capacity_2);
    if *start_bucket == Bucket::Two {
        c1_bucket = Bucket::Two;
        c2_bucket = Bucket::One;
        (c1_max, c2_max) = (capacity_2, capacity_1);
    }
    let (mut c1, mut c2) = (0, 0);
    let mut moves = 0;
    loop {
        if c1 == goal {
            return Some(BucketStats {
                moves,
                goal_bucket: c1_bucket,
                other_bucket: c2,
            });
        }
        if c2 == goal {
            return Some(BucketStats {
                moves,
                goal_bucket: c2_bucket,
                other_bucket: c1,
            });
        }

        if c2 == c2_max {
            c2 = 0;
            moves += 1;
            continue;
        }

        if c1 == 0 {
            c1 = c1_max;
            moves += 1;
            if c2_max == goal {
                c2 = c2_max;
                moves += 1;
            }
            continue;
        }
        if c1 > 0 && c2 < c2_max {
            if (c2_max - c2) >= c1 {
                c2 += c1;
                c1 = 0;
            } else {
                c1 = c1 - (c2_max - c2);
                c2 = c2_max;
            }
            moves += 1;
            continue;
        }
    }
}
