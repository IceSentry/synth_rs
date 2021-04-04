use crate::noise_maker::{osc, scale, EnvelopeADSR, FreqType, Note, WaveType};
use enum_dispatch::enum_dispatch;

#[enum_dispatch]
pub trait Instrument {
    fn sound(&self, dt: FreqType, note: &Note) -> (FreqType, bool);
    fn name(&self) -> String {
        String::from("Unknwon")
    }
}

#[enum_dispatch(Instrument)]
pub enum InstrumentType {
    Test,
    Bell,
    Bell8,
    Harmonica,
}

pub struct Test {
    env: EnvelopeADSR,
}

impl Test {
    pub fn new() -> Self {
        Self {
            env: EnvelopeADSR::new(),
        }
    }
}

impl Instrument for Test {
    fn sound(&self, dt: FreqType, note: &Note) -> (FreqType, bool) {
        let amplitude = self.env.amplitude(dt, note.on, note.off);
        let finished = amplitude <= 0.0;
        let dt = note.on - dt;
        let sound = amplitude * osc(dt, scale(note.id, 0), WaveType::SawFast, 0.0, 0.0);
        (sound, finished)
    }
}

pub struct Bell {
    volume: FreqType,
    env: EnvelopeADSR,
}

impl Bell {
    #[allow(dead_code)]
    pub fn new() -> Self {
        let mut env = EnvelopeADSR::new();
        env.attack_time = 0.01;
        env.decay_time = 1.0;
        env.sustain_amplitude = 0.0;
        env.release_time = 1.0;
        Self { volume: 1.0, env }
    }
}

impl Instrument for Bell {
    fn sound(&self, dt: FreqType, note: &Note) -> (FreqType, bool) {
        let amplitude = self.env.amplitude(dt, note.on, note.off);
        let finished = amplitude <= 0.0;
        let dt = note.on - dt;
        let waves = vec![
            osc(dt, scale(note.id + 12, 0), WaveType::Sine, 5.0, 0.001),
            0.5 * osc(dt, scale(note.id + 24, 0), WaveType::Sine, 0.0, 0.0),
            0.25 * osc(dt, scale(note.id + 36, 0), WaveType::Sine, 0.0, 0.0),
        ];
        let sound = amplitude * waves.iter().sum::<FreqType>() * self.volume;
        (sound, finished)
    }
}

pub struct Bell8 {
    volume: FreqType,
    env: EnvelopeADSR,
}

impl Bell8 {
    #[allow(dead_code)]
    pub fn new() -> Self {
        let mut env = EnvelopeADSR::new();
        env.attack_time = 0.01;
        env.decay_time = 0.5;
        env.sustain_amplitude = 0.8;
        env.release_time = 1.0;
        Self { volume: 1.0, env }
    }
}

impl Instrument for Bell8 {
    fn sound(&self, dt: FreqType, note: &Note) -> (FreqType, bool) {
        let amplitude = self.env.amplitude(dt, note.on, note.off);
        let finished = amplitude <= 0.0;
        let dt = note.on - dt;
        let waves = vec![
            osc(dt, scale(note.id + 12, 0), WaveType::Sine, 5.0, 0.001),
            0.5 * osc(dt, scale(note.id + 24, 0), WaveType::Sine, 0.0, 0.0),
            0.25 * osc(dt, scale(note.id + 36, 0), WaveType::Sine, 0.0, 0.0),
        ];
        let sound = amplitude * waves.iter().sum::<FreqType>() * self.volume;
        (sound, finished)
    }
}

pub struct Harmonica {
    volume: FreqType,
    env: EnvelopeADSR,
}

impl Harmonica {
    #[allow(dead_code)]
    pub fn new() -> Self {
        let mut env = EnvelopeADSR::new();
        env.attack_time = 0.05;
        env.decay_time = 1.0;
        env.sustain_amplitude = 0.95;
        env.release_time = 0.1;
        Self { volume: 1.0, env }
    }
}

impl Instrument for Harmonica {
    fn sound(&self, dt: FreqType, note: &Note) -> (FreqType, bool) {
        let amplitude = self.env.amplitude(dt, note.on, note.off);
        let finished = amplitude <= 0.0;
        let dt = note.on - dt;
        let waves = vec![
            osc(dt, scale(note.id, 0), WaveType::Square, 5.0, 0.001),
            0.5 * osc(dt, scale(note.id + 12, 0), WaveType::Square, 0.0, 0.0),
            0.05 * osc(dt, scale(note.id + 24, 0), WaveType::Noise, 0.0, 0.0),
        ];
        let sound = amplitude * waves.iter().sum::<FreqType>() * self.volume;
        (sound, finished)
    }
}
