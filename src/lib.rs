pub mod notes{
    extern crate rust_music_theory as rustmt;
    use rustmt::note::{Note,Pitch, NoteLetter};
    use std::collections::BTreeMap;

    pub fn create_note_mappings() -> BTreeMap<String,f64>{
        let mut NoteHz: BTreeMap<String, f64> = BTreeMap::new();

        let notes = [
            ("C", 261.63),
            ("C#", 277.18),
            ("Db", 277.18),
            ("D", 293.66),
            ("D#", 311.13),
            ("Eb", 311.13),
            ("E", 329.63),
            ("F", 349.63),
            ("F#", 369.99),
            ("Gb", 369.99),
            ("G", 392.00),
            ("G#", 415.30),
            ("Ab", 415.30),
            ("A", 440.00),
            ("A#", 466.16),
            ("Bb", 466.16),
            ("B", 493.88)
        ];

        for(note, freq) in &notes {
            NoteHz.insert(note.to_string(), *freq);
        }
        NoteHz
    }

    /// Creates a note given a letter representation of the note
    /// Ex A, D#, Bb...etc.
    /// In rust-music-theory crate, Notes are created with a Pitch Type
    /// This function first creates a Pitch from the letter representation
    /// and then uses that to create a Note type.
    /// ### Parameters
    /// - letter: A &str of a note name
    /// ### Returns
    /// - A Note type
    pub fn create_note(letter: &str) -> Note {
        println!("In create_note, letter is {}", letter);
        let result = Pitch::from_str(letter.trim()); //NOTE: use impl FromStr instead?
        println!("result from_str is {:?}", result);
        match result {
            Some(pitch) => {
                Note::new(pitch, 4) //NOTE: randomize octave?
            },
            None => {
                println!("Pitch not found from {}", letter);
                //TODO: Return None instead
                Note::new(Pitch::new(NoteLetter::A,0),4)
            },
        }
    }

    /// Creates a random note
    /// Using the 7 possible note letters, and 3 possible accidentals (as used in rust-music-theory crate)
    /// Randomly selects a combo of the two to create a Note
    /// ### Returns
    /// - A random Note Type
    pub fn rand_note() -> Note {
        let note_letter = ["C", "D", "E", "F", "G", "A", "B"];
        let accidental = ["s", "x", "b"];
        let rand_letter = note_letter[rand::random_range(..note_letter.len())];
        let rand_accidental = accidental[rand::random_range(..accidental.len())];

        let note = format!("{}{}",rand_letter, rand_accidental);
        // println!("Note created is {}", note);

        create_note(&note)
    }

    /// Creates the string equilavent of a Note
    /// # Parameters
    /// - a note of type Note
    /// # Returns
    /// - a string representation of that note. 
    pub fn get_note_letter(note: Note) -> String {
        let pitch = note.pitch; 
        let letter = pitch.letter;
        let accidental = pitch.accidental;
        let char_letter: char;
        let char_accidental: char;
        match letter {
            NoteLetter::A => char_letter = 'A',
            NoteLetter::B => char_letter ='B',
            NoteLetter::C => char_letter ='C',
            NoteLetter::D => char_letter ='D',
            NoteLetter::E => char_letter ='E',
            NoteLetter::F => char_letter ='F',
            NoteLetter::G => char_letter ='G',
        }

        match accidental {
            1 => char_accidental='#',
            2 => char_accidental='x',
            -1 => char_accidental='b',
            _=> char_accidental = '\0'
        }

        let mut result = String::new();  // Create an empty String.
        result.push(char_letter);              // Add the first character.
        result.push(char_accidental);   

        result
    }
}
pub mod intervals {
    extern crate rust_music_theory as rustmt;
    use rustmt::note::Note;
    use rustmt::interval::{Interval, IntervalError, Number, Quality};

    /// Returns a random Quality type, using rust-music-theory Quality enum.
    pub fn rand_quality() -> Quality {
        let qualities = [Quality::Major, Quality::Minor, Quality::Perfect, Quality::Augmented, Quality::Diminished];
        let quality = qualities[rand::random_range(..qualities.len())];
        quality
    }

    /// Returns a random interval(2nd, 4th, 6th..etc), using rust-music-theory Number enum.
    pub fn rand_interval() -> Number {
        let intervals = [Number::Unison, Number::Second, Number::Third, Number::Fourth, Number::Fifth, Number::Sixth, Number::Seventh, Number::Octave];
        let interval = intervals[rand::random_range(..intervals.len())];
        interval
    }

    /// Returns a random semitone number (0-12)
    pub fn rand_semitone() -> u8 {
        rand::random_range(0..12)
    }

    /// Creates an interval given the number of semitones
    /// ### Returns
    /// - An Interval type
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

    /// Creates an interval given a string value: m3, M6, P5..etc
    /// Matches the interval to the number of semitones and calls 
    /// create_interval_semitones
    /// ### Parameters:
    /// - a &str of the desired interval
    /// ### Returns
    /// - Interval type 
    pub fn create_interval_string(interval: &str) -> Interval{
        println!("In create_Interval_string, interval is {}", interval);
        let semitones: u8;
        match interval.trim() {
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

    /// Creates a random interval by randomly choosing a semitone value (0-12)
    /// and calling rust-music-theory crate's Interval::from_semitone
    /// ### Returns
    /// - a Result type
    pub fn create_rand_interval() -> Result<Interval, IntervalError> {
        let semitone = rand_semitone();
        let interval_result = Interval::from_semitone(semitone);
        interval_result
    }

    ///Creates a note that is a random interval above a given note
    /// ### Parameters:
    /// - A note value (the root) as a Note type 
    /// ### Returns:
    /// - A Note type (the 2nd note)
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

    /// Creates a note that is a given interval above or below a given note
    /// ### Parameters:
    /// - A note value (the root), of type Note 
    /// - an Interval, of type Interval
    /// - A direction (up or down)
    /// ### Returns:
    /// - A Note type (the 2nd note)
    pub fn create_note_from_given_interval(root: Note, interval: Interval, direction: &mut String) -> Note {
        direction.make_ascii_lowercase();
        //TODO: add error checking for if direciton is not valid?
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


    // fn convert_to_hz(pitch: String) -> f32{

    // }

    //Taken from: https://github.com/RustAudio/rodio/blob/master/examples/signal_generator.rs with some minor changes
    pub fn play_note(freq: f32) -> Result<(), Box<dyn Error>> {
        use rodio::source::{chirp, Function, SignalGenerator, Source};
        use std::thread;
        use std::time::Duration;

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
}