use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub struct Clock {
    minutes: i32,
}

const MINUTES_PER_DAY: i32 = 24 * 60;

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut minutes = ((hours * 60) + minutes) % MINUTES_PER_DAY;
        if minutes < 0 {
            minutes += MINUTES_PER_DAY;
        }
        Self { minutes }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let minutes = self.minutes + minutes;
        Self::new(0, minutes)
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let hours = self.minutes / 60;
        let minutes = self.minutes % 60;
        write!(f, "{:02}:{:02}", hours, minutes)
    }
}
