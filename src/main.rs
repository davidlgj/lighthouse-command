use std::io::{self, /* Write, */ BufRead};
use std::fs;

const FOLDER: &str = "/home/david/Projekt/components";

enum Command {
    Gitlab,
    Terminal,
    Atom,
}



fn read_components() -> std::io::Result<Vec<String>> {
    let mut folders = Vec::with_capacity(128);
    for entry in fs::read_dir(FOLDER)? {
        let dir = entry?;
        // writeln!(io::stderr(), "{:?}", dir.file_name());
        folders.push(dir.file_name().into_string().unwrap())
    }
    folders.sort();
    Ok(folders)
}


fn main() -> io::Result<()> {
    let folders = read_components().unwrap();
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut command;

    for line in handle.lines() {
        // writeln!(std::io::stderr(), "{:?}", line);
        match line {
            Ok(needle) => {
                let parsed;
                if needle.starts_with("g ") {
                    parsed = String::from(&needle[2..]);
                    command = Command::Gitlab;
                } else if needle.starts_with("t ") || needle.starts_with("a ") {
                    parsed = String::from(&needle[2..]);
                    command = Command::Terminal;
                } else {
                    parsed = needle;
                    command = Command::Atom;
                }
                if parsed.len() == 0 {
                    continue;
                }

                let tws_needle = String::from("tws-") + &parsed;
                for folder in &folders {
                    if folder.starts_with(&parsed) || folder.starts_with(&tws_needle) {
                        match command {
                            Command::Gitlab => {
                                if folder.starts_with("tws-") {
                                    print!("{{ {} | xdg-open https://gitlab.textalk.com/webshop/diversity/native-components/{} }} ", folder, folder);
                                    // writeln!(io::stderr(),"{{ {} | xdg-open https://gitlab.textalk.com/webshop/diversity/native-components/{} }} ", folder, folder);
                                } else {
                                    // Probably a theme
                                    print!("{{ {} | xdg-open https://gitlab.textalk.com/webshop/diversity/themes/{} }}  ", folder, folder);
                                    // writeln!(io::stderr(),"{{ {} | xdg-open https://gitlab.textalk.com/webshop/diversity/themes/{} }}  ", folder, folder);
                                }
                            },
                            Command::Terminal => print!("{{ {} | alacritty --working-directory  {}/{} }}", folder, FOLDER, folder),
                            _ => print!("{{ {} | atom {}/{} }}", folder, FOLDER, folder)
                        }
                    }
                }
                println!("");
            },
            _ => ()
        }
    }

    Ok(())
}
