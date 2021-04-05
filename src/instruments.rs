use crate::noise_maker::{osc, scale, EnvelopeADSR, FreqType, Note, WaveType};
use enum_dispatch::enum_dispatch;

#[enum_dispatch]
pub trait Instrument {
    fn play_note(&self, dt: FreqType, note: &Note) -> (FreqType, bool) {
        let amplitude = self.envelope().amplitude(dt, note.on, note.off);
        let finished = self.max_lifetime() > 0.0 && dt - note.on >= self.max_lifetime();
        let dt = note.on - dt;
        let waves = self
            .waves()
            .iter()
            .map(|wave| {
                wave.weight
                    * osc(
                        dt,
                        scale(note.id + wave.note_offset, 0),
                        wave.wave_type,
                        wave.lfo_hertz,
                        wave.lfo_amplitude,
                    )
            })
            .sum::<FreqType>();
        let sound = amplitude * waves * self.volume();
        (sound, finished)
    }

    fn waves(&self) -> Vec<WaveData> {
        vec![WaveData::default()]
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
}

#[derive(Clone, Copy)]
pub struct WaveData {
    weight: FreqType,
    note_offset: i32,
    wave_type: WaveType,
    lfo_hertz: FreqType,
    lfo_amplitude: FreqType,
}

impl std::default::Default for WaveData {
    fn default() -> Self {
        Self {
            weight: 1.0,
            note_offset: 0,
            wave_type: WaveType::Sine,
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
    waves: Vec<WaveData>,
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
            waves: vec![
                WaveData {
                    weight: 1.0,
                    note_offset: 12,
                    wave_type: WaveType::Sine,
                    lfo_amplitude: 5.0,
                    lfo_hertz: 0.001,
                },
                WaveData {
                    weight: 0.5,
                    note_offset: 24,
                    ..WaveData::default()
                },
                WaveData {
                    weight: 0.25,
                    note_offset: 36,
                    ..WaveData::default()
                },
            ],
        }
    }
}

impl Instrument for Bell {
    fn waves(&self) -> Vec<WaveData> {
        self.waves.clone()
    }

    fn envelope(&self) -> EnvelopeADSR {
        self.env
    }
}

pub struct Bell8 {
    waves: Vec<WaveData>,
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
            waves: vec![
                WaveData {
                    weight: 1.0,
                    note_offset: 12,
                    wave_type: WaveType::Sine,
                    lfo_amplitude: 5.0,
                    lfo_hertz: 0.001,
                },
                WaveData {
                    weight: 0.5,
                    note_offset: 24,
                    ..WaveData::default()
                },
                WaveData {
                    weight: 0.25,
                    note_offset: 36,
                    ..WaveData::default()
                },
            ],
        }
    }
}

impl Instrument for Bell8 {
    fn waves(&self) -> Vec<WaveData> {
        self.waves.clone()
    }

    fn envelope(&self) -> EnvelopeADSR {
        self.env
    }
}

pub struct Harmonica {
    waves: Vec<WaveData>,
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
            waves: vec![
                WaveData {
                    weight: 1.0,
                    note_offset: 0,
                    wave_type: WaveType::Square,
                    lfo_amplitude: 5.0,
                    lfo_hertz: 0.001,
                },
                WaveData {
                    weight: 0.5,
                    note_offset: 12,
                    wave_type: WaveType::Square,
                    ..WaveData::default()
                },
                WaveData {
                    weight: 0.25,
                    note_offset: 24,
                    wave_type: WaveType::Noise,
                    ..WaveData::default()
                },
            ],
        }
    }
}

impl Instrument for Harmonica {
    fn waves(&self) -> Vec<WaveData> {
        self.waves.clone()
    }

    fn envelope(&self) -> EnvelopeADSR {
        self.env
    }
}
