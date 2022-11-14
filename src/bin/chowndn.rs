use chowndn::commands::{replace, scan, version};

use clap::{Arg, Command};

fn cli() -> Command {
    Command::new("chowndn")
        .about("A command line tool for people of transgender experience to replace their deadname within a Git repo.")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new("version")
                .about("Displays the version of chowndn")
        )
        .subcommand(
            Command::new("scan")
                .about("Searches repo for instances of dead name")
                .arg(Arg::new("deadname")
                .help("Deadname to scan for"))
                .arg(Arg::new("repo")
                .help("path to git repo"))
        )
        .subcommand(
            Command::new("replace")
                .about("Searches repo for instances of dead name")
                .arg(Arg::new("deadname")
                .help("Deadname to replace"))
                .arg(Arg::new("newname")
                .help("New name to replace with"))
                .arg(Arg::new("repo")
                .help("path to git repo"))
        )
}

fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("version", _sub_matches)) => {
            version();
        }

        Some(("scan", _sub_matches)) => {
            let deadname = _sub_matches.get_one::<String>("deadname").unwrap();
            let repo = _sub_matches.get_one::<String>("repo").unwrap();

            scan(&deadname, &repo);
        }

        Some(("replace", _sub_matches)) => {
            let deadname = _sub_matches.get_one::<String>("deadname").unwrap();
            let newname = _sub_matches.get_one::<String>("newname").unwrap();
            let repo = _sub_matches.get_one::<String>("repo").unwrap();

            replace(&deadname, &newname, &repo);
        }
        _ => unreachable!(),
    }
}
