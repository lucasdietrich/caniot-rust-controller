use std::str::FromStr;

use chrono::{Datelike, Duration, NaiveDate, NaiveDateTime, NaiveTime, Utc};

use super::Scheduling as Sched;

fn get_now() -> NaiveDateTime {
    Utc::now().naive_utc()
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
    let d1 = NaiveDate::from_ymd_opt(2024, 7, 13).unwrap();
    let d2 = NaiveDate::from_ymd_opt(2024, 7, 15).unwrap();
    let t1 = NaiveTime::from_hms_opt(19, 0, 0).unwrap();
    let t2 = NaiveTime::from_hms_opt(21, 0, 0).unwrap();

    let tref = NaiveTime::from_hms_opt(20, 0, 0).unwrap();

    let s = Sched::Daily(tref);

    assert!(!s.is_unescheduled());
    assert_eq!(s.into_next(), s);
    assert_eq!(
        s.time_to_next(&NaiveDateTime::new(d1, t1)),
        Some(Duration::hours(1))
    );
    assert_eq!(
        s.time_to_next(&NaiveDateTime::new(d1, t2)),
        Some(Duration::hours(23))
    );
    assert_eq!(
        s.time_to_next(&NaiveDateTime::new(d2, t1)),
        Some(Duration::hours(1))
    );
    assert_eq!(
        s.time_to_next(&NaiveDateTime::new(d2, t2)),
        Some(Duration::hours(23))
    );

    assert_eq!(
        s.occurences(&NaiveDateTime::new(d1, t1), &NaiveDateTime::new(d1, t2)),
        vec![NaiveDateTime::new(d1, tref)]
    );
    assert_eq!(
        s.occurences(&NaiveDateTime::new(d1, t1), &NaiveDateTime::new(d2, t1)),
        vec![
            NaiveDateTime::new(d1, tref),
            NaiveDateTime::new(d1 + Duration::days(1), tref)
        ]
    );
    assert_eq!(
        s.occurences(&NaiveDateTime::new(d1, t1), &NaiveDateTime::new(d2, t2)),
        vec![
            NaiveDateTime::new(d1, tref),
            NaiveDateTime::new(d1 + Duration::days(1), tref),
            NaiveDateTime::new(d1 + Duration::days(2), tref)
        ]
    );
    assert_eq!(
        s.occurences(&NaiveDateTime::new(d1, t2), &NaiveDateTime::new(d2, t1)),
        vec![NaiveDateTime::new(d1 + Duration::days(1), tref)]
    );
    assert_eq!(
        s.occurences(&NaiveDateTime::new(d1, t2), &NaiveDateTime::new(d2, t2)),
        vec![
            NaiveDateTime::new(d1 + Duration::days(1), tref),
            NaiveDateTime::new(d1 + Duration::days(2), tref)
        ]
    );
    assert_eq!(
        s.occurences(&NaiveDateTime::new(d2, t1), &NaiveDateTime::new(d2, t2)),
        vec![NaiveDateTime::new(d2, tref)]
    );
}
