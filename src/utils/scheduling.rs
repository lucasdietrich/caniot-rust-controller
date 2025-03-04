use chrono::{DateTime, Duration, Local, NaiveDateTime, NaiveTime, Utc, Weekday};
use log::debug;

#[derive(Default, PartialEq, Eq, Debug, Clone, Copy)]
pub enum Scheduling {
    // Job is not scheduled at all
    #[default]
    Unscheduled,

    // Job is scheduled to run immediately
    Immediate,

    // Job is scheduled to run once at a specific date and time
    OnceAt(DateTime<Utc>),

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
    // The since date is exclusive and the until date is inclusive, i.e. ]since, until]
    // This is because the since date is the last date the job was executed
    pub fn occurences(&self, since: &DateTime<Utc>, until: &DateTime<Utc>) -> Vec<DateTime<Utc>> {
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
            Scheduling::Daily(local_time) => {
                let local_since = DateTime::<Local>::from(*since);
                let local_until = DateTime::<Local>::from(*until);

                let mut local_naive_dt = NaiveDateTime::new(local_since.date_naive(), *local_time);

                if local_naive_dt.time() <= local_since.time() {
                    local_naive_dt += Duration::days(1);
                }

                let mut occurrences = vec![];
                while local_naive_dt <= local_until.naive_local() {
                    occurrences.push(DateTime::<Utc>::from(
                        local_naive_dt.and_local_timezone(Local).single().unwrap(),
                    ));
                    local_naive_dt += Duration::days(1);
                }

                return occurrences;
            }
            _ => unimplemented!(),
        }
    }

    // Returns the duration to the next scheduled run
    pub fn time_to_next(&self, now: &DateTime<Utc>) -> Option<Duration> {
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
            Scheduling::Daily(local_event_time) => {
                let local_now_time = DateTime::<Local>::from(*now).naive_local().time();

                let time_to_next = if local_now_time <= *local_event_time {
                    *local_event_time - local_now_time
                } else {
                    (*local_event_time - local_now_time) + Duration::days(1)
                };

                debug!(
                    "local_now_time: {:?}, local_event_time: {:?}, time_to_next: {:?}",
                    local_now_time, local_event_time, time_to_next
                );

                Some(time_to_next)
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
            Scheduling::Daily(..) => self,
            Scheduling::Weekly(..) => self,
        }
    }
}
