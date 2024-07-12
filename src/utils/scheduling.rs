use chrono::{Datelike, Duration, NaiveDateTime, NaiveTime, Weekday};

#[derive(Default, PartialEq, Eq, Debug, Clone, Copy)]
pub enum Scheduling {
    // Job is not scheduled at all
    #[default]
    Unscheduled,

    // Job is scheduled to run immediately
    Immediate,

    // Job is scheduled to run once at a specific date and time
    OnceAt(NaiveDateTime),

    // Job is scheduled to run once after a specific duration
    OnceIn(Duration),

    // Job is scheduled to run every minute
    Minutely,

    // Job is scheduled to run every hour
    Hourly,

    // Job is scheduled to run every day at a specific time
    Daily(NaiveTime),

    // Job is scheduled to run every week on a specific day at a specific time
    Weekly(Weekday, NaiveTime),
}

impl Scheduling {
    pub fn is_unescheduled(&self) -> bool {
        matches!(self, Scheduling::Unscheduled)
    }

    // Returns all the occurences of the job between two dates
    pub fn occurences(&self, since: &NaiveDateTime, until: &NaiveDateTime) -> Vec<NaiveDateTime> {
        match self {
            Scheduling::Unscheduled => vec![],
            Scheduling::Immediate => vec![*since],
            Scheduling::OnceIn(d) => {
                if *since + *d <= *until {
                    vec![*since + *d]
                } else {
                    vec![]
                }
            }
            Scheduling::OnceAt(dt) => {
                if *since < *dt && *dt <= *until {
                    vec![*dt]
                } else {
                    vec![]
                }
            }
            Scheduling::Daily(time) => {
                let mut dt = NaiveDateTime::new(since.date(), *time);
                let mut occurrences = vec![];

                if time < &since.time() {
                    dt += Duration::days(1);
                }

                while dt <= *until {
                    occurrences.push(dt);
                    dt += Duration::days(1);
                }

                occurrences
            }
            _ => unimplemented!(),
        }
    }

    // Returns the duration to the next scheduled run
    pub fn time_to_next(&self, now: &NaiveDateTime) -> Option<Duration> {
        match self {
            Scheduling::Unscheduled => None,
            Scheduling::Immediate => Some(Duration::zero()),
            Scheduling::OnceIn(d) => Some(*d),
            Scheduling::OnceAt(dt) => {
                let duration = dt.signed_duration_since(*now);
                if duration.num_milliseconds() < 0 {
                    None
                } else {
                    Some(duration)
                }
            }
            Scheduling::Daily(time) => {
                let dt = NaiveDateTime::new(now.date(), *time);
                let duration = if dt < *now {
                    dt + Duration::days(1) - *now
                } else {
                    dt - *now
                };

                Some(duration)
            }
            _ => unimplemented!(),
        }
    }

    // Returns next scheduling state if the job is consired as expired and executed
    pub fn into_next(self) -> Self {
        match self {
            Scheduling::Unscheduled => Scheduling::Unscheduled,
            Scheduling::Immediate => Scheduling::Unscheduled,
            Scheduling::OnceIn(_d) => Scheduling::Unscheduled,
            Scheduling::OnceAt(_dt) => Scheduling::Unscheduled,
            Scheduling::Minutely => self,
            Scheduling::Hourly => self,
            Scheduling::Daily(_time) => self,
            Scheduling::Weekly(_weekday, _time) => self,
        }
    }
}
