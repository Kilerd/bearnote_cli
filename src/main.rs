#[macro_use]
extern crate serde_derive;

use std::any::Any;
use std::fs::{File, OpenOptions};
use std::io::{ErrorKind, Read, Write};
use std::io;
use std::io::prelude::*;
use std::path::Path;
use std::process::{Command, Stdio};

use chrono::NaiveDateTime;
use clap::{App, Arg, SubCommand, AppSettings};
use colored::*;
use reqwest::Client;

use crate::store::{Note, NoteFile, PostNote, NoteResponse};
use std::collections::HashMap;
use std::rc::Rc;

use crate::store::NotePassword;

mod store;

fn main() -> Result<(), std::io::Error> {
    let note_file = dirs::home_dir().map(|path| {
        path.join(".bearnote.toml")
    }).expect("not supported platform");

    if !note_file.exists() {
        let mut file = File::create(note_file.as_os_str())?;
        file.write_all(b"notes=[]");
    }
    let mut app = App::new("Bearnote")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("list")
                .about("list all local stored notes")
                .version("0.1.0")
                .author(env!("CARGO_PKG_AUTHORS"))
                .arg(
                    Arg::with_name("short")
                        .short("s")
                        .long("short")
                        .help("show each note in one line")
                )
        )
        .subcommand(
            SubCommand::with_name("add")
                .about("add a new note")
                .version("0.1.0")
                .author(env!("CARGO_PKG_AUTHORS"))
                .arg(
                    Arg::with_name("FILE")
                        .help("read note's content from the file")
                        .required(true)
                        .index(1)
                )
        )
        .subcommand(
            SubCommand::with_name("delete")
                .about("delete note")
                .version("0.1.0")
                .author(env!("CARGO_PKG_AUTHORS"))
                .arg(
                    Arg::with_name("ID")
                        .help("note's id need to delete. the unique prefix id is acceptable")
                        .required(true)
                        .index(1)
                )
        )
        ;
    let matches = app.clone().get_matches();


    if let Some(sub_command_matches) = matches.subcommand_matches("list") {
        let is_short = sub_command_matches.occurrences_of("short") > 0;
        let mut result = File::open(note_file.as_os_str())?;
        let mut string = String::new();
        let _i = result.read_to_string(&mut string)?;
        let mut notes: NoteFile = toml::from_str(&string).expect("error format");

        if is_short {
            println!("ID                                      TAGS");
        }

        for note in notes.notes {
            let tags = note.tag
                .unwrap_or(vec![])
                .iter()
                .map(|tag| tag.blue().to_string())
                .collect::<Vec<String>>()
                .join(", ");
            if is_short {
                println!("{}    {}", note.id.yellow().bold(), tags);
            } else {
                println!("ID:        {}", note.id.yellow().bold());
                if !tags.eq("".into()) {
                    println!("Tags:      {}", tags);
                }
                println!("Password:  {}", note.password.cyan());
                println!("Create At: {}", note.time);
                println!("Link:      {}\n", format!("https://www.bearnote.com/n/{}", note.id).white().underline());
            }
        }
    } else if let Some(sub_command_matches) = matches.subcommand_matches("add") {
        let file_name = sub_command_matches.value_of("FILE").expect("FILE is required");

        let file_path = Path::new(file_name);
        if !file_path.is_file() || !file_path.exists() {
            panic!("{} is not file or does not exist", file_name);
        }

        let extension = file_path.extension().unwrap_or("text".as_ref());
        let mut file = File::open(file_path)?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer);

        let post_note = PostNote {
            content: buffer,
            extension: extension.to_str().map(String::from),
        };
        let client = Client::new();
        let mut response = client.post("https://www.bearnote.com/n")
            .json(&post_note)
            .send()
            .expect("request error");

        let result1 = response.json::<NoteResponse>().expect("error on derialize note response");
        let string1 = result1.id.clone();
        save_note_to_file(result1.into());
        println!("save as {}", string1.yellow().bold());
    } else if let Some(sub_command_matches) = matches.subcommand_matches("delete") {
        let mut result = File::open(note_file.as_os_str())?;
        let mut string = String::new();
        let _i = result.read_to_string(&mut string)?;
        let mut notes: NoteFile = toml::from_str(&string).expect("error format");

        let id = sub_command_matches.value_of("ID").expect("note's ID is required");

        let notes_with_prefix_id: Vec<&Note> = notes.notes.iter().filter(|note| note.id.starts_with(id)).collect();

        match notes_with_prefix_id.len() {
            0 => {
                println!("Note not found");
            }
            1 => {
                let index = notes.notes.iter().position(|note| note.id.starts_with(id)).expect("error on getting index");

                let note1 = notes.notes.remove(index);

                let string2 = format!("https://www.bearnote.com/n/{}", note1.id);
                let result3 = Client::new()
                    .delete(string2.as_str())
                    .json(&NotePassword::from_str(note1.password))
                    .send();

                let result2 = toml::to_string(&notes).expect("error format");

                File::create(note_file)
                    .expect("cannot open file")
                    .write_all(result2.as_bytes())
                    .expect("error on write notes");

                println!("deleted id {}", note1.id.yellow());
            }
            _ => {
                println!("{}", "cannot found the unique id, please input more precise id".red());
            }
        }
    }
    Ok(())
}

fn save_note_to_file(note: Note) -> Result<(), io::Error> {
    let note_file = dirs::home_dir().map(|path| {
        path.join(".bearnote.toml")
    }).expect("not supported platform");

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(false)
        .append(true)
        .open(note_file)?;

    let mut map: HashMap<String, Vec<Note>> = HashMap::new();
    let append_note_list = vec![note];

    map.insert(String::from("notes"), append_note_list);
    let string = toml::to_string(&map).expect("error on serialize note");
    file.write_all(string.as_bytes());
    Ok(())
}
