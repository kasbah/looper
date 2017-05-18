extern crate jack;
use jack::prelude as j;

pub enum LooperState {
    Paused,
    Playing,
    Recording,
}

pub struct Looper {
    loop_  : Vec<f32>,
    cursor : usize,
    pub thru  : f32,
    pub state : LooperState,
}

impl Looper {
    pub fn new() -> Looper {
       Looper {
           loop_  : Vec::new(),
           thru   : 1.0,
           state  : LooperState::Paused,
           cursor : 0,
       }
    }
    fn run_thru (
        &self,
        in_a_p  : &j::AudioInPort,
        in_b_p  : &j::AudioInPort,
        out_a_p : &mut j::AudioOutPort,
        out_b_p : &mut j::AudioOutPort
    ) {
        let mut amplified_a : Vec<f32> = in_a_p.to_vec();
        let mut amplified_b : Vec<f32> = in_b_p.to_vec();
        amplified_a = amplified_a.iter().map(|&x| x * self.thru).collect();
        amplified_b = amplified_b.iter().map(|&x| x * self.thru).collect();
        out_a_p.clone_from_slice(&amplified_a);
        out_b_p.clone_from_slice(&amplified_b);
    }
    fn advance(&mut self, n_frames: j::JackFrames) {
        let new_cursor = self.cursor + n_frames as usize;
        let length = self.loop_.len();
        if new_cursor >= length {
            self.cursor = new_cursor - length;
        } else {
            self.cursor = new_cursor;
        }
    }
    pub fn run(
        &mut self,
        n_frames : j::JackFrames,
        in_a_p   : &j::AudioInPort,
        in_b_p   : &j::AudioInPort,
        out_a_p  : &mut j::AudioOutPort,
        out_b_p  : &mut j::AudioOutPort
    ) {
        match self.state {
            LooperState::Paused => {
            }
            LooperState::Playing => {
                self.advance(n_frames);
            }
            LooperState::Recording => {
            }
        }
        self.run_thru(&in_a_p, &in_b_p,  out_a_p,  out_b_p);
    }
}
