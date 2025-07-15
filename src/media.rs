#[derive(serde::Serialize)]
pub struct Media {
    pub picture: Option<String>,
    pub sound: Option<String>,
}
