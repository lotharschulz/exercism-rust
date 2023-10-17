use time::Duration;
use time::PrimitiveDateTime as DateTime;

pub fn after(start: DateTime) -> DateTime {
    start + Duration::seconds(1000000000) // note: a trailing semicolon would cause the test to fail 
}
