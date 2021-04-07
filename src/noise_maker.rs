use crate::instruments::{Instrument, InstrumentType};
use core::f32;
use rodio::source::Source;
use std::{
    f64::consts::{FRAC_2_PI, PI, TAU},
    sync::{Arc, Mutex},
    time::Duration,
};

pub type FreqType = f64;

/// Converts frequency (Hz) to angular velocity
fn w(hertz: FreqType) -> FreqType {
    hertz * 2.0 * PI
}

#[derive(Debug)]
pub struct Note {
    pub id: u8,
    pub on: FreqType,
    pub off: FreqType,
    pub active: bool,
    pub instrument_id: usize,
}

impl Default for Note {
    fn default() -> Self {
        Self {
            id: 0,
            on: 0.0,
            off: 0.0,
            active: false,
            instrument_id: 0,
        }
    }
}

#[derive(Clone, Copy)]
#[allow(dead_code)]
pub enum WaveType {
    Sine,
    Square,
    Triangle,
    SawSlow,
    SawFast,
    Noise,
}

pub fn osc(
    dt: FreqType,
    freq: FreqType,
    wave: WaveType,
    lfo_hertz: FreqType,
    lfo_amplitude: FreqType,
) -> FreqType {
    let mut phase = w(freq) * dt;
    let lfo_phase = w(lfo_hertz) * dt;
    phase += lfo_amplitude * lfo_phase * lfo_phase.sin();
    match wave {
        WaveType::Sine => phase.sin(),
        WaveType::Square => phase.sin().signum(),
        WaveType::Triangle => phase.sin().asin() * FRAC_2_PI,
        WaveType::SawSlow => {
            let out = (1..50)
                .map(|x| x as FreqType)
                .fold(0.0, |acc, curr| acc + ((curr * phase).sin() / curr));
            out * FRAC_2_PI
        }
        WaveType::SawFast => (phase % TAU) / PI - 1.0,
        WaveType::Noise => fastrand::i32(-1..1) as FreqType,
    }
}

#[derive(Clone, Copy)]
pub struct EnvelopeADSR {
    pub attack_time: FreqType,
    pub decay_time: FreqType,
    pub sustain_amplitude: FreqType,
    pub release_time: FreqType,
    pub start_amplitude: FreqType,
}

impl Default for EnvelopeADSR {
    fn default() -> Self {
        Self {
            attack_time: 0.1,
            decay_time: 0.1,
            sustain_amplitude: 1.0,
            release_time: 0.2,
            start_amplitude: 1.0,
        }
    }
}

impl EnvelopeADSR {
    pub fn amplitude(&self, dt: FreqType, dt_on: FreqType, dt_off: FreqType) -> FreqType {
        if dt_on <= 0.0 {
            return 0.0;
        }

        let lifetime = if dt_on > dt_off {
            dt - dt_on
        } else {
            dt_off - dt_on
        };

        let mut amplitude = if lifetime <= self.attack_time {
            // Attack
            (lifetime / self.attack_time) * self.start_amplitude
        } else if lifetime <= (self.attack_time + self.decay_time) {
            // Decay
            ((lifetime - self.attack_time) / self.decay_time)
                * (self.sustain_amplitude - self.start_amplitude)
                + self.start_amplitude
        } else {
            // Sustain
            self.sustain_amplitude
        };

        if dt_on <= dt_off {
            // Release
            amplitude = ((dt - dt_off) / self.release_time) * -amplitude + amplitude;
        }

        if amplitude <= 0.0001 {
            amplitude = 0.0;
        }

        amplitude
    }
}

pub struct NoiseMaker {
    pub data: Arc<Mutex<NoiseMakerData>>,
    num_sample: usize,
    instruments: Vec<InstrumentType>,
}

pub struct NoiseMakerData {
    pub dt: FreqType,
    pub notes: Vec<Note>,
}

impl Default for NoiseMakerData {
    fn default() -> Self {
        Self {
            dt: 0.0,
            notes: Vec::new(),
        }
    }
}

impl NoiseMaker {
    pub fn new(data: Arc<Mutex<NoiseMakerData>>, instruments: Vec<InstrumentType>) -> Self {
        Self {
            data,
            num_sample: 0,
            instruments,
        }
    }
}

impl Source for NoiseMaker {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 {
        1
    }

    fn sample_rate(&self) -> u32 {
        48000
    }

    fn total_duration(&self) -> Option<Duration> {
        None
    }
}

impl Iterator for NoiseMaker {
    type Item = f32;

    #[inline]
    fn next(&mut self) -> Option<f32> {
        let noise = if let Ok(mut data) = self.data.lock() {
            self.num_sample = self.num_sample.wrapping_add(1);
            data.dt = self.num_sample as FreqType / self.sample_rate() as FreqType;
            make_noise(data.dt, &mut data.notes, &self.instruments)
        } else {
            0.0
        };
        // dbg!(noise);
        Some(noise as f32)
    }
}

fn make_noise(dt: FreqType, notes: &mut Vec<Note>, instruments: &[InstrumentType]) -> FreqType {
    let mixed_output: FreqType = notes
        .iter_mut()
        .map(|note| {
            let (sound, finished) = instruments[note.instrument_id].play_note(dt, note);
            if finished && note.off > note.on {
                note.active = false;
            }
            sound
        })
        .sum();

    while let Some(index) = notes.iter().position(|x| !x.active) {
        notes.remove(index);
    }

    mixed_output * 0.2
}
