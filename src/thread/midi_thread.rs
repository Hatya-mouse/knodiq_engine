use crate::thread::audio_command::MidiCommand;
use midir::MidiInputPort;
use std::sync::mpsc;

pub(super) fn midi_thread(
    command_rx: mpsc::Receiver<MidiCommand>,
    midi_producer: ringbuf::Prod<MidiEvent>,
) {
    let mut midi_port: Option<MidiInputPort> = None;

    for command in command_rx {
        match command {
            MidiCommand::SetMidiPort(new_port) => {
                midi_port = Some(new_port);
            }
            MidiCommand::DisconnectMidiPort => {
                midi_port = None;
            }
        }
    }
}
