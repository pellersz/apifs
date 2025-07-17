#[derive(serde::Serialize, PartialEq, serde::Deserialize, Clone)]
pub struct Media {
    pub picture: Option<String>,
    pub sound: Option<String>,
}
