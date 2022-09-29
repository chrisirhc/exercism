use std::fmt;

const MINUTES_IN_AN_HOUR: i32 = 60;
const HOURS_IN_A_DAY: i32 = 24;

#[derive(PartialEq, Debug)]

pub struct Clock {
    minutes: i32,
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (hours, minutes) = Clock::normalize(0, self.minutes);
        write!(f, "{:02}:{:02}", hours, minutes)
    }
}

impl Clock {
    fn normalize_to_minutes(hours: i32, minutes: i32) -> i32 {
        Clock::to_minutes(Clock::normalize(hours, minutes))
    }

    fn normalize(hours: i32, minutes: i32) -> (i32, i32) {
        let new_minutes = minutes.rem_euclid(MINUTES_IN_AN_HOUR);
        let hours_carry_over = minutes.div_euclid(MINUTES_IN_AN_HOUR);
        let new_hours = (hours_carry_over + hours).rem_euclid(HOURS_IN_A_DAY);
        (new_hours, new_minutes)
    }

    fn to_minutes((hours, minutes): (i32, i32)) -> i32 {
        hours * MINUTES_IN_AN_HOUR + minutes
    }

    pub fn new(hours: i32, minutes: i32) -> Self {
        let new_minutes = Clock::normalize_to_minutes(hours, minutes);
        Clock {
            minutes: new_minutes,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let new_minutes = Clock::normalize_to_minutes(0, self.minutes + minutes);
        Clock {
            minutes: new_minutes,
        }
    }
}
