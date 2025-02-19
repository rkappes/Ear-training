extern crate rust_music_theory as rustmt;
use rustmt::note::{Note,Pitch, NoteLetter};
use rustmt::interval::{Interval,Number, Quality};
extern crate rodio;
use std::error::Error;


fn main() {
    let pitch = Pitch::new(NoteLetter::A,0);
    let note = Note::new(pitch, 4);
    let interval = Interval::new(3,Quality::Minor, Number::Third, None);

    println!("Note is {}", note);
    println!("Interval is {}", interval);

    let new_note = interval.second_note_from(note);

    println!("new_note is {}", new_note);

}

/* 
 //Taken from: https://github.com/RustAudio/rodio/blob/master/examples/signal_generator.rs
fn main() -> Result<(), Box<dyn Error>> {
    use rodio::source::{chirp, Function, SignalGenerator, Source};
    use std::thread;
    use std::time::Duration;

    let (_stream, stream_handle) = rodio::OutputStream::try_default()?; //Builder::open_default_stream()?;

    let test_signal_duration = Duration::from_millis(1000);
    let interval_duration = Duration::from_millis(1500);
    let sample_rate = rodio::cpal::SampleRate(48000);

    println!("Playing 440 Hz tone");
    stream_handle.play_raw(
        SignalGenerator::new(sample_rate, 440.0, Function::Sine)
            .amplify(0.1)
            .take_duration(test_signal_duration),
    )?;

    thread::sleep(interval_duration);

    Ok(())
}
*/