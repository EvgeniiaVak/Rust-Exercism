#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(Debug)]
enum Frame {
    Last(u16, u16, Option<u16>),
    Open(u16, u16),
    Spare(u16, u16),
    Strike,
}

pub struct BowlingGame {
    incomplete_frame: Option<Vec<u16>>,
    frames: Vec<Frame>,
}

impl BowlingGame {
    pub fn new() -> Self {
        Self {
            frames: vec![],
            incomplete_frame: None,
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if pins > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }
        if self.frames.len() == 10 {
            return Err(Error::GameComplete);
        }

        if let Some(mut prev_rolls) = self.incomplete_frame.take() {
            // the last frame is special
            if self.frames.len() == 9 {
                match prev_rolls.len() {
                    1 => {
                        let prev_roll_strike = prev_rolls[0] == 10;
                        if !prev_roll_strike && (prev_rolls[0] + pins) > 10 {
                            return Err(Error::NotEnoughPinsLeft);
                        }

                        prev_rolls.push(pins);

                        if prev_rolls[0] + prev_rolls[1] >= 10 {
                            // If someone throws a strike or a spare then they get a fill ball.
                            self.incomplete_frame = Some(prev_rolls);
                        } else {
                            self.frames
                                .push(Frame::Last(prev_rolls[0], prev_rolls[1], None));
                        }
                    }
                    2 => {
                        // the_two_balls_after_a_final_strike_cannot_score_an_invalid_number_of_pins
                        let prev_rolls_open = prev_rolls[0] + prev_rolls[1] == 10;
                        let prev_roll_strike = prev_rolls[1] == 10;
                        if !prev_rolls_open && !prev_roll_strike && (prev_rolls[1] + pins) > 10 {
                            return Err(Error::NotEnoughPinsLeft);
                        }

                        self.frames
                            .push(Frame::Last(prev_rolls[0], prev_rolls[1], Some(pins)));
                    }
                    _ => return Err(Error::GameComplete),
                }
            } else if prev_rolls.len() == 1 {
                let total_pins = prev_rolls[0] + pins;
                if total_pins > 10 {
                    return Err(Error::NotEnoughPinsLeft);
                }
                if total_pins == 10 {
                    self.frames.push(Frame::Spare(prev_rolls[0], pins));
                } else {
                    self.frames.push(Frame::Open(prev_rolls[0], pins));
                }
            }
        } else if pins == 10 {
            if self.frames.len() == 9 {
                self.incomplete_frame = Some(vec![pins]);
            } else {
                self.frames.push(Frame::Strike);
            }
        } else {
            self.incomplete_frame = Some(vec![pins]);
        }

        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        if self.frames.len() < 10 {
            return None;
        }

        let mut score = 0;

        for (i, frame) in self.frames.iter().enumerate() {
            match frame {
                Frame::Last(a, b, c) => {
                    let c = match c {
                        Some(c) => c,
                        None => &0,
                    };
                    score += a + b + c;
                }
                Frame::Open(a, b) => {
                    score += a + b;
                }
                Frame::Spare(a, b) => {
                    score += a + b;
                    let next_roll = match self.frames.get(i + 1) {
                        Some(Frame::Open(a, _)) => a,
                        Some(Frame::Spare(a, _)) => a,
                        Some(Frame::Strike) => &10,
                        Some(Frame::Last(a, _, _)) => a,
                        _ => &0,
                    };
                    score += next_roll;
                }
                Frame::Strike => {
                    score += 10;
                    let (a, b) = match self.frames.get(i + 1) {
                        Some(Frame::Open(a, b)) => (a, b),
                        Some(Frame::Spare(a, b)) => (a, b),
                        Some(Frame::Last(a, b, _)) => (a, b),
                        Some(Frame::Strike) => {
                            let a = &10;
                            let b = match self.frames.get(i + 2) {
                                Some(Frame::Open(b, _)) => b,
                                Some(Frame::Spare(b, _)) => b,
                                Some(Frame::Last(b, _, _)) => b,
                                Some(Frame::Strike) => &10,
                                _ => &0,
                            };
                            (a, b)
                        }
                        _ => (&0, &0),
                    };
                    score += a + b;
                }
            }
        }

        Some(score)
    }
}

impl Default for BowlingGame {
    fn default() -> Self {
        Self::new()
    }
}
