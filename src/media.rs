#[derive(serde::Serialize, PartialEq)]
pub struct Media {
    pub picture: Option<String>,
    pub sound: Option<String>,
}
