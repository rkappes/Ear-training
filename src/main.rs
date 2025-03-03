use ear_training::intervals; 
use ear_training::notes;
use ear_training::play;
use std::io;
// extern crate rodio;
// use std::error::Error;

fn main() {
    // let pitch = Pitch::new(NoteLetter::A,0);
    // let note1 = Note::new(pitch, 4);
    // let note2 = Note::new(pitch, 4);
    // let interval = Interval::new(semitone, quality, num, None);
    // let interval_result = create_rand_interval();
    // let interval = match interval_result {
    //     Ok(interval) => interval,
    //     Err(_error) => Interval::default(),
    // };
    // let rand_note = notes::rand_note();
    // println!("Rand note is {}", rand_note);

    let btree = notes::create_note_mappings();
    println!("{:?}", btree);

    // let freq = btree.get("A#");
    // match freq {
    //     Some(freq) => println!("freq for A# is {}", freq),
    //     None => println!("Key not found"),
    // }

    loop{
        let mut root = String::new();
        let mut choice = String::new();
        let mut res = String::new();

        println!("Welcome to the Ear-training tool.");
        println!("Select an option below by entering the corresponding number");
        println!("1. Random interval");
        println!("2. Random Interval from given note");
        println!("3. Given interval from random note");

        io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

        if choice == "1"{
            let root = notes::rand_note();
            let note2 = intervals::create_note_from_rand_interval(root.clone());

            let root_string = notes::get_note_letter(root.clone());
            let note2_string = notes::get_note_letter(note2);

            let root_hz = notes::get_hz(&root_string, &btree);
            play::play_note(root_hz);

            let note2_hz = notes::get_hz(&note2_string, &btree);
            play::play_note(note2_hz);
        }
        else if choice == "2" {
            println!("Enter the note you want as the root. Ex. A, D#, Bb..etc");
            io::stdin()
            .read_line(&mut root)
            .expect("Failed to read line");

            let root: &str = &root;
            let note1= notes::create_note(root);
            let note2 = intervals::create_note_from_rand_interval(note1);
            let note2_string = notes::get_note_letter(note2);

            // let mut freq = btree.get(root);
            // let root_hz: f32;
            // match freq {
            //     Some(freq) => {
            //         println!("freq for {} is {}", root, freq);
            //         root_hz = *freq as f32;
            //         play::play_note(root_hz);
            //     }
            //     None => println!("Key not found"),
            // }
            let root_hz = notes::get_hz(root, &btree);
            play::play_note(root_hz);

            let note2_hz = notes::get_hz(&note2_string, &btree);
            play::play_note(note2_hz);

            // freq = btree.get(&note2_string);
            // match freq {
            //     Some(freq) => {
            //         println!("freq for {} is {}", note2_string, freq);
            //         note2_hz = *freq as f32;
            //         play::play_note(note2_hz);
            //     }
            //     None => println!("Key not found"),
            // }
        }
        else{
            println!("Choice not valid")
        }

        println!("Again? y/n");
        io::stdin()
        .read_line(&mut res)
        .expect("Failed to read line");
        if res == "n"{
            break;
        }
    }

    // println!("Enter a pitch:");
    // let mut root=String::new();

    // io::stdin()
    //     .read_line(&mut root)
    //     .expect("Failed to read line");

    // println!("Entered note: {}", root);
    // let root: &str = &root;
    // let note1: rust_music_theory::note::Note = notes::create_note(root);
    // // // let note1 = Pitch::from_str("D#");
    // // // let note2 = notes::create_note("A");

    // println!("Note is {:?}", note1);

    // let note_string = notes::get_note_letter(note1);
    // println!("Note string is {}", note_string);


    // let freq = btree.get(&note_string);
    // let Hz: f32;
    // match freq {
    //     Some(freq) => {
    //         println!("freq is {}",freq);
    //         Hz = *freq as f32;
    //         play::play_note(Hz);
    //     }
    //     None => println!("Key not found"),
    // }
    // println!("Interval is {:?}", interval);

    // let new_note = intervals::create_note_from_rand_interval(note1); //interval.second_note_from(note);
    // println!("new_note is {}", new_note);

    // println!("Enter an interval:");
    // let mut interval = String::new();

    // io::stdin()
    // .read_line(&mut interval)
    // .expect("Failed to read line");

    // println!("Entered interval: {}", interval);

    // let interval: &str = &interval;
    // let interval = intervals::create_interval_string(interval);
    // println!("Interval created is {:?}", interval);

    // let mut direction = String::from("Down");
    // let new_note = intervals::create_note_from_given_interval(note2, interval, &mut direction); //interval.second_note_from(note);

    // println!("new_note is {}", new_note);
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