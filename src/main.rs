use std::env;
use std::fs;
use std::process::Command;
use chrono::prelude::*;
use clap::{Arg, Command as ClapCommand};

fn main() {
    let home = env::var("HOME").expect("Failed to get HOME directory");

    // notes files
    let notes = format!("{}/notes.txt", home);
    let notes_personal = format!("{}/notes-personal.txt", home);

    // Parse command line arguments using clap
    let matches = ClapCommand::new("notes")
        .arg(Arg::new("private")
            .short('p')
            .long("private")
            .help("Open non-work notes")
            .action(clap::ArgAction::SetTrue))
        .arg(Arg::new("clear")
            .short('c')
            .long("clear")
            .help("Insert 50 newlines to not show notes on startup")
            .action(clap::ArgAction::SetTrue))
        .arg(Arg::new("vscode")
            .short('v')
            .long("vscode")
            .help("Use VSCode")
            .action(clap::ArgAction::SetTrue))
        .get_matches();

    // default editor is nvim
    let editor = if matches.get_flag("vscode") {
        "code"
    } else {
        "nvim"
    };

    let file_path = if matches.get_flag("private") {
        write_header(&notes_personal, matches.get_flag("clear"));
        notes_personal
    } else {
        write_header(&notes, matches.get_flag("clear"));
        notes
    };

    let mut cmd = Command::new(editor)
        .arg(&file_path)
        .spawn()
        .expect("Failed to start editor");

    cmd.wait().expect("Failed to wait for editor");
}

// writeHeader writes the date to the top of the notes file. clear
// puts a bunch of newlines to not show notes on a video call.
fn write_header(file: &str, clear: bool) {
    let data = fs::read_to_string(file)
        .expect("Failed to read file");

    // build header in the Format
    // 01/02/2006
    // ----------
    let now: DateTime<Local> = Local::now();
    let today = now.format("%m/%d/%Y").to_string();

    let header = if data.starts_with(&today) {
        data.lines().next().unwrap().to_string()
    } else {
        today.clone()
    };

    // if clear is set, add 50 newlines to hide current
    // buffer when sharing screen
    let newlines = "\n".repeat(50);

    let new_content = if clear {
        if data.starts_with(&today) {
            // dump 50 newlines after the date header
            let rest_of_file = &data[header.len() + 12..];
            format!("{}\n----------\n{}{}", header, newlines, rest_of_file)
        } else {
            // write the header + lots of newlines
            format!("{}\n----------\n{}{}", header, newlines, data)
        }
    } else if !data.starts_with(&today) {
        // just write today's date at the top if it's not there
        format!("{}\n----------\n\n\n{}", header, data)
    } else {
        // no changes needed
        return;
    };

    fs::write(file, new_content).expect("Failed to write file");
}
