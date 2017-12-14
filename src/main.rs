extern crate clap;
extern crate regex;

use std::error::Error;
use std::process::exit;
use regex::Regex;
use std::fs;
use std::io;

use clap::{App, Arg, ArgMatches};

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
        if repo_path.file_name().eq(".git") {
            let repo_path_path = repo_path.path();
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

        let file_type = repo_path.file_type().unwrap();
        if file_type.is_dir() {
            dir_paths.push(repo_path.path().into_os_string().into_string().unwrap());
        }
    }
    Ok(dir_paths)
}


fn get_args<'a>() -> ArgMatches<'a> {
    App::new("find-repos")
        .version("1.1")
        .about("Find git repos")
        .author("Kyle D. <kdeal@kyledeal.com>")
        .arg(Arg::with_name("base_path")
                 .default_value("./")
                 .help("If file path should be printed"))
        .arg(Arg::with_name("full_path")
                 .help("If file path should be printed")
                 .long("full-path")
                 .short("p"))
        .arg(Arg::with_name("filter")
                .help("Filter the list by string")
                .takes_value(true)
                .long("filter")
                .short("f"))
        .arg(Arg::with_name("no_short_circuit")
                 .help("If subdirectories of a git repo should be explored")
                 .long("full-search")
                 .short("s"))
        .get_matches()
}


fn main() {
    let args = get_args();
    let regex = match args.value_of("filter") {
            Some(filter) => Regex::new(filter),
            None => Regex::new(r""),
    };
    if let Err(err) = regex {
        eprintln!("{}", err.description());
        exit(1);
    }
    let regex = regex.unwrap();

    let options = Options {
        full_path: args.is_present("full_path"),
        short_circuit: !args.is_present("no_short_circuit"),
        base_path: args.value_of("base_path").unwrap(),
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
