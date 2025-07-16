#[derive(serde::Serialize, PartialEq, serde::Deserialize)]
pub struct Media {
    pub picture: Option<String>,
    pub sound: Option<String>,
}
