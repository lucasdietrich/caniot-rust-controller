use chrono::{Local, NaiveDateTime, Utc};

fn main() {
    let local = Local::now();
    let tz = local.timezone();
    println!("Timezone: {:?}", tz);
    let utc = Utc::now();
    let naive_utc = Utc::now().naive_utc();
    let naive_local = Utc::now().naive_local();

    println!("Local: {}", local);
    println!("UTC: {}", utc);
    println!("Naive UTC: {}", naive_utc);
    println!("Naive Local: {}", naive_local);

    // Convert DateTime<Local> to DateTime<Utc>
    let utc_from_local = local.with_timezone(&Utc);
    println!("UTC from Local: {}", utc_from_local);

    // Time local
    let time_local = local.time();
}
