use std::fmt;

#[derive(PartialEq, Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

const MINUTES_IN_HOUR: i32 = 60;
const HOURS_IN_DAY: i32 = 24;
const DAYS_IN_MINUTES: i32 = MINUTES_IN_HOUR * HOURS_IN_DAY;

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock { hours, minutes }.normalize()
    }

    fn normalize(&self) -> Self {
        let total_minutes = ((((self.hours * MINUTES_IN_HOUR) + self.minutes) % DAYS_IN_MINUTES)
            + DAYS_IN_MINUTES)
            % DAYS_IN_MINUTES;
        Clock {
            hours: total_minutes / MINUTES_IN_HOUR,
            minutes: total_minutes % MINUTES_IN_HOUR,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes + minutes)
    }

    // pub fn to_string(&self) -> String {
    //     format!("{:0width$}:{:0width$}", self.hours, self.minutes, width = 2)
    // }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
