extern crate jack;
use jack::prelude as j;

use std::io::{self, Read};

use std::sync::mpsc::sync_channel;

// Create a simple streaming channel

mod looper;
use looper::*;

enum Messages {
    SetThru(f32),
}

fn main() {
    // Create client
    let (client, _status) = j::Client::new("rust_jack_simple", j::client_options::NO_START_SERVER)
        .unwrap();

    // Register ports. They will be used in a callback that will be
    // called when new data is available.
    let in_a       = client.register_port("rust_in_l", j::AudioInSpec::default()).unwrap();
    let in_b       = client.register_port("rust_in_r", j::AudioInSpec::default()).unwrap();
    let mut out_a  = client.register_port("rust_out_l", j::AudioOutSpec::default()).unwrap();
    let mut out_b  = client.register_port("rust_out_r", j::AudioOutSpec::default()).unwrap();
    let midi_in    = client.register_port("rust_midi_in", j::MidiInSpec::default()).unwrap();
    let mut looper = Looper::new();
    let (tx, rx) = sync_channel::<Messages>(0);
    let process_callback = move |_: &j::Client, ps: &j::ProcessScope| -> j::JackControl {
        let mut out_a_p = j::AudioOutPort::new(&mut out_a, ps);
        let mut out_b_p = j::AudioOutPort::new(&mut out_b, ps);
        let in_a_p      = j::AudioInPort::new(&in_a, ps);
        let in_b_p      = j::AudioInPort::new(&in_b, ps);
        let midi_in_p   = j::MidiInPort::new(&midi_in, ps);
        for e in midi_in_p.iter() {
            println!("{:?}", e);
        }
        let msg = rx.try_recv();
        match msg {
            Ok(Messages::SetThru(value)) => {
                looper.thru = value;
            },
            _ => {}
        }
        looper.run(ps.n_frames(), &in_a_p, &in_b_p, &mut out_a_p, &mut out_b_p);
        j::JackControl::Continue
    };
    let process = j::ClosureProcessHandler::new(process_callback);

    // Activate the client, which starts the processing.
    let active_client = j::AsyncClient::new(client, (), process).unwrap();

    // Wait for user input to quit
    println!("Enter 'q' to quit...");
    loop {
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).ok();
        match user_input.as_ref() {
            "q\n" => {
                break;
            },
            "p\n" => {
                println!("pausing");
            },
            "r\n" => {
                println!("recording");
            },
            "c\n" => {
                println!("playing");
            },
            "w\n" => {
                println!("up");
                tx.send(Messages::SetThru(1.0));
            },
            "s\n" => {
                println!("down");
                tx.send(Messages::SetThru(0.0));
            },
            _ => {},
        }
    }

    active_client.deactivate().unwrap();
}
