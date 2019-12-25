use failure::*;
use serde::*;
use serial::*;

use std::io::{Read, Write};
use std::path::Path;

const SERIAL_SETTINGS: PortSettings = PortSettings {
    baud_rate: Baud19200,
    char_size: Bits8,
    flow_control: FlowNone,
    parity: ParityNone,
    stop_bits: Stop1,
};

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum State {
    Aurora,
    Flash(Color),
    Off,
    Static(Color),
}

type Command = [u8; 2];

impl Into<Command> for State {
    fn into(self) -> Command {
        match self {
            State::Aurora => [b'A', b'<'],
            State::Flash(color) => [color as u8, b'*'],
            State::Static(color) => [color as u8, b'!'],
            State::Off => [b'X', b'X'],
        }
    }
}

pub struct Led(SystemPort);

impl Led {
    // Open AURORA LED serial port.
    pub fn open<P: AsRef<Path>>(path: P) -> Fallible<Self> {
        let mut port = serial::open(path.as_ref())?;
        port.configure(&SERIAL_SETTINGS)?;

        Ok(Self(port))
    }

    /// Set AORURA LED to given state.
    pub fn set(mut self, state: State) -> Fallible<()> {
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
    fn test_state_into_command() {
        let cmd: Command = State::Flash(Color::Blue).into();
        assert_eq!(&cmd, b"B*");
    }
}
