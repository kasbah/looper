extern crate jack;
use jack::prelude as j;
use std::io;

struct Process {
    in_a: j::Port<j::AudioInSpec>,
    in_b: j::Port<j::AudioInSpec>,
    out_a: j::Port<j::AudioOutSpec>,
    out_b: j::Port<j::AudioOutSpec>,
}

impl Process {
    fn callback(& mut self, _: & j::Client, ps: & j::ProcessScope) -> j::JackControl {
        let mut out_a_p = j::AudioOutPort::new(&mut self.out_a, ps);
        let mut out_b_p = j::AudioOutPort::new(&mut self.out_b, ps);
        let in_a_p = j::AudioInPort::new(&self.in_a, ps);
        let in_b_p = j::AudioInPort::new(&self.in_b, ps);
        out_a_p.clone_from_slice(&in_a_p);
        out_b_p.clone_from_slice(&in_b_p);
        j::JackControl::Continue
    }
}

fn main() {
    // Create client
    let (client, _status) = j::Client::new("rust_jack_simple", j::client_options::NO_START_SERVER)
        .unwrap();

    // Register ports. They will be used in a callback that will be
    // called when new data is available.
    let mut process = Process {
        in_a  : client.register_port("rust_in_l", j::AudioInSpec::default()).unwrap(),
        in_b  : client.register_port("rust_in_r", j::AudioInSpec::default()).unwrap(),
        out_a : client.register_port("rust_out_l", j::AudioOutSpec::default()).unwrap(),
        out_b : client.register_port("rust_out_r", j::AudioOutSpec::default()).unwrap(),
    };
    let p = j::ClosureProcessHandler::new(move |client: &j::Client, ps: &j::ProcessScope| process.callback(client, ps));

    // Activate the client, which starts the processing.
    let active_client = j::AsyncClient::new(client, (), p).unwrap();

    // Wait for user input to quit
    println!("Press enter/return to quit...");
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).ok();

    active_client.deactivate().unwrap();
}
