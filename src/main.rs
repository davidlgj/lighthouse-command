use std::io::{self, Write, BufRead};
use std::fs;
use std::env;

extern crate regex;
use regex::Regex;


#[derive(Debug)]
struct Settings {
    pub folder: Option<String>,
    pub keys: Vec<String>,
    pub commands: Vec<String>,
}


impl Settings {
    pub fn parse_commandline(args: Vec<String>) -> Settings {
        let mut commands = Vec::with_capacity(args.len());
        let mut keys = Vec::with_capacity(args.len());
        let mut folder = None;
        let mut skip = true;
        for arg in args {
            if arg.ends_with("component-launcher") {
                skip = false;
                continue;
            }
            if skip {
                continue;
            }

            // folder option, defaults to .
            if arg.starts_with("--folder=") {
                let mut split = arg.split('=');
                if let Some(_) = split.next() {
                    if let Some(p) = split.next() {
                        folder = Some(String::from(p));
                    }
                }
            } else {
                // All other args should follow the format
                // "<char>|<command>"
                if arg.contains('|') {
                    let mut split = arg.split('|');
                    if let Some(key) = split.next() {
                        if let Some(command) = split.next() {
                            keys.push(String::from(key.trim_start_matches('"')));
                            commands.push(String::from(command.trim_end_matches('"')));
                        }
                    }
                } else {
                    // ...maybe except one, it gets defaulted to '*'
                    keys.push(String::from("*"));
                    commands.push(String::from(arg.trim_matches('"')));
                }
            }
        }

        Settings {
            folder: folder,
            keys: keys,
            commands: commands
        }
    }
}


fn read_components(folder: &str) -> std::io::Result<Vec<String>> {
    let mut folders = Vec::with_capacity(128);
    for entry in fs::read_dir(folder)? {
        let dir = entry?;
        folders.push(dir.file_name().into_string().unwrap())
    }
    folders.sort();
    Ok(folders)
}




fn main() -> io::Result<()> {
    let args: Vec<_> = env::args().collect();

    if args.len() < 1 {
        println!("To few arguments. Someday I'll write a usage.");
        std::process::exit(1);
    }

    let settings = Settings::parse_commandline(args);

    let folders = match settings.folder {
        Some(f) => read_components(&f).unwrap(),
        None => read_components(".").unwrap(),
    };

    let re_start = Regex::new(r"(?P<key>[a-z])\s(?P<search>[.a-zA-Z\-]+)").unwrap();
    let re_end = Regex::new(r"(?P<search>[.a-zA-Z\-]+)\s(?P<key>[a-z])$").unwrap();

    let stdin = io::stdin();
    let handle = stdin.lock();
    for line in handle.lines() {
        match line {
            Ok(input) => {
                let key;
                let search;
                if let Some(capture) = re_end.captures(&input) {
                    key = String::from(&capture["key"]);
                    search = String::from(&capture["search"]);
                } else if let Some(capture) = re_start.captures(&input) {
                    key = String::from(&capture["key"]);
                    search = String::from(&capture["search"]);
                } else {
                    key = String::from("*");
                    search = input;
                }

                // Find a command
                let mut command = "echo {}";
                for (i, k) in settings.keys.iter().enumerate() {
                    if k == &key {
                        command = &settings.commands[i];
                    }
                }
                // writeln!(std::io::stderr(), "{:?}", search);

                // First exact matches, and then a little more fuzzy
                for folder in &folders {
                    if folder.starts_with(&search) {
                        print!("{{ {} | {} }}", folder, command.replace("{}",folder));
                    }
                }

                // And one little more "fuzzy"
                for folder in &folders {
                    if folder.contains(&search) {
                        print!("{{ {} | {} }}", folder, command.replace("{}",folder));
                    }
                }

                println!("");
            },
            _ => ()
        }
    }

    Ok(())
}
