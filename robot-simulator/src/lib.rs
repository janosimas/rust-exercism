#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

static DIRECTIONS: [Direction; 4] = [
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
        self.d = *DIRECTIONS
            .iter()
            .cycle()
            .skip_while(|&d| *d != self.d)
            .nth(1)
            .unwrap();
        self
    }

    pub fn turn_left(mut self) -> Self {
        self.d = *DIRECTIONS
            .iter()
            .rev()
            .cycle()
            .skip_while(|&d| *d != self.d)
            .nth(1)
            .unwrap();
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
