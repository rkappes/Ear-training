extern crate rust_music_theory as rustmt;
use ear_training::intervals; 
use ear_training::notes;
use ear_training::play;
use ear_training::chord;
use rustmt::note::Notes;
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

    // let chord = chord::create_chord("C, E, G");
    // println!("Chord is {:?}", chord);

    // let rand_chord = chord::rand_chord();
    // println!("Chord is {:?}", rand_chord);

    let btree = notes::create_note_mappings();
    println!("{:?}", btree);

    // let freq = btree.get("A#");
    // match freq {
    //     Some(freq) => println!("freq for A# is {}", freq),
    //     None => println!("Key not found"),
    // }

    loop{
        let mut root = String::new();
        let mut interval = String::new();
        let mut chord = String::new();
        let mut choice = String::new();
        let mut res = String::new();
        

        println!("Welcome to the Ear-training tool.");
        println!("Select an option below by entering the corresponding number");
        println!("1. Random interval - Guess the interval");
        println!("2. Random Interval from given note - Guess 2nd note");
        println!("3. Given interval from random note - ");
        println!("4. Randomly invert a given chord - Guess the inversion");
        println!("5. Random chord - guess the chord type");

        io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

        println!("choice is {}", choice);
        if choice.trim() == "1"{
            //Guess a random interval
            let root = notes::rand_note();
            let (note2, interval) = intervals::create_note_from_rand_interval(root.clone());

            let root_string = notes::get_note_letter(root.clone());
            let note2_string = notes::get_note_letter(note2);

            let root_hz = notes::get_hz(&root_string, &btree);
            play::play_note(root_hz);

            let note2_hz = notes::get_hz(&note2_string, &btree);
            play::play_note(note2_hz);

            let answer =  format!("{}{}",interval.quality, interval.number);
            println!("Interval was {}", answer);
        }
        else if choice.trim() == "2" {
            //Guess 2nd note from random interval applied
            println!("Enter the note you want as the root. Ex. A, D#, Bb..etc");
            io::stdin()
            .read_line(&mut root)
            .expect("Failed to read line");

            let root: &str = &root.trim();
            let note1= notes::create_note(root);
            let (note2, interval) = intervals::create_note_from_rand_interval(note1.clone());
            let note2_string = notes::get_note_letter(note2);

            let root_string = notes::get_note_letter(note1.clone());
            let root_hz = notes::get_hz(&root_string, &btree);
            play::play_note(root_hz);

            let note2_hz = notes::get_hz(&note2_string.trim(), &btree);
            play::play_note(note2_hz);

            let answer =  format!("{}{}",interval.quality, interval.number);
            println!("Interval was {}, 2nd note was {}", answer, note2_string);

        }
        else if choice.trim() == "3"{
            println!("Enter the interval you want to use. Ex. M3, P5, m6..etc");
            io::stdin()
            .read_line(&mut interval)
            .expect("Failed to read line");

            let root = notes::rand_note();
            let root_string = notes::get_note_letter(root.clone());

            let interval_type = intervals::create_interval_string(&interval.trim());
            let note2 = intervals::create_note_from_given_interval(root, interval_type, &mut String::from("Up"));

            let note2_string = notes::get_note_letter(note2);

            let root_hz = notes::get_hz(&root_string, &btree);
            play::play_note(root_hz);

            let note2_hz = notes::get_hz(&note2_string, &btree);
            play::play_note(note2_hz);
        }
        else if choice.trim() == "4"{
            //Guess the inversion
            println!("Enter the notes you want to use in the chord. Ex. 'C, E, G'");
            io::stdin()
            .read_line(&mut chord)
            .expect("Failed to read line");

            let chord_type = chord::create_chord(&chord);
            let chord_inverted = chord::rand_inversion(chord_type);
            println!("chord after inversion is {:?}", chord_inverted);
            let notes_in_chord = chord_inverted.notes();

            for note in notes_in_chord.into_iter() {
                let letter = notes::get_note_letter(note);
                println!("note letter is {}", letter);

                // let note_hz = notes::get_hz(&letter, &btree);
                // play::play_note(note_hz);
            }
            println!("inversion is {}", chord_inverted.inversion);
        }
        else if choice.trim() == "5"{
            //Guess the chord type
            let chord = chord::rand_chord();
            let notes_in_chord: Vec<rustmt::note::Note> = chord.notes();

            for note in notes_in_chord.into_iter() {
                let letter = notes::get_note_letter(note);
                println!("note letter is {}", letter);
                // let note_hz = notes::get_hz(&letter, &btree);
                // play::play_note(note_hz);
            }

            let answer =  format!("{}{}",chord.quality, chord.number);
            println!("Chord was {}", answer);

        }
        else{
            println!("Choice not valid")
        }

        println!("Again? y/n");
        io::stdin()
        .read_line(&mut res)
        .expect("Failed to read line");

        println!("res is {}", res);
        if res.trim() == "n"{
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