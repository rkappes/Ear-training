extern crate rust_music_theory as rustmt;
use rustmt::note::Note;
use rustmt::interval::{Interval, IntervalError}; //, Number, Quality};
use lazy_static::lazy_static;
use regex::Regex;
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

lazy_static! {
    static ref REGEX_INTERVAL: Regex = Regex::new("^[Mm][2367]$|^[PpAaDd][45]$").unwrap();
}

pub fn check_if_interval(input: &str)->bool{
    let input = input.replace(',', "");
    let notes = input.split_whitespace();
    let mut count = 0;
    for note in notes {
        match REGEX_INTERVAL.find(note){
            Some(_m) => count += 1,
            None => {
                println!("Interval {}, not valid", note);
                return false
            }
        }
    }

    if count > 1{
        println!("Please only enter a single interval");
        return false;
    }
    true
}
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
    interval_result.unwrap_or_default()
    //println!("Interval is {:?}", interval);
}

/// Creates an interval given a string value: m3, M6, P5..etc
/// Matches the interval to the number of semitones and calls 
/// create_interval_semitones
/// ### Parameters:
/// - a &str of the desired interval
/// ### Returns
/// - Interval type 
pub fn create_interval_string(interval: &str) -> Interval{
    //println!("In create_Interval_string, interval is {}", interval);
    let semitones: u8 = match interval.trim() {
        "unison" => 0,
        "m2" => 1,
        "M2" => 2,
        "m3" => 3,
        "M3" | "d4" | "D4" => 4,
        "P4" | "p4" => 5,
        "A4" | "a4" | "d5" | "D5" => 6,
        "P5" | "p5" => 7,
        "m6" | "A5" | "a5" => 8,
        "M6" => 9,
        "m7" => 10,
        "M7" => 11,
        "octave" => 12,
        _=> {
            println!("Unable to create interval from {}, defaulting to unison", interval);
            0 //TODO: Return an error or do something else?
        }
    };

    create_interval_semitones(semitones)
}

/// Creates a random interval by randomly choosing a semitone value (0-12)
/// and calling rust-music-theory crate's Interval::from_semitone
/// ### Returns
/// - a Result type for Interval
fn create_rand_interval() -> Result<Interval, IntervalError> {
    let semitone = rand_semitone();
    Interval::from_semitone(semitone)
}

///Creates a note that is a random interval above a given note
/// ### Parameters:
/// - A note value (the root) as a Note type 
/// ### Returns:
/// - A tuple of a Note type (the 2nd note) and the random interval used
pub fn create_note_from_rand_interval(root: Note) -> (Note, Interval) {
    let interval_result = create_rand_interval();
    let interval = interval_result.unwrap_or_default();

    //println!("Random Interval is {:?}", interval);
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
    if direction == "down" {
        interval.second_note_down_from(root)
    }
    else{
        interval.second_note_from(root)
    }
}


#[cfg(test)]
mod tests {
    use rust_music_theory::note::{Pitch,NoteLetter};

    use super::*;

    #[test]
    fn test_check_if_interval_invalid_input(){
        assert_eq!(false, check_if_interval("p3"));
        assert_eq!(false, check_if_interval("M4"));
        assert_eq!(false, check_if_interval("5"));
        assert_eq!(false, check_if_interval("p8"));
        assert_eq!(false, check_if_interval("h4"));
        assert_eq!(false, check_if_interval("m2 p4"));
        assert_eq!(false, check_if_interval("d7"));
    }

    #[test]
    fn test_check_if_interval_valid_input(){
        assert_eq!(true, check_if_interval("m2"));
        assert_eq!(true, check_if_interval("P4"));
        assert_eq!(true, check_if_interval("D5"));
        assert_eq!(true, check_if_interval("a4"));
        assert_eq!(true, check_if_interval("M7"));
        assert_eq!(true, check_if_interval("d5"));
        assert_eq!(true, check_if_interval("A4"));
    }

    #[test]
    fn test_rand_semitone(){
        for _ in 0..1000{
            let semitone = rand_semitone();
            assert!(semitone < 12, "Semitone {} out of range", semitone);
        }
    }

    #[test]
    fn test_create_interval_semitones(){
        let interval = create_interval_semitones(7);
        assert_eq!(7, interval.semitone_count);

        let bad_interval = create_interval_semitones(13);
        assert_eq!(0, bad_interval.semitone_count);
    }

    #[test]
    fn test_create_interval_string(){
        let interval = create_interval_string("P5");
        assert_eq!(7, interval.semitone_count);

        let interval = create_interval_string("d4");
        assert_eq!(4, interval.semitone_count);

        let interval = create_interval_string("p8");
        assert_eq!(0, interval.semitone_count);
    }

    #[test]
    fn test_create_rand_interval(){
        let interval_result = create_rand_interval();
        assert!(interval_result.is_ok());        
    }

    fn mock_create_rand_interval() -> Result<Interval, IntervalError> {
        Interval::from_semitone(4)
    }

    fn mock_create_note_from_rand_interval(note: Note)-> (Note, Interval) {
        let interval_result = mock_create_rand_interval();
        let interval = match interval_result {
            Ok(interval) => interval,
            Err(_error) => Interval::default(),
        };
    
        //println!("Random Interval is {:?}", interval);
        let new_note = interval.second_note_from(note);
        (new_note, interval)
    }

    #[test]
    fn test_create_note_from_rand_interval(){
        let pitch = Pitch::new(NoteLetter::B, 0);
        let note = Note::new(pitch, 4);

        let(note2, interval) = mock_create_note_from_rand_interval(note);

        let note2_pitch = note2.pitch.into_u8();
        assert_eq!(3, note2_pitch);
        assert_eq!(4, interval.semitone_count);
    }

    #[test]
    fn test_create_note_from_given_interval(){
        let pitch = Pitch::new(NoteLetter::F, 0);
        let root = Note::new(pitch, 4);
        let interval = Interval::from_semitone(10).unwrap();
        let note = create_note_from_given_interval(root, interval, &mut String::from("up"));

        let note2_pitch = note.pitch.into_u8();
        assert_eq!(3, note2_pitch);
        assert_eq!(5, note.octave);
    }
}
