use clap::{App, AppSettings, Arg};

pub fn app<'a>() -> App<'static, 'static> {
    App::new("find-repos")
        .version(crate_version!())
        .about("Find git repos")
        .author("Kyle D. <kdeal@kyledeal.com>")
        .setting(AppSettings::ColoredHelp)
        .arg(
            Arg::with_name("base_path")
                .default_value("./")
                .help("If file path should be printed"),
        )
        .arg(
            Arg::with_name("full_path")
                .help("If file path should be printed")
                .long("full-path")
                .short("p"),
        )
        .arg(
            Arg::with_name("filter")
                .help("Filter the list by string")
                .takes_value(true)
                .long("filter")
                .short("f"),
        )
        .arg(
            Arg::with_name("no_short_circuit")
                .help("If subdirectories of a git repo should be explored")
                .long("full-search")
                .short("s"),
        )
}
