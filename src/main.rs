extern crate jack;
use jack::prelude as j;
use std::io;

fn run(in_a_p: &j::AudioInPort, in_b_p: &j::AudioInPort, out_a_p: &mut j::AudioOutPort , out_b_p: &mut j::AudioOutPort) {
    let mut amplified_a : Vec<f32> = in_a_p.to_vec();
    let mut amplified_b : Vec<f32> = in_b_p.to_vec();
    amplified_a = amplified_a.iter().map(|x| x + (x * 0.2)).collect();
    amplified_b = amplified_b.iter().map(|x| x + (x * 0.2)).collect();
    out_a_p.clone_from_slice(&amplified_a);
    out_b_p.clone_from_slice(&amplified_b);
}

fn main() {
    // Create client
    let (client, _status) = j::Client::new("rust_jack_simple", j::client_options::NO_START_SERVER)
        .unwrap();

    // Register ports. They will be used in a callback that will be
    // called when new data is available.
    let in_a = client.register_port("rust_in_l", j::AudioInSpec::default()).unwrap();
    let in_b = client.register_port("rust_in_r", j::AudioInSpec::default()).unwrap();
    let mut out_a = client.register_port("rust_out_l", j::AudioOutSpec::default()).unwrap();
    let mut out_b = client.register_port("rust_out_r", j::AudioOutSpec::default()).unwrap();
    let process_callback = move |_: &j::Client, ps: &j::ProcessScope| -> j::JackControl {
        let mut out_a_p = j::AudioOutPort::new(&mut out_a, ps);
        let mut out_b_p = j::AudioOutPort::new(&mut out_b, ps);
        let in_a_p = j::AudioInPort::new(&in_a, ps);
        let in_b_p = j::AudioInPort::new(&in_b, ps);
        run(&in_a_p, &in_b_p, &mut out_a_p, &mut out_b_p);
        j::JackControl::Continue
    };
    let process = j::ClosureProcessHandler::new(process_callback);

    // Activate the client, which starts the processing.
    let active_client = j::AsyncClient::new(client, (), process).unwrap();

    // Wait for user input to quit
    println!("Press enter/return to quit...");
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).ok();

    active_client.deactivate().unwrap();
}
