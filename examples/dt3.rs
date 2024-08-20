use chrono::{DateTime, Local, NaiveDateTime, Utc};

fn main() {
    let now = "2024-07-18 22:08:59.673786430";
    let now: NaiveDateTime = NaiveDateTime::parse_from_str(now, "%Y-%m-%d %H:%M:%S%.f").unwrap();
    let now_utc: DateTime<Utc> = DateTime::from_naive_utc_and_offset(now, Utc);
    println!("now_utc: {:?}", now_utc);

    let now = "2024-07-19 00:08:59.673786430";
    let now: NaiveDateTime = NaiveDateTime::parse_from_str(now, "%Y-%m-%d %H:%M:%S%.f").unwrap();
    let now_local: DateTime<Local> = now.and_local_timezone(Local).unwrap();
    println!("now_local: {:?}", now_local);

    let naive_time = now.time();
    println!("naive_local_time: {:?}", naive_time);

    let diff = now_utc.signed_duration_since(now_local);
    println!("diff: {:?}", diff);
}
