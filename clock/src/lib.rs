use std::fmt;

#[derive(Debug)]
pub struct Clock {
    hours: f32,
}

impl Clock {
    const HOURS_IN_DAY: f32 = 24.;
    const MINUTES_IN_HOUR: f32 = 60.;

    pub fn hours(&self) -> i32 {
        self.hours.trunc() as i32
    }
    pub fn minutes(&self) -> i32 {
        (self.hours.fract() * Clock::MINUTES_IN_HOUR).round() as i32
    }

    fn round_to_24h(hours: f32) -> f32 {
        hours.rem_euclid(Clock::HOURS_IN_DAY)
    }

    fn normalize_time(hours: f32, minutes: f32) -> f32 {
        Clock::round_to_24h(hours + minutes / Clock::MINUTES_IN_HOUR)
    }

    pub fn new(hours: i32, minutes: i32) -> Self {
        let hours = hours as f32;
        let minutes = minutes as f32;

        Clock {
            hours: Clock::normalize_time(hours, minutes),
        }
    }

    pub fn add_minutes(&mut self, minutes: i32) -> Self {
        let minutes = minutes as f32;

        Clock {
            hours: Clock::normalize_time(self.hours, minutes),
        }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours(), self.minutes())
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.hours() == other.hours() && self.minutes() == other.minutes()
    }
}

impl Eq for Clock {}
