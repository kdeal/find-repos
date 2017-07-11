use std::path::Path;
use std::fs;

fn main() {
    let repo_root = "/Users/kyle/repos/".to_string();
    let mut repo_paths: Vec<String> = vec![];
    let mut dirs_to_read: Vec<String> = vec![repo_root];
    loop {
        let paths = dirs_to_read.pop();
        if paths.is_none() {
            break;
        }
        let paths = paths.unwrap();
        let paths = fs::read_dir(paths).expect("Failed to read directory");
        for path in paths {
            let repo_path = path.unwrap();
            let file_name = repo_path.file_name().into_string().unwrap();
            if file_name == ".git" {
                let repo_path_path = repo_path.path();
                let parent_path = repo_path_path.parent().unwrap();
                let striped: &Path = parent_path.strip_prefix("/Users/kyle/repos/")
                    .unwrap();
                repo_paths.push(String::from(striped.to_str().unwrap()));
                break;
            }
            let file_type = repo_path.file_type().unwrap();
            if !file_name.starts_with(".") & file_type.is_dir() {
                dirs_to_read.push(repo_path.path()
                    .into_os_string().into_string().unwrap()
                );
            }
        }
    }

    for path in repo_paths {
        println!("{}", path)
    }
}
