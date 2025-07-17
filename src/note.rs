use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Note {
    pub name: String,
    pub text: String,
    //media: Media,
}
