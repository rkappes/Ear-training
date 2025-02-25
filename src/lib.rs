pub mod notes{
    extern crate rust_music_theory as rustmt;
    use rustmt::note::{Note,Pitch, NoteLetter};

    pub fn create_note(letter: &str) -> Note {
        println!("In create_note, letter is {}", letter);
        let result = Pitch::from_str(letter.trim());
        println!("result from_str is {:?}", result);
        match result {
            Some(pitch) => {
                Note::new(pitch, 4)
            },
            None => {
                println!("Pitch not found from {}", letter);
                Note::new(Pitch::new(NoteLetter::A,0),4)
            },
        }
        // let pitch = Pitch::new(NoteLetter::A,0);
        // let note = Note::new(pitch, 4);
        // note
    }

    pub fn rand_note() -> Note {
        let note_letter = ["C", "D", "E", "F", "G", "A", "B"];
        let accidental = ["s", "x", "b"];
        let rand_letter = note_letter[rand::random_range(..note_letter.len())];
        let rand_accidental = accidental[rand::random_range(..accidental.len())];

        let note = format!("{}{}",rand_letter, rand_accidental);
        // println!("Note created is {}", note);

        create_note(&note)
    }
}
pub mod intervals {
    extern crate rust_music_theory as rustmt;
    use rustmt::note::Note;
    use rustmt::interval::{Interval, IntervalError, Number, Quality};


    pub fn rand_quality() -> Quality {
        let qualities = [Quality::Major, Quality::Minor, Quality::Perfect, Quality::Augmented, Quality::Diminished];
        let quality = qualities[rand::random_range(..qualities.len())];
        quality
    }

    pub fn rand_interval() -> Number {
        let intervals = [Number::Unison, Number::Second, Number::Third, Number::Fourth, Number::Fifth, Number::Sixth, Number::Seventh, Number::Octave];
        let interval = intervals[rand::random_range(..intervals.len())];
        interval
    }

    pub fn rand_semitone() -> u8 {
        rand::random_range(0..12)
    }

    //pub fn create_interval(quality: &str, semitone: u8, interval: i32) -> Interval{
    pub fn create_interval_semitones(semitones: u8) -> Interval{

        let interval_result = Interval::from_semitone(semitones);
        let interval = match interval_result {
            Ok(interval) => interval,
            Err(_error) => Interval::default(),
        };
    /* 
        let q = match quality{
            "perfect" => Quality::Perfect,
            "major" => Quality::Major,
            "minor"=> Quality::Minor,
            "augmented"=> Quality::Augmented,
            "diminished"=> Quality::Diminished,
            &_=> Quality::Perfect,
        };

        let int = match interval{
            1 => Number::Unison,
            2 => Number::Second,
            3 => Number::Third,
            4 => Number::Fourth,
            5 => Number::Fifth,
            6 => Number::Sixth,
            7 => Number::Seventh,
            8 => Number::Octave,
            _ => Number::Unison,
        };

        let interval = Interval::new(semitone,q, int, None);
    */
        println!("Interval is {:?}", interval);

        interval
    }

    pub fn create_interval_string(interval: &str) -> Interval{
        println!("In create_Interval_string, interval is {}", interval.trim());
        let semitones: u8;
        match interval {
            "unison" => semitones = 0,
            "m2" => semitones = 1,
            "M2" => semitones =2,
            "m3" => semitones =3,
            "M3" => semitones =4,
            "P4" => semitones =5,
            "D5" => semitones =6,
            "P5" => semitones =7,
            "m6" => semitones =8,
            "M6" => semitones =9,
            "m7" => semitones =10,
            "M7" => semitones =11,
            "octave" => semitones =12,
            _=> semitones = 0 //TODO: Return an error or do something else?
        }

        create_interval_semitones(semitones)
    }
    pub fn create_rand_interval() -> Result<Interval, IntervalError> {
        let semitone = rand_semitone();
        let interval_result = Interval::from_semitone(semitone);
        interval_result
    }

    pub fn create_note_from_rand_interval(root: Note) -> Note {
        let interval_result = create_rand_interval();
        let interval = match interval_result {
            Ok(interval) => interval,
            Err(_error) => Interval::default(),
        };

        println!("Random Interval is {:?}", interval);
        let new_note = interval.second_note_from(root);
        new_note
    }

    pub fn create_note_from_given_interval(root: Note, interval: Interval, direction: &mut String) -> Note {
        direction.make_ascii_lowercase();
        let note =  if direction == "down" {
            interval.second_note_down_from(root)
        }
        else{
            interval.second_note_from(root)
        };
        note
    }
}

pub mod play {
    extern crate rodio;
    use std::error::Error;

    //Taken from: https://github.com/RustAudio/rodio/blob/master/examples/signal_generator.rs with some minor changes
    fn play_note(freq: f32) -> Result<(), Box<dyn Error>> {
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
}