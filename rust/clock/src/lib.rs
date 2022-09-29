use std::fmt;

const MINUTES_IN_AN_HOUR: i32 = 60;
const HOURS_IN_A_DAY: i32 = 24;

#[derive(PartialEq, Debug)]

pub struct Clock {
    minutes: i32,
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:02}:{:02}",
            self.minutes
                .div_euclid(MINUTES_IN_AN_HOUR)
                .rem_euclid(HOURS_IN_A_DAY),
            self.minutes.rem_euclid(MINUTES_IN_AN_HOUR)
        )
    }
}

impl Clock {
    fn normalize(hours: i32, minutes: i32) -> i32 {
        let new_minutes = minutes.rem_euclid(MINUTES_IN_AN_HOUR);
        let hours_carry_over = minutes.div_euclid(MINUTES_IN_AN_HOUR);
        let new_hours = (hours_carry_over + hours).rem_euclid(HOURS_IN_A_DAY);
        new_hours * MINUTES_IN_AN_HOUR + new_minutes
    }

    pub fn new(hours: i32, minutes: i32) -> Self {
        let new_minutes = Clock::normalize(hours, minutes);
        Clock {
            minutes: new_minutes,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let new_minutes = Clock::normalize(0, self.minutes + minutes);
        Clock {
            minutes: new_minutes,
        }
    }
}
