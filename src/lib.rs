pub mod notes{
    extern crate rust_music_theory as rustmt;
    use rustmt::note::{Note,Pitch, NoteLetter};
    use std::collections::BTreeMap;

    pub fn create_note_mappings() -> BTreeMap<String,f64>{
        let mut NoteHz: BTreeMap<String, f64> = BTreeMap::new();

        let notes = [
            ("C4", 261.63),
            ("C#4", 277.18),
            ("Db4", 277.18),
            ("D4", 293.66),
            ("D#4", 311.13),
            ("Eb4", 311.13),
            ("E4", 329.63),
            ("F4", 349.63),
            ("F#4", 369.99),
            ("Gb4", 369.99),
            ("G4", 392.00),
            ("G#4", 415.30),
            ("Ab4", 415.30),
            ("A4", 440.00),
            ("A#4", 466.16),
            ("Bb4", 466.16),
            ("B4", 493.88),
            ("C5", 523.25),
            ("C#5", 554.37),
            ("Db5", 554.37),
            ("D5", 587.33),
            ("D#5", 622.25),
            ("Eb5", 622.25),
            ("E5", 659.25),
            ("F5", 698.46),
            ("F#5", 739.99),
            ("Gb5", 739.99),
            ("G5", 783.99),
            ("G#5", 830.61),
            ("Ab5", 830.61),
            ("A5", 880.00),
            ("A#5", 932.33),
            ("Bb5", 932.33),
            ("B5", 987.77)
        ];

        for(note, freq) in &notes {
            NoteHz.insert(note.to_string(), *freq);
        }
        NoteHz
    }


    pub fn get_hz(key: &str, btree: &BTreeMap<String,f64>) -> f32{

        //TODO: handle double sharps - Gx. 
        // If 'x' detected - remap key to actual note? Gx --> A?
        let mut freq = btree.get(key);
        let mut note_hz: f32 = 0.0;
        match freq {
            Some(freq) => {
                println!("freq for {} is {}", key, freq);
                note_hz = *freq as f32;
            }
            None => println!("Key not found"),
        }
        note_hz
}

    pub fn create_rand_pitch() -> Pitch{
        let integer = rand::random_range(0..12);
        Pitch::from_u8(integer)
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
        let octave = note.octave.to_string();
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

        let mut result = String::new(); 
        result.push(char_letter);  
        if char_accidental != '\0'{           
            result.push(char_accidental);  
        } 
        result.push_str(&octave);
        println!("note letter is {}", result);
        result
    }
}
pub mod intervals {
    extern crate rust_music_theory as rustmt;
    use std::string;

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
    pub fn create_note_from_rand_interval(root: Note) -> (Note, Interval) {
        let interval_result = create_rand_interval();
        let interval = match interval_result {
            Ok(interval) => interval,
            Err(_error) => Interval::default(),
        };

        println!("Random Interval is {:?}", interval);
        let new_note = interval.second_note_from(root);
        (new_note, interval)
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

pub mod chord{
    extern crate rust_music_theory as rustmt;
    use rustmt::chord::{Chord, Number, Quality};

    /// Creates an random Number value to be used in chord creation
    pub fn rand_number() -> Number{
        let numbers = [Number::Triad, Number::Seventh];
        let number = numbers[rand::random_range(..numbers.len())];
        number
    }

    /// Creates a random quality to be used in chord creation
    /// chord quality is limited to those for triads
    pub fn rand_quality_triad() -> Quality {
        let qualities = [Quality::Major, Quality::Minor, Quality::Diminished, Quality::Augmented]; 
        let quality = qualities[rand::random_range(..qualities.len())];
        quality
    }

    /// Creates a random quality to be used in chord creation
    /// chord quality is limited to those for seven chords
    pub fn rand_quality_seventh() -> Quality {
        let qualities = [Quality::Major, Quality::Minor, Quality::Diminished, Quality::Augmented, Quality::HalfDiminished,Quality::Dominant];
        let quality = qualities[rand::random_range(..qualities.len())];
        quality
    }

    // Creates a chord from a string - Ex "C, E, G"
    pub fn create_chord(string: & str) -> Chord{ //TODO: specify inversion?
        Chord::from_string(string)
    }

    /// Creates a random chord
    pub fn rand_chord() -> Chord{
        let number = rand_number();
        let quality = if number == Number::Triad{
            rand_quality_triad()
        }
        else{
            rand_quality_seventh()
        };
        
        let pitch = crate::notes::create_rand_pitch();

        println!("In rand_chord, pitch is {}, number is {} and quality is {}", pitch, number, quality);

        Chord::new(pitch, quality, number)
    } 

    /// Applies a random inversion to a given chord
    pub fn rand_inversion(chord: Chord) -> Chord {
        let root = chord.root;
        let quality = chord.quality;
        let number = chord.number;
        let inversion = rand::random_range(1..4);
        println!("In rand_inversion, inversion is {}", inversion);
        Chord::with_inversion(root, quality, number,inversion)
    }
}
pub mod play {
    extern crate rodio;
    use rodio::source::{Function, SignalGenerator, SineWave, Source};
    use rodio::{dynamic_mixer, OutputStream, Sink};
    use std::thread;
    use std::time::Duration;
    use std::error::Error;
    
    /// Taken from: https://github.com/RustAudio/rodio/blob/master/examples/signal_generator.rs 
    /// with some minor changes
    pub fn play_note(freq: f32) -> Result<(), Box<dyn Error>> {
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

    // Taken from https://github.com/RustAudio/rodio/blob/f1eaaa4a6346933fc8a58d5fd1ace170946b3a94/examples/mix_multiple_sources.rs
    pub fn play_notes(){
        // Construct a dynamic controller and mixer, stream_handle, and sink.
        let (controller, mixer) = dynamic_mixer::mixer::<f32>(2, 44_100);
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();

        // Create four unique sources. The frequencies used here correspond
        // notes in the key of C and in octave 4: C4, or middle C on a piano,
        // E4, G4, and A4 respectively.
        let source_c = SineWave::new(261.63)
            .take_duration(Duration::from_secs_f32(1.))
            .amplify(0.20);
        let source_e = SineWave::new(329.63)
            .take_duration(Duration::from_secs_f32(1.))
            .amplify(0.20);
        let source_g = SineWave::new(392.0)
            .take_duration(Duration::from_secs_f32(1.))
            .amplify(0.20);
        let source_a = SineWave::new(440.0)
            .take_duration(Duration::from_secs_f32(1.))
            .amplify(0.20);

        // Add sources C, E, G, and A to the mixer controller.
        controller.add(source_c);
        controller.add(source_e);
        controller.add(source_g);
        controller.add(source_a);

        // Append the dynamic mixer to the sink to play a C major 6th chord.
        sink.append(mixer);

        // Sleep the thread until sink is empty.
        sink.sleep_until_end();
    }
}