#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Direction {
    North = 0,
    East = 1,
    South = 2,
    West = 3,
}

impl From<u8> for Direction {
    fn from(d: u8) -> Self {
        match d % 4 {
            0 => Self::North,
            1 => Self::East,
            2 => Self::South,
            3 => Self::West,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Robot {
    x: i32,
    y: i32,
    d: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Self { x, y, d }
    }

    #[must_use]
    pub fn turn_right(self) -> Self {
        Self {
            d: (self.d as u8 + 1).into(),
            ..self
        }
    }

    #[must_use]
    pub fn turn_left(self) -> Self {
        Self {
            d: (self.d as u8 + 3).into(),
            ..self
        }
    }

    #[must_use]
    pub fn advance(self) -> Self {
        let (mut x, mut y) = self.position();

        match self.direction() {
            Direction::North => y += 1,
            Direction::East => x += 1,
            Direction::South => y -= 1,
            Direction::West => x -= 1,
        }

        Self { x, y, ..self }
    }

    #[must_use]
    pub fn instructions(self, instructions: &str) -> Self {
        let mut robot = self;
        for c in instructions.chars() {
            robot = match c {
                'A' => robot.advance(),
                'L' => robot.turn_left(),
                'R' => robot.turn_right(),
                _ => unimplemented!(),
            }
        }

        robot
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.d
    }
}
