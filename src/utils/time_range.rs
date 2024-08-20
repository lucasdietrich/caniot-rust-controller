use chrono::{DateTime, Local, NaiveTime, Utc};

// Represent a time range in the Local timezone
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TimeRange {
    // Included boundary
    lower_bound: NaiveTime,
    // Excluded boundary
    upper_bound: NaiveTime,
    // tells whether lower bound is greater than upper bound
    // in which case comparison is inverted
    inverted: bool,
}

impl TimeRange {
    pub fn new(lower_bound: NaiveTime, upper_bound: NaiveTime) -> Self {
        Self {
            lower_bound,
            upper_bound,
            inverted: lower_bound > upper_bound,
        }
    }

    pub fn contains(&self, time: &NaiveTime) -> bool {
        let after_lower = time >= &self.lower_bound;
        let before_upper = time < &self.upper_bound;
        if self.inverted {
            after_lower || before_upper
        } else {
            after_lower && before_upper
        }
    }

    pub fn contains_utc(&self, utc: &DateTime<Utc>) -> bool {
        self.contains_local(&DateTime::<Local>::from(*utc))
    }

    pub fn contains_local(&self, local: &DateTime<Local>) -> bool {
        self.contains(&local.time())
    }
}

#[cfg(test)]
mod tests {
    use chrono::{DateTime, Local, NaiveDateTime, NaiveTime};

    use super::TimeRange;

    #[test]
    fn test_normal() {
        let t1 = NaiveTime::from_hms_opt(5, 0, 0).unwrap();
        let t2 = NaiveTime::from_hms_opt(7, 0, 0).unwrap();
        let range = TimeRange::new(t1, t2);

        assert!(!range.contains(&NaiveTime::from_hms_opt(0, 0, 0).unwrap()));
        assert!(!range.contains(&NaiveTime::from_hms_opt(4, 59, 59).unwrap()));
        assert!(range.contains(&NaiveTime::from_hms_opt(5, 0, 0).unwrap()));
        assert!(range.contains(&NaiveTime::from_hms_opt(6, 0, 0).unwrap()));
        assert!(range.contains(&NaiveTime::from_hms_opt(6, 59, 59).unwrap()));
        assert!(!range.contains(&NaiveTime::from_hms_opt(7, 0, 0).unwrap()));
        assert!(!range.contains(&NaiveTime::from_hms_opt(23, 59, 59).unwrap()));
    }

    #[test]
    fn test_inverted() {
        let t1 = NaiveTime::from_hms_opt(7, 0, 0).unwrap();
        let t2 = NaiveTime::from_hms_opt(5, 0, 0).unwrap();
        let range = TimeRange::new(t1, t2);

        assert!(range.contains(&NaiveTime::from_hms_opt(0, 0, 0).unwrap()));
        assert!(range.contains(&NaiveTime::from_hms_opt(4, 59, 59).unwrap()));
        assert!(!range.contains(&NaiveTime::from_hms_opt(5, 0, 0).unwrap()));
        assert!(!range.contains(&NaiveTime::from_hms_opt(6, 0, 0).unwrap()));
        assert!(!range.contains(&NaiveTime::from_hms_opt(6, 59, 59).unwrap()));
        assert!(range.contains(&NaiveTime::from_hms_opt(7, 0, 0).unwrap()));
        assert!(range.contains(&NaiveTime::from_hms_opt(23, 59, 59).unwrap()));
    }

    #[test]
    fn test_local() {
        let t1 = NaiveTime::from_hms_opt(0, 0, 0).unwrap();
        let t2 = NaiveTime::from_hms_opt(0, 10, 0).unwrap();
        let range = TimeRange::new(t1, t2);

        let string = "2024-07-19 00:08:59.673786430";
        let naive: NaiveDateTime =
            NaiveDateTime::parse_from_str(string, "%Y-%m-%d %H:%M:%S%.f").unwrap();
        let local: DateTime<Local> = naive.and_local_timezone(Local).unwrap();

        assert!(range.contains_local(&local));
    }
}
