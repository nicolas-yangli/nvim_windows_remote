use std::path::Path;

#[macro_use]
extern crate clap;
use clap::{App, Arg};

use nvim_windows_remote::*;

fn main() {
    let args = App::new("nvim_windows_remote")
        .arg(Arg::with_name("edit_command")
            .short("c")
            .long("edit-command")
            .takes_value(true)
            .possible_values(&EditCommand::variants()),
        )
        .arg(Arg::with_name("file").index(1))
        .get_matches();

    let edit_command = value_t!(args.value_of("edit_command"), EditCommand).unwrap();
    let filename = args.value_of("file").unwrap();
    let filepath = Path::new(&filename);
    edit_with_existing_nvim(edit_command, filepath).unwrap();
}
