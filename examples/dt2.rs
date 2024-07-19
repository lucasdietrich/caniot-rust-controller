use caniot_controller::utils::Scheduling;
use chrono::{DateTime, NaiveDateTime, NaiveTime, Utc};

fn main() {
    let now = "2024-07-18 22:08:59.673786430";
    let now: NaiveDateTime = NaiveDateTime::parse_from_str(now, "%Y-%m-%d %H:%M:%S%.f").unwrap();
    let now_utc: DateTime<Utc> = DateTime::from_naive_utc_and_offset(now, Utc);
    println!("{:?}", now_utc);

    let scheduling = Scheduling::Daily(NaiveTime::from_hms_opt(0, 2, 0).unwrap());
    println!("{:?}", scheduling);

    let time_to_next = scheduling.time_to_next(&now_utc);
    println!("{:?}", time_to_next);
}
