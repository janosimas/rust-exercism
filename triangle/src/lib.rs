use std::ops::{Add, Sub};

pub struct Triangle<Number> {
    sides: [Number; 3],
}

impl<Number> Triangle<Number> {
    pub fn build(mut sides: [Number; 3]) -> Option<Triangle<Number>>
    where
        Number: PartialOrd + Add<Output = Number> + Copy + Sub<Output = Number>,
    {
        let zero = sides[0] - sides[0];

        sides.sort_by(|a, b| a.partial_cmp(b).unwrap());
        if sides[0] == zero || (sides[0] + sides[1] < sides[2]) {
            return None;
        }

        Some(Triangle { sides })
    }

    pub fn is_equilateral(&self) -> bool
    where
        Number: PartialEq,
    {
        self.sides[0] == self.sides[1] && self.sides[0] == self.sides[2]
    }

    pub fn is_scalene(&self) -> bool
    where
        Number: PartialEq,
    {
        !self.is_equilateral() && !self.is_isosceles()
    }

    pub fn is_isosceles(&self) -> bool
    where
        Number: PartialEq,
    {
        !self.is_equilateral() && (self.sides[0] == self.sides[1] || self.sides[1] == self.sides[2])
    }
}
