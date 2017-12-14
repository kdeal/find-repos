#[macro_use]
extern crate clap;

use std::env;
use std::fs;

use clap::Shell;

#[allow(dead_code)]
#[path = "src/app.rs"]
mod app;

fn main() {
    let outdir = match env::var_os("OUT_DIR") {
        None => return,
        Some(outdir) => outdir,
    };
    fs::create_dir_all(&outdir).unwrap();

    let mut app = app::app();
    app.gen_completions("find-repos", Shell::Bash, &outdir);
    app.gen_completions("find-repos", Shell::Fish, &outdir);
    app.gen_completions("find-repos", Shell::PowerShell, &outdir);
}
