use device_query::{DeviceQuery, DeviceState, Keycode};
use instruments::{InstrumentType, Test};
use noise_maker::{NoiseMaker, NoiseMakerData, Note};
use rodio::{source::Source, OutputStream, Sink};
use std::io::Write;
use std::{
    sync::{Arc, Mutex},
    thread,
};

mod instruments;
mod noise_maker;

fn main() {
    let data = Arc::new(Mutex::new(NoiseMakerData::default()));

    {
        let data = data.clone();
        thread::spawn(move || {
            let instruments = vec![InstrumentType::from(Test::new())];
            let (_stream, stream_handle) = OutputStream::try_default().unwrap();
            let sink = Sink::try_new(&stream_handle).unwrap();
            let source = NoiseMaker::new(data, instruments).amplify(0.20);
            sink.append(source);
            sink.sleep_until_end();
        });
    }

    println!(
        r#"
        |   |   |   |   |   | |   |   |   |   | |   | |   |   |   |
        |   | S |   |   | F | | G |   |   | J | | K | | L |   |   |
        |   |___|   |   |___| |___|   |   |___| |___| |___|   |   |__
        |     |     |     |     |     |     |     |     |     |     |
        |  Z  |  X  |  C  |  V  |  B  |  N  |  M  |  ,  |  .  |  /  |
        |_____|_____|_____|_____|_____|_____|_____|_____|_____|_____|
        "#
    );

    loop {
        let device_state = DeviceState::new();
        let keys = device_state.get_keys();

        for k in 0..16 {
            let is_pressed = match k {
                0 if keys.contains(&Keycode::Z) => true,
                1 if keys.contains(&Keycode::S) => true,
                2 if keys.contains(&Keycode::X) => true,
                3 if keys.contains(&Keycode::C) => true,
                4 if keys.contains(&Keycode::F) => true,
                5 if keys.contains(&Keycode::V) => true,
                6 if keys.contains(&Keycode::G) => true,
                7 if keys.contains(&Keycode::B) => true,
                8 if keys.contains(&Keycode::N) => true,
                9 if keys.contains(&Keycode::J) => true,
                10 if keys.contains(&Keycode::M) => true,
                11 if keys.contains(&Keycode::K) => true,
                12 if keys.contains(&Keycode::Comma) => true,
                13 if keys.contains(&Keycode::L) => true,
                14 if keys.contains(&Keycode::Dot) => true,
                15 if keys.contains(&Keycode::Slash) => true,
                _ => false,
            };

            if let Ok(mut data) = data.lock() {
                let dt = data.dt;
                match data.notes.iter_mut().find(|n| n.id == k) {
                    Some(note) => {
                        if is_pressed {
                            if note.off > note.on {
                                note.on = dt;
                                note.active = true;
                            }
                        } else if note.off < note.on {
                            note.off = dt;
                        }
                    }
                    None => {
                        if is_pressed {
                            let note = Note {
                                id: k,
                                on: dt,
                                off: 0.0,
                                channel: 0,
                                active: true,
                            };
                            data.notes.push(note);
                        }
                    }
                }

                print!("\rNotes: {}  ", data.notes.len());
                std::io::stdout().flush().unwrap();
            }
        }

        if keys.contains(&Keycode::Escape) {
            break;
        }
    }
}
