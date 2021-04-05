use anyhow::Result;
use device_query::{DeviceQuery, DeviceState, Keycode};
use instruments::{Default, InstrumentType};
use noise_maker::{NoiseMaker, NoiseMakerData, Note};
use rodio::{OutputStream, Sink};
use std::{
    io::Write,
    sync::{Arc, Mutex},
};

mod instruments;
mod noise_maker;

fn main() -> Result<()> {
    let instruments = vec![InstrumentType::from(Default::new())];
    let data = Arc::new(Mutex::new(NoiseMakerData::default()));

    let (_stream, stream_handle) = OutputStream::try_default()?;
    let sink = Sink::try_new(&stream_handle)?;
    sink.set_volume(0.2);
    sink.append(NoiseMaker::new(data.clone(), instruments));

    println!(
        r#"
        |   |   | |   |   |   |   | |   | |   |   |   |   | |   |   |
        |   | S | | D |   |   | G | | H | | J |   |   | L | | ; |   |
        |   |___| |___|   |   |___| |___| |___|   |   |___| |___|   |
        |     |     |     |     |     |     |     |     |     |     |
        |  Z  |  X  |  C  |  V  |  B  |  N  |  M  |  ,  |  .  |  /  |
        |_____|_____|_____|_____|_____|_____|_____|_____|_____|_____|
        "#
    );

    loop {
        let device_state = DeviceState::new();
        let keys = device_state.get_keys();

        for k in 0..=16 {
            let is_pressed = is_key_pressed(k, &keys);

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
                                instrument_id: 0,
                                active: true,
                            };
                            data.notes.push(note);
                        }
                    }
                }
                print!("\rNotes: {}  ", data.notes.len());
                std::io::stdout().flush()?;
            }
        }

        if keys.contains(&Keycode::Escape) {
            break;
        }

        sink.play();
    }
    Ok(())
}

fn is_key_pressed(key_id: i32, keys: &[Keycode]) -> bool {
    match key_id {
        0 if keys.contains(&Keycode::Z) => true,
        1 if keys.contains(&Keycode::S) => true,
        2 if keys.contains(&Keycode::X) => true,
        3 if keys.contains(&Keycode::D) => true,
        4 if keys.contains(&Keycode::C) => true,
        5 if keys.contains(&Keycode::V) => true,
        6 if keys.contains(&Keycode::G) => true,
        7 if keys.contains(&Keycode::B) => true,
        8 if keys.contains(&Keycode::H) => true,
        9 if keys.contains(&Keycode::N) => true,
        10 if keys.contains(&Keycode::J) => true,
        11 if keys.contains(&Keycode::M) => true,
        12 if keys.contains(&Keycode::Comma) => true,
        13 if keys.contains(&Keycode::L) => true,
        14 if keys.contains(&Keycode::Dot) => true,
        15 if keys.contains(&Keycode::Semicolon) => true,
        16 if keys.contains(&Keycode::Slash) => true,
        _ => false,
    }
}
