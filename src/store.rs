use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct NoteFile {
    pub notes: Vec<Note>
}




#[derive(Serialize, Deserialize, Debug)]
pub struct Note {
    pub id: String,
    pub password: String,
    pub tag: Option<Vec<String>>,
    pub time: NaiveDateTime,
    pub extension: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PostNote {
    pub content: String,
    pub extension: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NoteResponse {
    pub id: String,
    pub content: String,
    pub password: String,
    pub tag: Option<Vec<String>>,
    pub create_at: NaiveDateTime,
    pub view: i64,
    pub extension: String,
    pub is_delete: bool
}

impl From<NoteResponse> for Note {
    fn from(response: NoteResponse) -> Self {
        Self {
            id: response.id,
            password: response.password,
            tag: Some(vec![]),
            time: response.create_at,
            extension: response.extension
        }
    }
}