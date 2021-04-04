use device_query::{DeviceQuery, DeviceState, Keycode};
use noise_maker::{Freq, NoiseMaker, NoiseMakerData};
use rodio::{source::Source, OutputStream, Sink};
use std::io::Write;
use std::{
    sync::{Arc, Mutex},
    thread,
};

mod noise_maker;

fn main() {
    let data = Arc::new(Mutex::new(NoiseMakerData::default()));

    {
        let data = data.clone();
        thread::spawn(move || {
            let (_stream, stream_handle) = OutputStream::try_default().unwrap();
            let sink = Sink::try_new(&stream_handle).unwrap();
            let source = NoiseMaker::new(data).amplify(0.10);
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

    let octave_base_freq: Freq = 110.0; // A2		        // frequency of octave represented by keyboard
    let _12th_root_of_2: Freq = 2.0_f64.powf(1.0 / 12.0); // assuming western 12 notes per ocatve

    let mut curr_key = -1;

    loop {
        let device_state = DeviceState::new();
        let keys = device_state.get_keys();

        let mut key_pressed = false;
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
            if is_pressed {
                if curr_key != k {
                    if let Ok(mut data) = data.lock() {
                        data.freq = octave_base_freq * _12th_root_of_2.powf(k as Freq);
                        let dt = data.dt;
                        data.envelope.note_on(dt);
                        print!("\rNote On: {}s {}Hz            ", dt, data.freq);
                        std::io::stdout().flush().unwrap();
                        curr_key = k;
                    }
                }
                key_pressed = true;
            }
        }

        if !key_pressed {
            if let Ok(mut data) = data.lock() {
                if curr_key != -1 {
                    let dt = data.dt;
                    data.envelope.note_off(dt);
                    print!("\rNote Off: {}s                 ", dt);
                    std::io::stdout().flush().unwrap();
                    curr_key = -1;
                }
                // data.freq = 0.0;
            }
        }

        if keys.contains(&Keycode::Escape) {
            break;
        }
    }
}
