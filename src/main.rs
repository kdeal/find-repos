use std::path::Path;
use std::fs;
use std::env;
use std::ffi::OsString;

fn main() {
    let git_dir = OsString::from(".git");
    let repo_root = match env::args().nth(1) {
        Some(path) => path,
        None => format!("{}/repos", env::home_dir().unwrap().to_str().unwrap()),
    };
    let mut dirs_to_read: Vec<String> = vec![repo_root.clone()];
    let repo_root = repo_root.as_str();

    loop {
        let paths = dirs_to_read.pop();
        if paths.is_none() {
            break;
        }
        let paths = fs::read_dir(paths.unwrap()).expect("Failed to read directory");
        for path in paths {
            let repo_path = path.unwrap();
            if repo_path.file_name() == git_dir {
                let repo_path_path = repo_path.path();
                let parent_path = repo_path_path.parent().unwrap();
                let striped: &Path = parent_path.strip_prefix(repo_root).unwrap();
                println!("{}", striped.to_str().unwrap());
                break;
            }
            let file_type = repo_path.file_type().unwrap();
            if file_type.is_dir() {
                dirs_to_read.push(repo_path.path().into_os_string().into_string().unwrap());
            }
        }
    }
}
