#[macro_use]
extern crate serde_derive;

use clap::{App, SubCommand, Arg};
use std::fs::File;
use std::io::{ErrorKind, Read, Write};
use std::path::Path;
use std::io::prelude::*;
use crate::store::{Note, NoteFile};
use chrono::NaiveDateTime;
use colored::*;

mod store;

fn main() -> Result<(), std::io::Error> {
    let note_file = dirs::home_dir().map(|path| {
        path.join(".bearnote.toml")
    }).expect("not supported platform");

    let matches = App::new("Bearnote")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
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
        .get_matches();

    if !note_file.exists() {
        let mut file = File::create(note_file.as_os_str())?;
        file.write_all(b"notes=[]");
    }

    if let Some(sub_command_matches) = matches.subcommand_matches("list") {
        let is_short = sub_command_matches.occurrences_of("short") > 0;
        let mut result = File::open(note_file.as_os_str())?;
        let mut string = String::new();
        let _i = result.read_to_string(&mut string)?;
        let mut notes: NoteFile = toml::from_str(&string).expect("error format");
        for note in notes.notes {
            let tags = note.tag
                .unwrap_or(vec![])
                .iter()
                .map(|tag| tag.blue().to_string())
                .collect::<Vec<String>>()
                .join(", ");
            if is_short {
                println!("ID                                      TAGS");
                println!("{}    {}", note.id.yellow().bold(), tags);
            }else {
                println!("ID:        {}", note.id.yellow().bold());
                if !tags.eq("".into()) {
                    println!("Tags:      {}", tags);
                }
                println!("Password:  {}", note.password.cyan());
                println!("Create At: {}", note.time);
                println!("Link:      {}\n", format!("https://www.bearnote.com/n/{}", note.id).white().underline());
            }
        }
    }
    Ok(())
}
