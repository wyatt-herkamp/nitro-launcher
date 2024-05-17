use chrono::Duration;
use serde::{de::Visitor, Deserializer, Serializer};

pub fn serialize<S>(date: &Duration, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_i64(date.num_seconds())
}
macro_rules! number {
    ($fn_name:ident, $t:ty) => {
        fn $fn_name<E>(self, v: $t) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            let v_as_i64 = v as i64;
            let max = i64::MAX;
            let min = 0;
            if (v_as_i64 < min || v_as_i64 > max) {
                return Err(E::custom::<String>(format!(
                    "Must be within Range of {} to {}",
                    min, max
                )));
            }
            Ok(Duration::seconds(v_as_i64))
        }
    };
}
struct DurationVisitor;
impl<'de> Visitor<'de> for DurationVisitor {
    type Value = Duration;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "An Integer")
    }
    number!(visit_i64, i64);
    number!(visit_u64, u64);
    number!(visit_i32, i32);
}
pub fn deserialize<'de, D>(deserializer: D) -> Result<Duration, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_i64(DurationVisitor)
}
