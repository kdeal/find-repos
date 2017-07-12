use std::path::Path;
use std::fs;
use std::env;

fn main() {
    let repo_root = match env::args().nth(1) {
        Some(path) => path,
        None => format!("{}/repos", env::home_dir().unwrap().to_str().unwrap()),
    };
    let mut dirs_to_read: Vec<String> = vec![repo_root.clone()];
    let repo_root = repo_root.as_str();

    'dirs: while !dirs_to_read.is_empty() {
        let path = dirs_to_read.pop().unwrap();
        let paths = fs::read_dir(path);
        if !paths.is_ok() {
            continue;
        }

        let mut dir_paths: Vec<String> = vec![];
        for path in paths.unwrap() {
            if !path.is_ok() {
                continue;
            }

            let repo_path = path.unwrap();
            if repo_path.file_name().eq(".git") {
                let repo_path_path = repo_path.path();
                let parent_path = repo_path_path.parent().unwrap();
                let striped: &Path = parent_path.strip_prefix(repo_root).unwrap();
                println!("{}", striped.to_str().unwrap());
                continue 'dirs;
            }

            let file_type = repo_path.file_type().unwrap();
            if file_type.is_dir() {
                dir_paths.push(repo_path.path().into_os_string().into_string().unwrap());
            }
        }
        dirs_to_read.append(&mut dir_paths);
    }
}
