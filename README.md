# midi2x11
A little tool to convert MIDI key presses to X11 key presses.

## Usage
This builds just like any other Rust project using Cargo.

``` shell
$ cargo build --release
...
Finished release [optimized] target(s) in 6.06s
$ ./target/release/midi2x11 23:Return
Available input ports:
0: Midi-Bridge:Digital Piano:(capture_0) Digital Piano MIDI 0
...
```

## License

This project is licensed under the MIT license, see the `LICENSE` file.
