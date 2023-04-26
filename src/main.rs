use clap::{arg, command, ArgGroup, Command};

mod commands;
mod languages;

fn main() {
    let cli = cli();

    let matches = cli.get_matches();

    match matches.subcommand() {
        Some(("daily", _)) => {
            commands::daily();
        }
        Some(("problem", submatches)) => {
            println!("Submatches: {:?}", submatches.get_one::<String>("language"));
            commands::problem();
        }
        Some(("random", _)) => {
            commands::random();
        }
        _ => unreachable!(),
    }
}

fn cli() -> Command {
    command!() // requires `cargo` feature
        .bin_name("leetcode")
        .about("A simple CLI tool for quickly create boilerplate code for leetcode problems")
        .subcommand_required(true)
        .subcommand(
            Command::new("daily")
                .about("Fetch the daily problem for today")
                .arg(language_arg()),
        )
        .subcommand(
            Command::new("problem")
                .about("Fetch a specific problem")
                .arg(language_arg())
                .args(vec![
                    arg!(-t --title <TITLE>)
                        .action(clap::ArgAction::Set)
                        .help("The title of the problem, e.g. 'Two Sum'"),
                    arg!(-n --number <NUMBER>)
                        .action(clap::ArgAction::Set)
                        .help("The number of the problem, e.g. '1'"),
                    arg!(-u --url <URL>).action(clap::ArgAction::Set).help(
                        "The url of the problem, e.g. 'https://leetcode.com/problems/two-sum/'",
                    ),
                ])
                .group(
                    ArgGroup::new("problem")
                        .args(vec!["title", "number", "url"])
                        .required(true),
                ),
        )
        .subcommand(
            Command::new("random")
                .about("Fetch a random problem")
                .arg(language_arg()),
        )
}

fn language_arg() -> clap::Arg {
    arg!(-l --language <PROGRAMMING_LANGUAGE>)
        .action(clap::ArgAction::Set)
        .help("The programming language you want to use")
}
