use serde::de::Deserialize;
use serde::ser::Serialize;

pub trait CacheAble<T> {
    fn deserialize<'de>(cache_string: &'de str) -> Self
    where
        Self: Deserialize<'de>,
    {
        let res: Self = serde_json::from_str(cache_string)
            .expect("Could not deserialize string into CashedUniversity");
        res
    }

    fn serialize(&self) -> String
    where
        Self: Serialize,
    {
        serde_json::to_string(self).expect("Could not serialize CachedUniversity")
    }

    fn from_base(base: T) -> Self;
}
