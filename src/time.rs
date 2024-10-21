use chrono::*;

pub struct UTC8(DateTime<FixedOffset>);

impl From<DateTime<Utc>> for UTC8 {
    fn from(value: DateTime<Utc>) -> Self {
        UTC8(value.with_timezone(&FixedOffset::east_opt(8 * 3600).unwrap()))
    }
}

#[test]
fn utc_to_beijing() {
    let unix_epoch = DateTime::<Utc>::UNIX_EPOCH;
    let unix_epoch_utc8 = UTC8::from(unix_epoch);
    println!(
        "unix_epoch={unix_epoch}\nunix_epoch_utc8={}",
        unix_epoch_utc8.0
    );
}
