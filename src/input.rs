use midir::{MidiInput, MidiInputConnection};
use std::sync::{Arc, Mutex};
pub fn fetch_ports<F>(callback: F) -> Option<Vec<MidiInputConnection<()>>>
where
    F: FnMut(&[u8]) + Send + 'static,
{
    let instance = MidiInput::new("reading_midi").unwrap();
    let ports = instance.ports();

    if ports.is_empty() {
        println!("No MIDI ports found!");
        return None;
    }

    let callback = Arc::new(Mutex::new(callback));
    let mut connections = Vec::new();
    //this will loop through all the ports makes MidiInput instances for them
    //they all will clone the callback thread to use it and get stored into the connection variable
    for (i, p) in ports.iter().enumerate() {
        let port_instance = MidiInput::new(&format!("reading_midi_{}", i)).unwrap();
        println!("Port {}: {}", i, port_instance.port_name(p).unwrap());

        let cb = Arc::clone(&callback);
        if let Ok(conn) = port_instance.connect(
            p,
            &format!("main_{}", i),
            move |_timestamp, message, _| {
                if let Ok(mut cb) = cb.lock() {
                    cb(message);
                }
            },
            (),
        ) {
            connections.push(conn);
        }
    }

    if connections.is_empty() {
        None
    } else {
        println!("Successfully connected to all MIDI ports!");
        Some(connections)
    }
}

//{ this used to make me connect to the last port of the midi but it wasn't practical }

//pub fn connect_to_last<F>(
//    port: &MidiInputPort,
//    instance: MidiInput,
//    mut callback: F,
//) -> Option<MidiInputConnection<()>>
//where
//    F: FnMut(&[u8]) + Send + 'static,
//{
//    let conn = instance
//        .connect(
//            port,
//            "main",
//            move |_timestamp, message, _| {
//                callback(message);
//            },
//            (),
//        )
//        .ok()?;
//
//    println!("Successfully connected to MIDI port!");
//    Some(conn)
//}
