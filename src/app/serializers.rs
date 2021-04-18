use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub fn serialize_naive_datetime<S>(dt: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    DateTime::<Utc>::from_utc(dt.clone(), Utc)
        .to_rfc3339()
        .serialize(serializer)
}

pub fn serialize_optional_naive_datetime<S>(
    dt: &Option<NaiveDateTime>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match dt {
        Some(dt) => DateTime::<Utc>::from_utc(dt.clone(), Utc)
            .to_rfc3339()
            .serialize(serializer),
        None => serializer.serialize_none(),
    }
}

pub fn deserialize_optional_naive_datetime<'de, D>(
    deserializer: D,
) -> Result<Option<NaiveDateTime>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Option::<String>::deserialize(deserializer)?;

    match value {
        Some(s) => DateTime::parse_from_rfc3339(s.as_str())
            .map(|v| Some(v.naive_local()))
            .map_err(serde::de::Error::custom),
        None => Ok(None),
    }
}
