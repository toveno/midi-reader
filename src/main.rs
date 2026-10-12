mod input;
use slint::ComponentHandle;
use slint::Model;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = MainWindow::new()?;
    let ui_handle = ui.as_weak();

    let _conn = input::fetch_ports(move |message: &[u8]| {
        if message.len() < 3 {
            return;
        }
        
        let status = message[0] & 0xF0; 
        // from my (very basic) understanding of midi, a 0x90 from message[0] is a note on (and 0x80 is off)
        // however we can have 0x91! this means the midi input did happen but it happened on channel 2!
        // we can prevent having a specific channel sending in the data and accepting all midi inputs across all channels
        // a way to achieve this is by AND-ING it with a 0xF0 (0x11110000) it'll now always give us a clean 0x90 no channel number in sight
        //I WOULD NEVER HAVE THOUGHT OF THAT IN A MILLION YEARS >:D
        let note = message[1];
        let velocity = message[2];
        let is_on = status == 0x90 && velocity > 0;
        let is_off = status == 0x80 || (status == 0x90 && velocity == 0);

        if !is_on && !is_off {
            return;
        }

        if let Some(key) = midi_note_to_key(note) {
            let ui_handle = ui_handle.clone();
            slint::invoke_from_event_loop(move || { //this will take in the data from the midi thread safely into the UI thread, thanks slint! that is cool
                if let Some(ui) = ui_handle.upgrade() {
                    let state = ui.global::<KeyboardState>();

                    match key {
                        KeyType::White(idx) => {
                            let keys = state.get_white_active_keys();
                            if let Some(vec) = keys.as_any().downcast_ref::<slint::VecModel<bool>>()
                            {
                                vec.set_row_data(idx, is_on);
                            }
                        }
                        KeyType::Black(idx) => {
                            let keys = state.get_black_active_keys();
                            if let Some(vec) = keys.as_any().downcast_ref::<slint::VecModel<bool>>()
                            {
                                vec.set_row_data(idx, is_on);
                            }
                        }
                        //KeyType::Sus(idx)
                    }
                }
            })
            .unwrap();
        }
    });
    ui.run()
}

enum KeyType {
    White(usize),
    Black(usize),
    //Sus(usize)
}

fn midi_note_to_key(note: u8) -> Option<KeyType> {
    if note < 21 || note > 108 {
        return None; // Outside 88-key range
    }

    // an index of key numbers starting from A0 which is 21 in midi
    let rel = (note - 21) as usize;
    let octave: usize = rel / 12;
    let note_in_octave = rel % 12;

    match note_in_octave {
        0 => Some(KeyType::White(octave * 7 + 0)),  // A
        1 => Some(KeyType::Black(octave * 5 + 0)),  // A#
        2 => Some(KeyType::White(octave * 7 + 1)),  // B
        3 => Some(KeyType::White(octave * 7 + 2)),  // C
        4 => Some(KeyType::Black(octave * 5 + 1)),  // C#
        5 => Some(KeyType::White(octave * 7 + 3)),  // D
        6 => Some(KeyType::Black(octave * 5 + 2)),  // D#
        7 => Some(KeyType::White(octave * 7 + 4)),  // E
        8 => Some(KeyType::White(octave * 7 + 5)),  // F
        9 => Some(KeyType::Black(octave * 5 + 3)),  // F#
        10 => Some(KeyType::White(octave * 7 + 6)), // G
        11 => Some(KeyType::Black(octave * 5 + 4)), // G#
        //12 => Some(KeyType::Sus(octave * 5 + 4)), // sustain pedal...WIP
        _ => None,
    }
}
