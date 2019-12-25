use aorura::*;
use docopt::Docopt;
use failure::*;

use serde::de::{Deserializer, IntoDeserializer};
use serde::*;

use std::env;
use std::path::PathBuf;

fn deserialize_state<'de, D>(deserializer: D) -> Result<State, D::Error>
where
    D: Deserializer<'de>,
{
    let input = String::deserialize(deserializer)?;
    let input_segments: Vec<&str> = input.split(':').collect();

    match input_segments[..] {
        [state_str, color_str] if ["flash", "static"].contains(&state_str) => {
            let color = Color::deserialize(color_str.into_deserializer())?;
            match state_str {
                "flash" => Ok(State::Flash(color)),
                "static" => Ok(State::Static(color)),
                _ => unreachable!(),
            }
        }
        _ => State::deserialize(input.into_deserializer()),
    }
}

const USAGE: &'static str = "
Usage: aorura-cli --path PATH --state STATE
       aorura-cli --help

Sets AORURA LED state.

Options:
  --path PATH        path to AORURA serial port
  --state STATE      desired LED state

States: aurora, flash:COLOR, off, static:COLOR
Colors: blue, green, orange, purple, red, yellow
";

#[derive(Debug, Deserialize)]
struct Args {
    flag_path: PathBuf,
    #[serde(deserialize_with = "deserialize_state")]
    flag_state: State,
}

fn main() -> Fallible<()> {
    let args: Args = Docopt::new(USAGE)?
        .argv(env::args())
        .deserialize()
        .unwrap_or_else(|e| e.exit());

    Led::open(args.flag_path)?.set(args.flag_state)
}
