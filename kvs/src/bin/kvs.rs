use clap::{App, Arg, SubCommand};
use std::env;

fn main() {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .subcommand(
            SubCommand::with_name("set")
                .about("Set the value of the key")
                .arg(Arg::with_name("KEY").index(1))
                .arg(Arg::with_name("VALUE").index(2)),
        )
        .subcommand(
            SubCommand::with_name("get")
                .about("Get the value of the key")
                .arg(Arg::with_name("KEY")),
        )
        .subcommand(
            SubCommand::with_name("rm")
                .about("Remove the value of the key")
                .arg(Arg::with_name("KEY")),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("set") {
        if !matches.is_present("KEY") || !matches.is_present("VALUE") {
            panic!();
        } else {
            eprintln!("unimplemented");
            panic!();
        }
    } else if let Some(matches) = matches.subcommand_matches("get") {
        if !matches.is_present("KEY") {
            panic!();
        } else {
            eprintln!("unimplemented");
            panic!();
        }
    } else if let Some(matches) = matches.subcommand_matches("rm") {
        if !matches.is_present("KEY") {
            panic!();
        } else {
            eprintln!("unimplemented");
            panic!();
        }
    } else {
        panic!();
    }
}
