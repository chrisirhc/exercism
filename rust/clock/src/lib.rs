use std::fmt;

#[derive(PartialEq, Debug)]

pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl Clock {
    fn normalize(hours: i32, minutes: i32) -> (i32, i32) {
        let new_minutes: i32;
        let mut hours_carry_over = 0;
        if minutes < 0 && minutes % 60 != 0 {
            new_minutes = (minutes % 60) + 60;
            hours_carry_over = -1;
        } else {
            new_minutes = minutes % 60;
        }
        hours_carry_over = (minutes / 60) + hours_carry_over;

        let mut new_hours = (hours_carry_over + hours) % 24;
        if new_hours < 0 {
            new_hours = new_hours + 24; 
        }
        (new_hours, new_minutes)
    }

    pub fn new(hours: i32, minutes: i32) -> Self {
        let (new_hours, new_minutes) = Clock::normalize(hours, minutes);
        Clock {
            hours: new_hours,
            minutes: new_minutes,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let (new_hours, new_minutes) = Clock::normalize(self.hours, self.minutes + minutes);
        Clock {
            hours: new_hours,
            minutes: new_minutes,
        }
    }
}
