use aorura::*;
use docopt::Docopt;
use failure::*;

use serde::de::{Deserializer, IntoDeserializer};
use serde::*;

use std::env;
use std::path::PathBuf;

fn deserialize_option_state<'de, D>(deserializer: D) -> Result<Option<State>, D::Error>
where
    D: Deserializer<'de>,
{
    let maybe_input: Option<String> = Option::deserialize(deserializer)?;
    if maybe_input == None {
        return Ok(None);
    }

    let input = maybe_input.unwrap();
    let input_segments: Vec<&str> = input.split(':').collect();

    let state = match input_segments[..] {
        [state_str, color_str] if ["flash", "static"].contains(&state_str) => {
            let color = Color::deserialize(color_str.into_deserializer())?;
            match state_str {
                "flash" => State::Flash(color),
                "static" => State::Static(color),
                _ => unreachable!(),
            }
        }
        _ => State::deserialize(input.into_deserializer())?,
    };

    Ok(Some(state))
}

fn state_into_string(state: State) -> Fallible<String> {
    Ok(match state {
        State::Flash(color) => format!("flash:{}", serde_plain::to_string(&color)?),
        State::Static(color) => format!("static:{}", serde_plain::to_string(&color)?),
        _ => serde_plain::to_string(&state)?,
    })
}

const USAGE: &'static str = "
Usage: aorura-cli <path> [--set STATE]
       aorura-cli --help

Gets, and optionally, sets AORURA LED state.

Options:
  --set STATE  set LED to given state

States: aurora, flash:COLOR, off, static:COLOR
Colors: blue, green, orange, purple, red, yellow
";

#[derive(Debug, Deserialize)]
struct Args {
    arg_path: PathBuf,
    #[serde(deserialize_with = "deserialize_option_state")]
    flag_set: Option<State>,
}

fn main() -> Fallible<()> {
    let args: Args = Docopt::new(USAGE)?
        .argv(env::args())
        .deserialize()
        .unwrap_or_else(|e| e.exit());

    let mut led = Led::open(args.arg_path)?;
    let state = match args.flag_set {
        Some(state) => {
            led.set(state)?;
            state
        }
        None => led.get()?,
    };

    println!("{}", state_into_string(state)?);

    Ok(())
}
