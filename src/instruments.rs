use crate::{
    noise_maker::{osc, EnvelopeADSR, FreqType, Note as NoiseMakerNote, WaveType},
    note::Note,
};
use enum_dispatch::enum_dispatch;

#[enum_dispatch]
pub trait Instrument {
    fn play_note(&self, dt: FreqType, note: &NoiseMakerNote) -> (FreqType, bool) {
        let amplitude = self.envelope().amplitude(dt, note.on, note.off);
        let finished = self.max_lifetime() > 0.0 && dt - note.on >= self.max_lifetime();
        let dt = note.on - dt;
        let oscillators = self
            .oscillators()
            .iter()
            .map(|config| {
                config.weight
                    * osc(
                        dt,
                        Note::from((note.id as i8 + config.note_offset) as u8).freq(),
                        config.wave,
                        config.lfo_hertz,
                        config.lfo_amplitude,
                    )
            })
            .sum::<FreqType>();
        let sound = amplitude * oscillators * self.volume();
        (sound, finished)
    }

    fn oscillators(&self) -> Vec<OscillatorConfig> {
        vec![OscillatorConfig::default()]
    }

    fn envelope(&self) -> EnvelopeADSR {
        EnvelopeADSR::default()
    }

    fn volume(&self) -> FreqType {
        1.0
    }

    fn max_lifetime(&self) -> FreqType {
        1.0
    }
}

#[enum_dispatch(Instrument)]
pub enum InstrumentType {
    Default,
    Bell,
    Bell8,
    Harmonica,
    DrumKick,
}

#[derive(Clone, Copy)]
pub struct OscillatorConfig {
    weight: FreqType,
    note_offset: i8,
    wave: WaveType,
    lfo_hertz: FreqType,
    lfo_amplitude: FreqType,
}

impl std::default::Default for OscillatorConfig {
    fn default() -> Self {
        Self {
            weight: 1.0,
            note_offset: 0,
            wave: WaveType::Sine,
            lfo_hertz: 0.0,
            lfo_amplitude: 0.0,
        }
    }
}

pub struct Default {}

impl Default {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {}
    }
}

impl Instrument for Default {}

pub struct Bell {
    oscillators: Vec<OscillatorConfig>,
    env: EnvelopeADSR,
}

impl Bell {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            env: EnvelopeADSR {
                attack_time: 0.01,
                decay_time: 1.0,
                sustain_amplitude: 0.0,
                release_time: 1.0,
                ..EnvelopeADSR::default()
            },
            oscillators: vec![
                OscillatorConfig {
                    weight: 1.0,
                    note_offset: 12,
                    wave: WaveType::Sine,
                    lfo_amplitude: 5.0,
                    lfo_hertz: 0.001,
                },
                OscillatorConfig {
                    weight: 0.5,
                    note_offset: 24,
                    ..OscillatorConfig::default()
                },
                OscillatorConfig {
                    weight: 0.25,
                    note_offset: 36,
                    ..OscillatorConfig::default()
                },
            ],
        }
    }
}

impl Instrument for Bell {
    fn oscillators(&self) -> Vec<OscillatorConfig> {
        self.oscillators.clone()
    }

    fn envelope(&self) -> EnvelopeADSR {
        self.env
    }
}

pub struct Bell8 {
    oscillators: Vec<OscillatorConfig>,
    env: EnvelopeADSR,
}

impl Bell8 {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            env: EnvelopeADSR {
                attack_time: 0.01,
                decay_time: 1.0,
                sustain_amplitude: 0.8,
                release_time: 1.0,
                ..EnvelopeADSR::default()
            },
            oscillators: vec![
                OscillatorConfig {
                    weight: 1.0,
                    note_offset: 12,
                    wave: WaveType::Sine,
                    lfo_amplitude: 5.0,
                    lfo_hertz: 0.001,
                },
                OscillatorConfig {
                    weight: 0.5,
                    note_offset: 24,
                    ..OscillatorConfig::default()
                },
                OscillatorConfig {
                    weight: 0.25,
                    note_offset: 36,
                    ..OscillatorConfig::default()
                },
            ],
        }
    }
}

impl Instrument for Bell8 {
    fn oscillators(&self) -> Vec<OscillatorConfig> {
        self.oscillators.clone()
    }

    fn envelope(&self) -> EnvelopeADSR {
        self.env
    }
}

pub struct Harmonica {
    oscillators: Vec<OscillatorConfig>,
    env: EnvelopeADSR,
}

impl Harmonica {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            env: EnvelopeADSR {
                attack_time: 0.05,
                decay_time: 1.0,
                sustain_amplitude: 0.95,
                release_time: 0.1,
                ..EnvelopeADSR::default()
            },
            oscillators: vec![
                OscillatorConfig {
                    weight: 1.0,
                    note_offset: 0,
                    wave: WaveType::Square,
                    lfo_amplitude: 5.0,
                    lfo_hertz: 0.001,
                },
                OscillatorConfig {
                    weight: 0.5,
                    note_offset: 12,
                    wave: WaveType::Square,
                    ..OscillatorConfig::default()
                },
                OscillatorConfig {
                    weight: 0.25,
                    note_offset: 24,
                    wave: WaveType::Noise,
                    ..OscillatorConfig::default()
                },
            ],
        }
    }
}

impl Instrument for Harmonica {
    fn oscillators(&self) -> Vec<OscillatorConfig> {
        self.oscillators.clone()
    }

    fn envelope(&self) -> EnvelopeADSR {
        self.env
    }
}

pub struct DrumKick {
    oscillators: Vec<OscillatorConfig>,
    env: EnvelopeADSR,
    max_lifetime: FreqType,
}

impl DrumKick {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            env: EnvelopeADSR {
                attack_time: 0.01,
                decay_time: 0.15,
                sustain_amplitude: 0.0,
                release_time: 0.0,
                ..EnvelopeADSR::default()
            },
            max_lifetime: 1.5,
            oscillators: vec![
                OscillatorConfig {
                    weight: 0.99,
                    note_offset: -36,
                    wave: WaveType::Sine,
                    lfo_amplitude: 1.0,
                    lfo_hertz: 1.0,
                },
                OscillatorConfig {
                    weight: 0.01,
                    note_offset: 0,
                    wave: WaveType::Noise,
                    ..OscillatorConfig::default()
                },
            ],
        }
    }
}

impl Instrument for DrumKick {
    fn oscillators(&self) -> Vec<OscillatorConfig> {
        self.oscillators.clone()
    }

    fn envelope(&self) -> EnvelopeADSR {
        self.env
    }

    fn max_lifetime(&self) -> FreqType {
        self.max_lifetime
    }
}
