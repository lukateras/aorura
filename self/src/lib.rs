use failure::{Error, *};
use num_enum::*;
use serde::*;
use serial::*;

use std::convert::TryFrom;
use std::io::{Read, Write};
use std::path::Path;

const SERIAL_SETTINGS: PortSettings = PortSettings {
    baud_rate: Baud19200,
    char_size: Bits8,
    flow_control: FlowNone,
    parity: ParityNone,
    stop_bits: Stop1,
};

#[derive(Clone, Copy, Debug, Deserialize, IntoPrimitive, PartialEq, Serialize, TryFromPrimitive)]
#[serde(rename_all = "lowercase")]
#[repr(u8)]
pub enum Color {
    Blue = b'B',
    Green = b'G',
    Orange = b'O',
    Purple = b'P',
    Red = b'R',
    Yellow = b'Y',
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum State {
    Aurora,
    Flash(Color),
    Off,
    Static(Color),
}

pub type Command = [u8; 2];

pub const STATUS_COMMAND: Command = [b'S', b'S'];

impl Default for State {
    fn default() -> Self { State::Flash(Color::Blue) }
}

impl Into<Command> for State {
    fn into(self) -> Command {
        match self {
            State::Aurora => [b'A', b'<'],
            State::Flash(color) => [color.into(), b'*'],
            State::Static(color) => [color.into(), b'!'],
            State::Off => [b'X', b'X'],
        }
    }
}

impl TryFrom<&Command> for State {
    type Error = Error;

    fn try_from(cmd: &Command) -> Fallible<Self> {
        match cmd {
            [b'A', b'<'] => Ok(State::Aurora),
            [color, b'*'] => Ok(State::Flash(Color::try_from(*color)?)),
            [color, b'!'] => Ok(State::Static(Color::try_from(*color)?)),
            [b'X', b'X'] => Ok(State::Off),
            _ => bail!("command does not represent a state: {:?}", cmd),
        }
    }
}

pub struct Led(SystemPort);

impl Led {
    // Open AORURA LED serial port.
    pub fn open<P: AsRef<Path>>(path: P) -> Fallible<Self> {
        let mut port = serial::open(path.as_ref())?;
        port.configure(&SERIAL_SETTINGS)?;

        Ok(Self(port))
    }

    /// Get AORURA LED state.
    pub fn get(&mut self) -> Fallible<State> {
        let mut cmd = [0u8; 2];

        self.0.write(&STATUS_COMMAND)?;
        self.0.read_exact(&mut cmd)?;

        State::try_from(&cmd)
    }

    /// Set AORURA LED to given state.
    pub fn set(&mut self, state: State) -> Fallible<()> {
        let cmd: Command = state.into();
        let mut output = [0u8; 1];

        self.0.write(&cmd)?;
        self.0.read_exact(&mut output)?;

        ensure!(&output == b"Y");

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_from_command() {
        let state = State::try_from(b"B*").unwrap();
        assert_eq!(state, State::Flash(Color::Blue));
    }

    #[test]
    fn test_state_into_command() {
        let cmd: Command = State::Flash(Color::Blue).into();
        assert_eq!(&cmd, b"B*");
    }
}
