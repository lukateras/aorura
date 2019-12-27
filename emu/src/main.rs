use aorura_emu::{Pty, Server};
use docopt::Docopt;
use failure::*;
use serde::*;

use std::env;
use std::path::PathBuf;

const USAGE: &'static str = "
Usage: aorura-emu <path>
       aorura-emu --help

Emulates AORURA LED device over a PTY symlinked to given path.
";

#[derive(Debug, Deserialize)]
struct Args {
    arg_path: PathBuf,
}

fn main() -> Fallible<()> {
    env_logger::init();

    let args: Args = Docopt::new(USAGE)?
        .argv(env::args())
        .deserialize()
        .unwrap_or_else(|e| e.exit());

    let mut pty = Pty::open(args.arg_path)?;
    Server::new().run(&mut pty.master, true)
}
