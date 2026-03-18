pub trait JsonSerialize {
    fn to_json(&self) -> String;
}
