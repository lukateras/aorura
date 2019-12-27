use failure::*;

use nix::fcntl::{self, OFlag};
use nix::pty::*;
use nix::sys::stat::Mode;

use std::fs::{remove_file, File};
use std::os::unix::fs::symlink;
use std::os::unix::io::*;
use std::path::{Path, PathBuf};

pub struct Pty {
    pub master: File,
    pub slave: File,
    target_path: PathBuf,
}

impl Pty {
    pub fn open<P: AsRef<Path>>(target: P) -> Fallible<Self> {
        let master_fd = posix_openpt(OFlag::O_RDWR | OFlag::O_NOCTTY)?;

        grantpt(&master_fd)?;
        unlockpt(&master_fd)?;

        let slave_path = PathBuf::from(unsafe { ptsname(&master_fd) }?);
        let slave_fd = fcntl::open(&slave_path, OFlag::O_RDWR, Mode::empty())?;

        let target_path = target.as_ref();
        symlink(&slave_path, &target_path)?;

        Ok(Pty {
            master: unsafe { File::from_raw_fd(master_fd.into_raw_fd()) },
            slave: unsafe { File::from_raw_fd(slave_fd) },
            target_path: target_path.to_path_buf(),
        })
    }
}

impl Drop for Pty {
    fn drop(&mut self) {
        let _ = remove_file(&self.target_path);
    }
}
