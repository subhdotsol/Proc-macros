pub trait JsonSerialize {
    fn to_json(&self) -> String;
}

pub trait JsonDeserialize: Sized {
    fn from_json(json: &str) -> Result<Self, String>;
}
