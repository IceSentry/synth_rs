use core::f32;
use rodio::source::Source;
use std::sync::{Arc, Mutex};
use std::{
    f64::consts::{FRAC_2_PI, FRAC_PI_2, PI},
    time::Duration,
};

use crate::instruments::{Instrument, InstrumentType};

pub type FreqType = f64;

/// Converts frequency (Hz) to angular velocity
fn w(hertz: FreqType) -> FreqType {
    hertz * 2.0 * PI
}

#[derive(Debug)]
pub struct Note {
    pub id: i32,
    pub on: FreqType,
    pub off: FreqType,
    pub active: bool,
    pub channel: usize,
}

impl Note {
    pub fn new() -> Self {
        Self {
            id: 0,
            on: 0.0,
            off: 0.0,
            active: false,
            channel: 0,
        }
    }
}

impl Default for Note {
    fn default() -> Self {
        Self::new()
    }
}

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
    let base_freq = w(freq) * dt + lfo_amplitude * freq * (w(lfo_hertz) * dt).sin();
    match wave {
        WaveType::Sine => base_freq.sin(),
        WaveType::Square => {
            if base_freq.sin() > 0.0 {
                1.0
            } else {
                -1.0
            }
        }
        WaveType::Triangle => base_freq.sin().asin() * FRAC_2_PI,
        WaveType::SawSlow => {
            let out = (1..50)
                .map(|x| x as FreqType)
                .fold(0.0, |acc, curr| acc + ((curr * base_freq).sin() / curr));
            out * FRAC_2_PI
        }
        WaveType::SawFast => FRAC_2_PI * (freq * PI * (dt % (1.0 / freq)) - FRAC_PI_2),
        WaveType::Noise => fastrand::i32(-1..1) as FreqType,
    }
}

pub fn scale(note_id: i32, _scale_id: i32) -> FreqType {
    256.0 * 1.059_463_094_359_295_3_f64.powi(note_id)
}

#[derive(Clone, Copy)]
pub struct EnvelopeADSR {
    pub attack_time: FreqType,
    pub decay_time: FreqType,
    pub sustain_amplitude: FreqType,
    pub release_time: FreqType,
    pub start_amplitude: FreqType,
}

impl EnvelopeADSR {
    pub fn new() -> Self {
        Self {
            attack_time: 0.1,
            decay_time: 0.1,
            sustain_amplitude: 1.0,
            release_time: 0.2,
            start_amplitude: 1.0,
        }
    }

    pub fn amplitude(&self, dt: FreqType, dt_on: FreqType, dt_off: FreqType) -> FreqType {
        let mut amplitude = 0.0;
        let mut release_amplitude = 0.0;

        if dt_on > dt_off {
            let lifetime = dt - dt_on;

            if lifetime <= self.attack_time {
                amplitude = (lifetime / self.attack_time) * self.start_amplitude;
            }

            if lifetime > self.attack_time && lifetime <= (self.attack_time + self.decay_time) {
                amplitude = ((lifetime - self.attack_time) / self.decay_time)
                    * (self.sustain_amplitude - self.start_amplitude)
                    + self.start_amplitude;
            }

            if lifetime > (self.attack_time + self.decay_time) {
                amplitude = self.sustain_amplitude;
            }
        } else {
            let lifetime = dt_off - dt_on;

            if lifetime <= self.attack_time {
                release_amplitude = (lifetime / self.attack_time) * self.start_amplitude;
            }

            if lifetime > self.attack_time && lifetime <= (self.attack_time + self.decay_time) {
                release_amplitude = ((lifetime - self.attack_time) / self.decay_time)
                    * (self.sustain_amplitude - self.start_amplitude)
                    + self.start_amplitude;
            }

            if lifetime > (self.attack_time + self.decay_time) {
                release_amplitude = self.sustain_amplitude;
            }

            amplitude =
                ((dt - dt_off) / self.release_time) * -release_amplitude + release_amplitude;
        }

        if amplitude <= 0.0001 {
            amplitude = 0.0;
        }

        amplitude
    }
}

pub struct NoiseMaker {
    data: Arc<Mutex<NoiseMakerData>>,
    instruments: Vec<InstrumentType>,
}

pub struct NoiseMakerData {
    num_sample: usize,
    pub dt: FreqType,
    pub envelope: EnvelopeADSR,
    pub notes: Vec<Note>,
}

impl NoiseMakerData {
    pub fn new() -> Self {
        Self {
            envelope: EnvelopeADSR::new(),
            dt: 0.0,
            num_sample: 0,
            notes: Vec::new(),
        }
    }
}

impl Default for NoiseMakerData {
    fn default() -> Self {
        Self::new()
    }
}

impl NoiseMaker {
    pub fn new(data: Arc<Mutex<NoiseMakerData>>, instruments: Vec<InstrumentType>) -> Self {
        Self { data, instruments }
    }

    fn make_noise(&self) -> FreqType {
        match self.data.lock() {
            Ok(mut data) => {
                let dt = data.dt;
                let mixed_output: FreqType = data
                    .notes
                    .iter_mut()
                    .map(|note| {
                        let (sound, finished) = self.instruments[note.channel].sound(dt, note);
                        if finished && note.off > note.on {
                            note.active = false;
                        }
                        sound
                    })
                    .sum();

                while let Some(index) = data.notes.iter().position(|x| !x.active) {
                    data.notes.remove(index);
                }

                mixed_output * 0.2
            }
            Err(_) => 0.0,
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
        // 20
    }

    fn total_duration(&self) -> Option<Duration> {
        None
    }
}

impl Iterator for NoiseMaker {
    type Item = f32;

    #[inline]
    fn next(&mut self) -> Option<f32> {
        if let Ok(mut data) = self.data.lock() {
            data.num_sample = data.num_sample.wrapping_add(1);
            data.dt = data.num_sample as FreqType / self.sample_rate() as FreqType;
        }
        let noise = self.make_noise();
        // dbg!(noise);
        Some(noise as f32)
    }
}
