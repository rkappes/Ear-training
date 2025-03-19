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
    static ref REGEX_INTERVAL: Regex = Regex::new("^[MmPpAaDd][234567]").unwrap();
}

pub fn check_if_interval(input: &str)->bool{
    //TODO: trime leading and trailing whitespace and replace ',' with ''
    let input = input.replace(',', "");
    let notes = input.trim().split_whitespace();
    let mut count = 0;
    for note in notes {
        match REGEX_INTERVAL.find(&note){
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
    let interval = match interval_result {
        Ok(interval) => interval,
        Err(_error) => Interval::default(),
    };
    //println!("Interval is {:?}", interval);

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
    //println!("In create_Interval_string, interval is {}", interval);
    let semitones: u8;
    match interval.trim() {
        "unison" => semitones = 0,
        "m2" => semitones = 1,
        "M2" => semitones =2,
        "m3" => semitones =3,
        "M3" => semitones =4,
        "P4" | "p4" => semitones =5,
        "A4" | "a4" | "d5" | "D5" => semitones = 6,
        "P5" | "p5" => semitones = 7,
        "m6" | "A5" | "a5" => semitones =8,
        "M6" => semitones =9,
        "m7" => semitones =10,
        "M7" => semitones =11,
        "octave" => semitones =12,
        _=> {
            println!("Unable to create interval from {}, defaulting to unison", interval);
            semitones = 0 //TODO: Return an error or do something else?
        }
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
    let note =  if direction == "down" {
        interval.second_note_down_from(root)
    }
    else{
        interval.second_note_from(root)
    };
    note
}
//}