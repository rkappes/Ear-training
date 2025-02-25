pub mod intervals {
    extern crate rust_music_theory as rustmt;
    use rustmt::note::{Note,Pitch, NoteLetter};
    use rustmt::interval::{Interval, IntervalError, Number, Quality};

    pub fn create_note(letter: &str) -> Note {
        let result = Pitch::from_str(letter);
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
    pub fn create_interval(semitones: u8) -> Interval{

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