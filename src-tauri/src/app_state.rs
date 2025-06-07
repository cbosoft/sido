use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Duration;

use fundsp::hacker::*;
use tauri::ipc::Channel;

use crate::marker::marker;
use crate::note::SpecifiedNote;
use crate::patch::Patch;


pub struct AppState {
    net: Net,
    seq: Sequencer,
    patch: Patch,
}


impl AppState {
    pub fn new(mut net: Net) -> Self {
        let mut seq = Sequencer::new(false, 2);
        net.chain(Box::new(seq.backend()));
        net.commit();
        let patch = Patch::from_file("boop.yaml").unwrap();
        Self {
            net,
            seq,
            patch,
        }
    }

    pub fn boop(&mut self) {
        self.seq.push_relative(0.0, 0.5, Fade::Smooth, 0.0, 0.0, Box::new( saw_hz(110.0)*5.0 >> pan(0.0) ));
    }

    pub fn play_current_patch(&mut self, note: SpecifiedNote) {
        self.patch.play(note, &mut self.seq, 0.5);
    }

    pub fn play_sequence(&mut self, bpm: f64, beats: u64, divisions: u64, notes: Vec<SpecifiedNote>, time_ind: Channel<u64>) {
        let bps = bpm / 60.0;
        let spd = 1.0 / (divisions as f64) / bps;
        let mark = Arc::new(AtomicU64::new(0));
        for (i, note) in notes.into_iter().enumerate() {
            let (s, _e) = self.patch.play(note, &mut self.seq, spd);
        }

        let end = beats * divisions;
        for i in 0..end {
            let mark = Arc::clone(&mark);
            let s = spd * i as f64;
            let e = spd * (i+1) as f64;
            self.seq.push_relative(s, e, Fade::Smooth, 0.0, 0.0, Box::new( (zero() | marker(i as u64, mark)) >> pan(0.0) ));
        }

        let mut now = 0u64;
        time_ind.send(now).unwrap();
        while now < (end - 1) {
            let new_now = mark.load(Ordering::Relaxed);
            if new_now != now {
                now = new_now;
                time_ind.send(now).unwrap();
            }
            std::thread::sleep(Duration::from_secs_f64(spd*0.1));
        }
        time_ind.send(end).unwrap();
    }

    pub fn set_patch(&mut self, patch: Patch) {
        self.patch = patch;
    }
}
