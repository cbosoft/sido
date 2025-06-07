use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use fundsp::hacker::*;


#[derive(Clone)]
pub struct Marker {
    payload: u64,
    output: Arc<AtomicU64>,
}

impl Marker {
    pub fn new(payload: u64, output: Arc<AtomicU64>) -> Self {
        Self {
            payload,
            output,
        }
    }
}

pub fn marker(payload: u64, output: Arc<AtomicU64>) -> An<Marker> {
    An(Marker::new(payload, output))
}

impl AudioNode for Marker {
    const ID: u64 = 88;
    type Inputs = U0;
    type Outputs = U0;

    fn tick(&mut self, input: &Frame<f32, Self::Inputs>) -> Frame<f32, Self::Outputs> {
        let output = input.clone();
        self.output.store(self.payload, Ordering::Relaxed);
        output
    }
}
