use std::path::Path;
use std::fs;
use std::io;
use std::env;


fn check_dir(dir: String, repo_root: &str) -> io::Result<Vec<String>> {
    let mut dir_paths: Vec<String> = vec![];
    for path in fs::read_dir(dir)? {
        if path.is_err() {
            continue;
        }

        let repo_path = path.unwrap();
        if repo_path.file_name().eq(".git") {
            let repo_path_path = repo_path.path();
            let parent_path = repo_path_path.parent().unwrap();
            let striped: &Path = parent_path.strip_prefix(repo_root).unwrap();
            println!("{}", striped.to_str().unwrap());
            return Ok(vec![]);
        }

        let file_type = repo_path.file_type().unwrap();
        if file_type.is_dir() {
            dir_paths.push(repo_path.path().into_os_string().into_string().unwrap());
        }
    }
    Ok(dir_paths)
}

fn main() {
    let repo_root = match env::args().nth(1) {
        Some(path) => path,
        None => format!("{}/repos", env::home_dir().unwrap().to_str().unwrap()),
    };
    let mut dirs_to_read: Vec<String> = vec![repo_root.clone()];
    let repo_root = repo_root.as_str();

    while !dirs_to_read.is_empty() {
        let path = dirs_to_read.pop().unwrap();
        let dir_paths = check_dir(path, repo_root);
        if dir_paths.is_ok() {
            dirs_to_read.append(&mut dir_paths.unwrap());
        }
    }
}
