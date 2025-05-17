use std::cmp::{PartialEq, PartialOrd};
use std::ops::Add;

pub struct Triangle<T> {
    sides: [T; 3],
}

impl<T> Triangle<T>
where
    T: Copy + PartialEq + PartialOrd + Add<Output = T> + Default,
{
    pub fn build(sides: [T; 3]) -> Option<Triangle<T>> {
        // Check if any side is zero (or negative)
        if sides.iter().any(|&side| side <= T::default()) {
            return None;
        }

        // Check triangle inequality: sum of any two sides must be greater than the third side
        if !(sides[0] + sides[1] > sides[2]
            && sides[1] + sides[2] > sides[0]
            && sides[0] + sides[2] > sides[1])
        {
            return None;
        }

        Some(Triangle { sides })
    }

    pub fn is_equilateral(&self) -> bool {
        self.sides[0] == self.sides[1] && self.sides[1] == self.sides[2]
    }

    pub fn is_scalene(&self) -> bool {
        self.sides[0] != self.sides[1]
            && self.sides[1] != self.sides[2]
            && self.sides[0] != self.sides[2]
    }

    pub fn is_isosceles(&self) -> bool {
        self.sides[0] == self.sides[1]
            || self.sides[1] == self.sides[2]
            || self.sides[0] == self.sides[2]
    }
}
