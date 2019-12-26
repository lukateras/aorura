use aorura::*;
use docopt::Docopt;
use failure::*;
use serde::*;

use nix::fcntl::{self, OFlag};
use nix::pty::*;
use nix::sys::stat::Mode;

use std::convert::TryFrom;
use std::env;
use std::fs::{remove_file, File};
use std::io::prelude::*;
use std::os::unix::fs::symlink;
use std::os::unix::io::*;
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

struct Pty {
    master_fd: PtyMaster,
    slave_fd: RawFd,
    slave_path: PathBuf,
}

impl Pty {
    fn open() -> Fallible<Self> {
        let master_fd = posix_openpt(OFlag::O_RDWR | OFlag::O_NOCTTY)?;

        grantpt(&master_fd)?;
        unlockpt(&master_fd)?;

        let slave_path = PathBuf::from(unsafe { ptsname(&master_fd) }?);
        let slave_fd = fcntl::open(&slave_path, OFlag::O_RDWR, Mode::empty())?;

        Ok(Pty {
            master_fd,
            slave_fd,
            slave_path,
        })
    }
}

fn main() -> Fallible<()> {
    env_logger::init();

    let args: Args = Docopt::new(USAGE)?
        .argv(env::args())
        .deserialize()
        .unwrap_or_else(|e| e.exit());

    let mut state = State::Flash(Color::Blue);

    let pty = Pty::open()?;
    let _ = remove_file(&args.arg_path);
    symlink(&pty.slave_path, &args.arg_path)?;

    let mut f = unsafe { File::from_raw_fd(pty.master_fd.clone().into_raw_fd()) };
    let mut cmd: Command = [0u8; 2];

    loop {
        f.read_exact(&mut cmd)?;

        if cmd == STATUS_COMMAND {
            cmd = state.into();
            f.write(&cmd)?;
        } else {
            match State::try_from(&cmd) {
                Ok(new_state) => {
                    state = new_state;
                    f.write(b"Y")?;
                }
                Err(err) => {
                    log::warn!("{}", err);
                    f.write(b"N")?;
                }
            }
        }
    }
}
