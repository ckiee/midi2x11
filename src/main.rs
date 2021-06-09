use anyhow::{anyhow, bail, Result};
use key::KeyPresser;
use midir::{Ignore, MidiInput};
use std::env;
use std::io::{stdin, stdout, Write};
use std::sync::mpsc::channel;
use std::collections::HashMap;

mod key;
const USAGE: &str = "Usage:\t midi2x11 midi_key:x11_key ...\n\t\t midi2x11 23:Return 24:Escape";

fn main() -> Result<()> {
    let mut keymap: HashMap<u8, u8> = HashMap::new();
    let kp = KeyPresser::new();

    if env::args().len() < 2 {
        bail!(format!("Invalid Usage\n\n{}", USAGE));
    }

    for arg in env::args().skip(1) {
        let vec: Vec<&str> = arg.split(':').collect();
        if vec.len() != 2 {
            bail!(format!("Invalid Argument\n\n{}", USAGE));
        }
        keymap.insert(vec[0].parse()?, kp.get_keycode(vec[1])?);
    }

    let mut midi_in = MidiInput::new("midi2x11 reading input")?;
    midi_in.ignore(Ignore::None);

    let in_ports = midi_in.ports();
    let in_port = match in_ports.len() {
        0 => bail!("no input port found"),
        1 => {
            println!(
                "Choosing the only available input port: {}",
                midi_in.port_name(&in_ports[0]).unwrap()
            );
            &in_ports[0]
        }
        _ => {
            println!("\nAvailable input ports:");
            for (i, p) in in_ports.iter().enumerate() {
                println!("{}: {}", i, midi_in.port_name(p).unwrap());
            }
            print!("Please select input port: ");
            stdout().flush()?;
            let mut input = String::new();
            stdin().read_line(&mut input)?;
            in_ports
                .get(input.trim().parse::<usize>()?)
                .ok_or(anyhow!("invalid input port selected"))?
        }
    };

    let in_port_name = midi_in.port_name(in_port)?;

    let (tx, rx) = channel();

    // _conn_in needs to be a named parameter, because it needs to be kept alive until the end of the scope
    let _conn_in = midi_in.connect(
        in_port,
        "midi2x11-read-input",
        move |stamp, message, _| {
            if message.len() > 1 {
                // 1 byte long ones are boring
                #[cfg(debug_assertions)]
                println!("{}: {:?} (len = {})", stamp, message, message.len());
            }
            match message.len() {
                3 if message[0] == 144 || message[0] == 128 => {
                    // key up/down
                    let down = message[0] == 144;
                    let key = message[1];
                    tx.send(MidiEvent { key, down }).unwrap();
                }
                _ => {}
            }
        },
        (),
    );

    println!("Connection open, reading input from '{}'...", in_port_name);

    for event in rx.iter() {
        kp.send_key_event(
            event.down,
            (*keymap.get(&event.key).or_else(|| Some(&0)).unwrap()).into(), // should be okay to unwrap here due to the or_else
        );
    }

    Ok(())
}

struct MidiEvent {
    down: bool,
    key: u8,
}
