use std::{env, fs};
use std::borrow::Borrow;
use std::env::{home_dir, set_current_dir};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::PathBuf;
use std::process;
use std::process::Command;
use std::io::{self, BufRead};
use std::path::Path;
use std::ptr::null;

extern crate dirs;

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("Args length: {}", args.len());

    if args.len() != 2 {
        eprintln!("Expected 1 argument");
        std::process::exit(69);
    }

    let phrase = &args[1];
    println!("phrase is: {}", phrase);

    let filepath = dirs::home_dir().unwrap().display().to_string() + "/SavedDir.txt";

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(dirs::home_dir().unwrap().display().to_string() + "/SavedDir.txt")
        .unwrap();

    if Exists(&filepath, phrase.to_string()) { ParseFile(filepath, phrase.to_string()) }
    //else { BookMark(phrase, &file) }
}

fn BookMark(mut name: String, mut filepath: &File) {
    name.push_str(" ");
    name.push_str(env::current_dir().unwrap().display().to_string().as_ref());
    name.push_str("\n");
    filepath.write(name.as_ref());
}

fn Exists(filepath: &String, phrase: String) -> bool {
    let contents = fs::read_to_string(filepath);
    let exists;

    if contents.unwrap().to_string().contains(&phrase) { exists = true; }
    else { exists = false; }

    return exists;
}

fn ParseFile(mut filepath: String, phrase: String) {
    let mut lineToUse = String::from("");

    if let Ok(lines) = ReadLines(filepath) {
        for line in lines {
            if let Ok(lineContents) = line {
                println!("{}", lineContents);
                if(lineContents.contains(&phrase)){
                    lineToUse = String::from(lineContents);
                    break;
                }
            }
        }
    }

    lineToUse.replace(&phrase, "");
    let path = lineToUse.replace(&phrase, "");
    println!("Path: {}", path);
    //RunCommand(path);
    //assert!(env::set_current_dir(&path).is_ok());
    //RunCommand("command.sh".to_string());
    RunCdCommand(path);
}

fn ReadLines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn RunCommand(path: String) {
    let mut program = String::from("cd");
    program.push_str(&*path);
    println!("Command: {}", program);
    Command::new(program).spawn().expect("Error running command");
}

fn RunCdCommand(path: String) {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(dirs::home_dir().unwrap().display().to_string() + "/RunCdCommand.bash")
        .unwrap();

    let mut program = String::new();
    program.push_str("cd");
    program.push_str(&path);

    file.write(&program.as_ref());
    //Command::new("sleep").arg(".25").spawn().unwrap();
    let mut command = "source ".to_string();
    command.push_str(&*dirs::home_dir().unwrap().display().to_string().to_string());
    command.push_str("RunCdCommand.bash");
    println!("Running command: {}", command);
    Command::new(command);
}
