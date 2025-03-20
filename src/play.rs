extern crate rodio;
use rodio::source::{SineWave, Source};
use rodio::{dynamic_mixer, OutputStream, Sink};
use std::thread;
use std::time::Duration;

/// Plays the given pitches individually and together
/// Source: https://github.com/RustAudio/rodio/blob/f1eaaa4a6346933fc8a58d5fd1ace170946b3a94/examples/mix_multiple_sources.rs
/// Changes made to take an vec of note frequencies to play
/// ### Parameters
/// - A vec of f32 types which should be note frequencies
pub fn play_notes(notes: Vec<f32>){
    // Construct a dynamic controller and mixer, stream_handle, and sink.
    let (controller, mixer) = dynamic_mixer::mixer::<f32>(2, 44_100);
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    for i in notes{
        let new_source = SineWave::new(i)
        .take_duration(Duration::from_secs_f32(1.))
        .amplify(0.20);
        
        match stream_handle.play_raw(new_source.clone()) {
            Ok(_) => {
            }
            Err(e) =>{
                println!("Error playing note.{} ", e)
            }
        }

        let interval_duration = Duration::from_millis(1500);
        thread::sleep(interval_duration);
        controller.add(new_source);
    }

    // Append the dynamic mixer to the sink.
    sink.append(mixer);

    // Sleep the thread until sink is empty.
    sink.sleep_until_end();
}

// Function not used, but left here as it was used as reference for play_notes function above
// Source: https://github.com/RustAudio/rodio/blob/master/examples/signal_generator.rs 
/* 
fn play_note(freq: f32) -> Result<(), Box<dyn Error>> {
    let (_stream, stream_handle) = rodio::OutputStream::try_default()?; //Builder::open_default_stream()?;

    let test_signal_duration = Duration::from_millis(1000);
    let interval_duration = Duration::from_millis(1500);
    let sample_rate = rodio::cpal::SampleRate(48000);

    println!("Playing {} Hz tone", freq);
    stream_handle.play_raw(
        SignalGenerator::new(sample_rate, freq, Function::Sine)
            .amplify(0.1)
            .take_duration(test_signal_duration),
    )?;

    thread::sleep(interval_duration);

    Ok(())
}
*/

    
