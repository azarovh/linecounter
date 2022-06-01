use std::io::{BufRead, BufReader};
use std::{env, fs, process};
use walkdir::WalkDir;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!(
            "Wrong number of arguments. Expected: [path, extension]; actual:{:?}",
            args
        );
        process::exit(1);
    }
    let working_dir = &args[1];
    let extension = &args[2];

    for entry in WalkDir::new(working_dir)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| {
            if let Ok(entry) = e {
                if entry
                    .file_name()
                    .to_string_lossy()
                    .ends_with(&extension[..])
                {
                    return Some(entry);
                }
            }
            return None;
        })
    {
        let input = fs::File::open(entry.path()).unwrap();
        let buffered = BufReader::new(input);

        println!(
            "{} has {} lines",
            entry.file_name().to_string_lossy(),
            buffered.lines().count()
        );
    }
}
