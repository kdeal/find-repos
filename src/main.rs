extern crate clap;
extern crate regex;

use regex::Regex;
use std::fs;
use std::io;
use std::process::exit;

use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    // If file path should be printed
    #[arg(default_value = "./")]
    base_path: String,
    // If file path should be printed
    #[arg(short = 'p', long)]
    full_path: bool,
    // Filter the list by string
    #[arg(short, long)]
    filter: Option<String>,
    // If subdirectories of a git repo should be explored
    #[arg(short = 's', long = "full-search")]
    no_short_circuit: bool,
}

struct Options<'a> {
    full_path: bool,
    short_circuit: bool,
    base_path: &'a str,
    filter: Regex,
}

fn check_dir(dir: String, options: &Options) -> io::Result<Vec<String>> {
    let mut dir_paths: Vec<String> = vec![];
    for path in fs::read_dir(dir)? {
        if path.is_err() {
            continue;
        }

        let repo_path = path.unwrap();
        let repo_path_path = repo_path.path();
        if repo_path.file_name().eq(".git") {
            let parent_path = repo_path_path.parent().unwrap();
            let short_path = parent_path.strip_prefix(options.base_path).unwrap();
            if !options.filter.is_match(short_path.to_str().unwrap()) {
                continue;
            }

            if options.full_path {
                println!("{}", parent_path.to_str().unwrap());
            } else {
                println!("{}", short_path.to_str().unwrap());
            }

            if options.short_circuit {
                return Ok(vec![]);
            }
        }

        let mut file_type = repo_path.file_type().unwrap();
        if file_type.is_symlink() {
            let symlink_path = fs::read_link(&repo_path_path).unwrap();
            let metadata_result = fs::symlink_metadata(&symlink_path);
            if metadata_result.is_err() {
                continue;
            }

            let file_metadata = metadata_result.unwrap();
            file_type = file_metadata.file_type();
        }

        if file_type.is_dir() {
            dir_paths.push(repo_path_path.into_os_string().into_string().unwrap());
        }
    }
    Ok(dir_paths)
}

fn main() {
    let args = Cli::parse();
    let regex = match args.filter.as_deref() {
        Some(filter) => Regex::new(filter),
        None => Regex::new(r""),
    };
    if let Err(err) = regex {
        eprintln!("{}", err);
        exit(1);
    }
    let regex = regex.unwrap();

    let options = Options {
        full_path: args.full_path,
        short_circuit: !args.no_short_circuit,
        base_path: args.base_path.as_ref(),
        filter: regex,
    };

    let mut dirs_to_read: Vec<String> = vec![String::from(options.base_path)];

    while !dirs_to_read.is_empty() {
        let path = dirs_to_read.pop().unwrap();
        let dir_paths = check_dir(path, &options);
        if dir_paths.is_ok() {
            dirs_to_read.append(&mut dir_paths.unwrap());
        }
    }
}
