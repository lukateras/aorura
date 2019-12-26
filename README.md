# AORURA

AORURA LED library, state manager CLI and emulator.

## Protocol

AORURA exposes serial 19200n8 connection. Every command is exactly two bytes:

- `XX` turns off LED
- `A<` enables signature aurora LED state
- color byte + `!` causes LED to light up with given color
- color byte + `*` causes LED to flash with given color at .5s interval

AORURA will respond with `Y` or `N` byte, depending on whether command was
successfully applied or not, respectively.

There is one more command, `SS`, which causes AORURA to respond with 2-byte
command representation of its current state.

AORURA's initial LED state is flashing blue (`B*`).

Valid color bytes:

- `B`: blue
- `G`: green
- `O`: orange
- `P`: purple
- `R`: red
- `Y`: yellow

## Library

[`aorura`](self) is a Rust crate that implements [AORURA protocol](#protocol).
Usage example:

```rust
use aorura::*;
use failure::*;

fn main() -> Fallible<()> {
  let led = Led::open("/dev/ttyUSB0")?;

  led.set(State::Flash(Color::Red))?; // cause LED to flash with red color
  led.set(State::Off)?; // turn off LED

  assert_eq!(led.get()?, State::Off);
  assert_eq!(State::try_from(b"B*")?, State::Flash(Color::Blue));

  Ok(())
}
```

## CLI

[`aorura-cli`](cli) is a state manager CLI built on top of [`aorura`](#library).

```
Usage: aorura-cli <path> [--set STATE]
       aorura-cli --help

Gets, and optionally, sets AORURA LED state.

Options:
  --set STATE  set LED to given state

States: aurora, flash:COLOR, off, static:COLOR
Colors: blue, green, orange, purple, red, yellow
```

Usage example:

```shell
path=/dev/ttyUSB0
original_state=$(aorura-cli $path)

# Enable flashing yellow LED:
aorura-cli $path --set flash:yellow

# Do something time-consuming:
sleep 10

# Revert back to original LED state:
aorura-cli $path --set $original_state
```

## Emulator

[`aorura-emu`](emu) is a PTY-based AORURA emulator. It can be used with
[CLI](#cli) in lieu of [compatible hardware](#compatible-hardware). For
example, this emulator is used to make status LED visible on HoloPortOS VM.

```
Usage: aorura-emu <path>
       aorura-emu --help

Emulates AORURA LED device over a PTY symlinked to given path.
```

## Compatible hardware

- AORURA-3 (HoloPort and HoloPort+)

  ![AORURA-3 photo](res/aorura-3.jpg)

- AORURA-UART-1 (HoloPort Nano)

  ![AORURA-UART-1 photo](res/aorura-uart-1.jpg)
