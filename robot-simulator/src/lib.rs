// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

const DIRECTIONS_ORDER: [Direction; 4] = [
    Direction::North,
    Direction::East,
    Direction::South,
    Direction::West,
];

pub struct Robot {
    x: i32,
    y: i32,
    d: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Robot { x, y, d }
    }

    pub fn turn_right(mut self) -> Self {
        let index = DIRECTIONS_ORDER
            .iter()
            .position(|d| *d == self.d)
            .map(|i| {
                if i == DIRECTIONS_ORDER.len() - 1 {
                    0
                } else {
                    i + 1
                }
            })
            .unwrap();
        self.d = DIRECTIONS_ORDER[index];
        self
    }

    pub fn turn_left(mut self) -> Self {
        let index = DIRECTIONS_ORDER
            .iter()
            .position(|d| *d == self.d)
            .map(|i| {
                if i == 0 {
                    DIRECTIONS_ORDER.len() - 1
                } else {
                    i - 1
                }
            })
            .unwrap();
        self.d = DIRECTIONS_ORDER[index];
        self
    }

    pub fn advance(mut self) -> Self {
        match self.d {
            Direction::North => self.y += 1,
            Direction::East => self.x += 1,
            Direction::South => self.y -= 1,
            Direction::West => self.x -= 1,
        };
        self
    }

    pub fn instructions(self, instructions: &str) -> Self {
        instructions
            .chars()
            .fold(self, |new_self, instruction| match instruction {
                'R' => new_self.turn_right(),
                'L' => new_self.turn_left(),
                'A' => new_self.advance(),
                _ => new_self,
            })
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.d
    }
}
