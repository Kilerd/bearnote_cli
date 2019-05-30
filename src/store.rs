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
    pub time: NaiveDateTime
}