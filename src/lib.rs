pub mod notes{
    extern crate rust_music_theory as rustmt;
    use rustmt::note::{Note,Pitch, NoteLetter};
    use std::collections::BTreeMap;

    /// Creates a BTree that will be used to find the corresponding Hz for a given pitch
    /// Notes included are ony for octaves 4 and 5
    /// Enharmonic notes are included
    pub fn create_note_mappings() -> BTreeMap<String,f64>{
        let mut note_hz: BTreeMap<String, f64> = BTreeMap::new();

        let notes = [
            ("Cb4", 246.94),
            ("C4", 261.63),
            ("C#4", 277.18),
            ("Db4", 277.18),
            ("Cx4", 293.66),
            ("D4", 293.66),
            ("D#4", 311.13),
            ("Eb4", 311.13),
            ("Dx4", 329.63),
            ("E4", 329.63),
            ("Fb4", 329.63),
            ("F4", 349.63),
            ("Ex4", 369.99),
            ("F#4", 369.99),
            ("Gb4", 369.99),
            ("Fx4", 392.00),
            ("G4", 392.00),
            ("G#4", 415.30),
            ("Ab4", 415.30),
            ("Gx4", 440.00),
            ("A4", 440.00),
            ("A#4", 466.16),
            ("Bb4", 466.16),
            ("Ax4", 493.88),
            ("B4", 493.88),
            ("Cb5", 493.88),
            ("C5", 523.25),
            ("Bx4", 554.37),
            ("C#5", 554.37),
            ("Db5", 554.37),
            ("Cx5", 587.33),
            ("D5", 587.33),
            ("D#5", 622.25),
            ("Eb5", 622.25),
            ("Dx5", 659.25),
            ("E5", 659.25),
            ("Fb5", 659.25),
            ("F5", 698.46),
            ("Ex5", 739.99),
            ("F#5", 739.99),
            ("Gb5", 739.99),
            ("Fx5", 783.99),
            ("G5", 783.99),
            ("G#5", 830.61),
            ("Ab5", 830.61),
            ("Gx5", 880.00),
            ("A5", 880.00),
            ("A#5", 932.33),
            ("Bb5", 932.33),
            ("Ax5", 987.77),
            ("B5", 987.77)
        ];

        for(note, freq) in &notes {
            note_hz.insert(note.to_string(), *freq);
        }
        note_hz
    }

    /// Given a pitch name (ex. C, Eb, A#..etc) find the corresponding Hz from the BTree
    /// If the pitch name is not found, Hz of 0 is returned
    /// ### Parameters
    ///  - key: the pitch name
    /// - btree: the created BTree of notes/pitches
    /// ### Returns
    /// - a f32 as the hz for the pitch. 
    pub fn get_hz(key: &str, btree: &BTreeMap<String,f64>) -> f32{
        let freq = btree.get(key);
        let mut note_hz: f32 = 0.0;
        match freq {
            Some(freq) => {
                println!("freq for {} is {}", key, freq);
                note_hz = *freq as f32;
            }
            None => println!("Hz for note {} not found", key),
        }
        note_hz
    }

    /// Creates a random pitch by using a random number of semitones (0-11)
    /// ### Returns
    /// - A Pitch type
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
    /// - An Option of type Note
    pub fn create_note(letter: &str) -> Option<Note> {
        println!("In create_note, letter is {}", letter);
        if let Some(pitch) = Pitch::from_str(letter.trim()){ //NOTE: use impl FromStr instead?
        //println!("result from_str is {:?}", pitch);
        //match result {
            Some(Note::new(pitch, 4)) 
        }else{
            None
        }
    }

    /// Creates a random note in octave 4
    /// ### Returns
    /// - A Note
    pub fn rand_note() -> Note {
        let pitch = create_rand_pitch();
        Note::new(pitch,4)
    }

    /// Creates the string representation (aka name) of a Note
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
    use rustmt::note::Note;
    use rustmt::interval::{Interval, IntervalError}; //, Number, Quality};

/* 
    /// Returns a random Quality type, using rust-music-theory Quality enum.
    fn rand_quality() -> Quality {
        let qualities = [Quality::Major, Quality::Minor, Quality::Perfect, Quality::Augmented, Quality::Diminished];
        let quality = qualities[rand::random_range(..qualities.len())];
        quality
    }

    /// Returns a random interval(2nd, 4th, 6th..etc), using rust-music-theory Number enum.
    fn rand_interval() -> Number {
        let intervals = [Number::Unison, Number::Second, Number::Third, Number::Fourth, Number::Fifth, Number::Sixth, Number::Seventh, Number::Octave];
        let interval = intervals[rand::random_range(..intervals.len())];
        interval
    }
*/
    /// Returns a random number (0-12) to be used as the number of semitones
    /// ### Returns
    /// - number as u8
    fn rand_semitone() -> u8 {
        rand::random_range(0..12)
    }

    /// Creates an interval given the number of semitones
    /// If it fails to create an interval from the given semitones, defaults to unison
    /// ### Returns
    /// - An Interval type
    fn create_interval_semitones(semitones: u8) -> Interval{

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
    /// - a Result type for Interval
    fn create_rand_interval() -> Result<Interval, IntervalError> {
        let semitone = rand_semitone();
        let interval_result = Interval::from_semitone(semitone);
        interval_result
    }

    ///Creates a note that is a random interval above a given note
    /// ### Parameters:
    /// - A note value (the root) as a Note type 
    /// ### Returns:
    /// - A tuple of a Note type (the 2nd note) and the random interval used
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
    /// This 'Number' is an enum of chord types from rust-music-theory Chord::Number
    /// ### Returns
    /// - a Number, aka chord type
    fn rand_number() -> Number{
        let numbers = [Number::Triad, Number::Seventh];
        let number = numbers[rand::random_range(..numbers.len())];
        number
    }

    /// Creates a random quality to be used in triad chord creation
    /// The 'Quality' is an enum of chord qualites from rust-music-theory
    /// In this function, chord quality options are limited to those for triads
    /// ### Returns
    /// - a chord Quality
    fn rand_quality_triad() -> Quality {
        let qualities = [Quality::Major, Quality::Minor, Quality::Diminished, Quality::Augmented]; 
        let quality = qualities[rand::random_range(..qualities.len())];
        quality
    }

    /// Creates a random quality to be used in seven chord creation
    /// The 'Quality' is an enum of chord qualites from rust-music-theory
    /// In this function, chord quality options are limited to those for seven chords
    /// ### Returns
    /// - a chord Quality
    fn rand_quality_seventh() -> Quality {
        let qualities = [Quality::Major, Quality::Minor, Quality::Diminished, Quality::Augmented, Quality::HalfDiminished,Quality::Dominant];
        let quality = qualities[rand::random_range(..qualities.len())];
        quality
    }

    /// Creates a chord from a string - Ex "C, E, G"
    /// Is functionaly a wrapper function for Chord::from_string
    /// ### Returns
    /// - a Chord
    pub fn create_chord(string: & str) -> Chord{
        Chord::from_string(string)
    }

    /// Creates a random chord
    /// ### Returns
    /// - a chord type
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
    /// Inversion are: 0 (root), 1 (first inversion), 2 (second inversion), 3 (third inversion - seven chords only)
    /// ### Returns
    /// - a chord type
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
    use rodio::source::{SineWave, Source};
    use rodio::{dynamic_mixer, OutputStream, Sink};
    use std::thread;
    use std::time::Duration;
    
    /// Plays the given pitches individually and together
    /// Source: https://github.com/RustAudio/rodio/blob/f1eaaa4a6346933fc8a58d5fd1ace170946b3a94/examples/mix_multiple_sources.rs
    /// Changed made to take an vec of note frequencies to play
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
                    ()
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

    
}