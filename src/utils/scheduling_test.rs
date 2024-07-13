use chrono::{DateTime, Duration, Local, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc};

use super::Scheduling as Sched;

fn get_now() -> DateTime<Utc> {
    Utc::now()
}

#[test]
fn test_unscheduled() {
    let now = get_now();
    let (since, until) = (now.clone(), now.clone());

    let s = Sched::Unscheduled;
    assert!(s.is_unescheduled());
    assert_eq!(s.time_to_next(&now), None);
    assert!(s.occurences(&since, &until).is_empty());
    assert_eq!(s.into_next(), Sched::Unscheduled);
}

#[test]
fn test_immediate() {
    let now = get_now();
    let (since, until) = (now.clone(), now.clone());

    let s = Sched::Immediate;
    assert!(!s.is_unescheduled());
    assert_eq!(s.time_to_next(&now), Some(Duration::zero()));
    assert_eq!(s.occurences(&since, &until), vec![since]);
    assert_eq!(s.into_next(), Sched::Unscheduled);
}

#[test]
fn test_once_at() {
    let now = get_now();
    let (since, until) = (now.clone(), now.clone() + Duration::days(1));
    let schedule_time = since + Duration::hours(1);

    let s = Sched::OnceAt(schedule_time);
    assert!(!s.is_unescheduled());
    assert_eq!(s.time_to_next(&now), Some(Duration::hours(1)));
    assert_eq!(s.occurences(&since, &until), vec![schedule_time]);
    assert_eq!(s.into_next(), Sched::Unscheduled);
}

#[test]
fn test_once_in() {
    let now = get_now();
    let (since, until) = (now.clone(), now.clone() + Duration::days(1));
    let duration = Duration::hours(2);

    let s = Sched::OnceIn(duration);
    assert!(!s.is_unescheduled());
    assert_eq!(s.time_to_next(&now), Some(duration));
    assert_eq!(s.occurences(&since, &until), vec![since + duration]);
    assert_eq!(s.into_next(), Sched::Unscheduled);
}

#[test]
fn test_daily() {
    // Naive local to Utc
    fn nltu(d: NaiveDate, t: NaiveTime) -> DateTime<Utc> {
        Local
            .from_local_datetime(&NaiveDateTime::new(d, t))
            .single()
            .unwrap()
            .into()
    }

    let d1 = NaiveDate::from_ymd_opt(2024, 7, 13).unwrap();
    let d2 = NaiveDate::from_ymd_opt(2024, 7, 15).unwrap();
    let t1 = NaiveTime::from_hms_opt(19, 0, 0).unwrap();
    let t2 = NaiveTime::from_hms_opt(21, 0, 0).unwrap();

    let tref = NaiveTime::from_hms_opt(20, 0, 0).unwrap();

    let s = Sched::Daily(tref);

    assert!(!s.is_unescheduled());
    assert_eq!(s.into_next(), s);
    assert_eq!(s.time_to_next(&nltu(d1, t1)), Some(Duration::hours(1)));
    assert_eq!(s.time_to_next(&nltu(d1, t2)), Some(Duration::hours(23)));
    assert_eq!(s.time_to_next(&nltu(d2, t1)), Some(Duration::hours(1)));
    assert_eq!(s.time_to_next(&nltu(d2, t2)), Some(Duration::hours(23)));

    assert_eq!(
        s.occurences(&nltu(d1, t1), &nltu(d1, t2)),
        vec![nltu(d1, tref)]
    );
    assert_eq!(
        s.occurences(&nltu(d1, t1), &nltu(d2, t1)),
        vec![nltu(d1, tref), nltu(d1 + Duration::days(1), tref)]
    );
    assert_eq!(
        s.occurences(&nltu(d1, t1), &nltu(d2, t2)),
        vec![
            nltu(d1, tref),
            nltu(d1 + Duration::days(1), tref),
            nltu(d1 + Duration::days(2), tref)
        ]
    );
    assert_eq!(
        s.occurences(&nltu(d1, t2), &nltu(d2, t1)),
        vec![nltu(d1 + Duration::days(1), tref)]
    );
    assert_eq!(
        s.occurences(&nltu(d1, t2), &nltu(d2, t2)),
        vec![
            nltu(d1 + Duration::days(1), tref),
            nltu(d1 + Duration::days(2), tref)
        ]
    );
    assert_eq!(
        s.occurences(&nltu(d2, t1), &nltu(d2, t2)),
        vec![nltu(d2, tref)]
    );

    // TODO Test edge cases
}
