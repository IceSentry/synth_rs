use core::f32;
use rodio::source::Source;
use std::sync::{Arc, Mutex};
use std::{
    f32::consts::{FRAC_2_PI, FRAC_PI_2, PI},
    time::Duration,
};

/// Converts frequency (Hz) to angular velocity
fn w(hertz: f32) -> f32 {
    hertz * 2.0 * PI
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

pub fn osc(dt: f32, freq: f32, wave: WaveType, lfo_hertz: f32, lfo_amplitude: f32) -> f32 {
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
                .map(|x| x as f32)
                .fold(0.0, |acc, curr| acc + ((curr * base_freq).sin() / curr));
            out * FRAC_2_PI
        }
        WaveType::SawFast => FRAC_2_PI * (freq * PI * (dt % (1.0 / freq)) - FRAC_PI_2),
        WaveType::Noise => fastrand::i32(-1..1) as f32,
    }
}

pub struct EnvelopeADSR {
    attack_time: f32,
    decay_time: f32,
    sustain_amplitude: f32,
    release_time: f32,
    start_amplitude: f32,

    trigger_on_time: f32,
    trigger_off_time: f32,
    note_on: bool,
}

impl EnvelopeADSR {
    fn new() -> Self {
        Self {
            attack_time: 0.1,
            decay_time: 0.1,
            sustain_amplitude: 1.0,
            release_time: 0.2,
            start_amplitude: 1.0,
            trigger_on_time: 0.0,
            trigger_off_time: 0.0,
            note_on: false,
        }
    }

    fn amplitude(&self, dt: f32) -> f32 {
        let mut amplitude = 0.0;
        let mut release_amplitude = 0.0;
        let lifetime = dt - self.trigger_on_time;

        if self.note_on {
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

            amplitude = ((dt - self.trigger_off_time) / self.release_time) * -release_amplitude
                + release_amplitude;
        }

        if amplitude <= 0.0001 {
            amplitude = 0.0;
        }

        amplitude
    }

    pub fn note_on(&mut self, dt_on: f32) {
        self.trigger_on_time = dt_on;
        self.note_on = true;
    }

    pub fn note_off(&mut self, dt_off: f32) {
        self.trigger_off_time = dt_off;
        self.note_on = false;
    }
}

pub struct NoiseMaker {
    data: Arc<Mutex<NoiseMakerData>>,
}

pub struct NoiseMakerData {
    num_sample: usize,
    pub dt: f32,
    pub freq: f32,
    pub envelope: EnvelopeADSR,
}

impl NoiseMakerData {
    pub fn new() -> Self {
        Self {
            envelope: EnvelopeADSR::new(),
            freq: 0.0,
            dt: 0.0,
            num_sample: 0,
        }
    }
}

impl Default for NoiseMakerData {
    fn default() -> Self {
        Self::new()
    }
}

impl NoiseMaker {
    pub fn new(data: Arc<Mutex<NoiseMakerData>>) -> Self {
        Self { data }
    }

    fn make_noise(&self) -> f32 {
        if let Ok(data) = self.data.lock() {
            data.envelope.amplitude(data.dt) * (osc(data.freq, data.dt, WaveType::Sine, 0.0, 0.0))
        } else {
            0.0
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
        if let Ok(mut data) = self.data.lock() {
            data.num_sample = data.num_sample.wrapping_add(1);
            data.dt = data.num_sample as f32 / self.sample_rate() as f32;
        }
        Some(self.make_noise())
    }
}
