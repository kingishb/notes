use chrono::prelude::*;
use clap::{Parser, ValueEnum};
use std::env;
use std::fs;
use std::process::Command;


#[derive(ValueEnum, Debug, PartialEq, Clone)]
#[clap(rename_all="kebab_case")]
enum Editor {
    Vim,
    Cursor,
    VSCode
}


/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Clear the visual screen by inserting 50 newlines on the newest note.
    #[arg(short, long)]
    clear: bool,

    /// Editor to use
    #[arg(value_enum)]
    editor: Editor,
}

fn main() {
    let cli = Cli::parse();
    let home = env::var("HOME").expect("Failed to get HOME directory");

    // notes files
    let notes = format!("{}/notes.txt", home);

    let editor = match cli.editor {
        Editor::Vim => "nvim",
        Editor::Cursor => "cursor",
        Editor::VSCode => "code"
    };

    let file_path = {
        write_header(&notes, cli.clear);
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
    let data = fs::read_to_string(file).expect("Failed to read file");

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

    // if clear is set, add 50 newlines to hide curren buffer when sharing screen
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
