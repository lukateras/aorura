use aorura::*;
use failure::*;

use std::convert::TryFrom;
use std::io::prelude::*;
use std::sync::Mutex;

pub struct Server {
    state: Mutex<State>,
}

impl Server {
    pub fn new() -> Self {
        Self {
            state: Mutex::new(Default::default()),
        }
    }

    pub fn get(&self) -> State {
        *self.state.lock().unwrap()
    }

    pub fn run<RW: Read + Write>(&self, master: &mut RW, is_writable: bool) -> Fallible<()> {
        let mut cmd: Command = Default::default();

        loop {
            master.read_exact(&mut cmd)?;

            let mut state = self.state.lock().unwrap();

            if cmd == STATUS_COMMAND {
                cmd = (*state).into();
                master.write(&cmd)?;
            } else {
                match State::try_from(&cmd) {
                    Ok(new_state) => {
                        if is_writable {
                            *state = new_state;
                        }
                        master.write(b"Y")?;
                    }
                    Err(err) => {
                        log::warn!("{}", err);
                        master.write(b"N")?;
                    }
                }
            }
        }
    }

    pub fn set(&self, state: State) -> () {
        *self.state.lock().unwrap() = state
    }
}
