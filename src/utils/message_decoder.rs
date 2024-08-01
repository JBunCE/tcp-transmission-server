use serde::Deserialize;

pub fn from_utf8(data: &[u8]) -> &str {
    std::str::from_utf8(data).unwrap()
}

pub fn decode_message<'a, T>(data: &'a str) -> T
where
    T: Deserialize<'a>,
{
    serde_json::from_str::<T>(data).unwrap()
}