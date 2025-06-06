use fundsp::hacker::*;

pub struct AppState {
    net: Net,
    seq: Sequencer
}

impl AppState {
    pub fn new(mut net: Net) -> Self {
        let mut seq = Sequencer::new(false, 2);
        net.chain(Box::new(seq.backend()));
        net.commit();
        Self {
            net,
            seq
        }
    }

    pub fn boop(&mut self) {
        self.seq.push_relative(0.0, 0.5, Fade::Smooth, 0.0, 0.0, Box::new( saw_hz(110.0)*5.0 >> pan(0.0) ));
    }
}
