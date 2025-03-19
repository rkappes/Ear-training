extern crate rust_music_theory as rustmt;
use rustmt::note::{Note,Pitch, NoteLetter};
use std::collections::BTreeMap;
use lazy_static::lazy_static;
use regex::Regex;

// This code is taken from rust-music-theaory crate from Pitch.rs. 
// I wanted to use it to validate if the user enters a valid note(s)
// however as this code was not made public from the pitch module and 
// I wanted to use it in a slightly different way, I duplicated it here
lazy_static! {
    static ref REGEX_PITCH: Regex = Regex::new("^[ABCDEFGabcdefg][b♭♯#s𝄪x]*").unwrap();
}

/// Given a string (which will be user input)
/// validate if the string contains valid notes
/// ### Parameters
/// - an input str
/// ### Returns
/// - the number of valid notes
pub fn validate_input(input: &str)->u8{
    //TODO: trime leading and trailing whitespace and replace ',' with ''
    let input = input.replace(',', "");
    let notes = input.trim().split_whitespace();
    let mut count = 0;
    for note in notes {
        match REGEX_PITCH.find(&note){
            Some(_m) => count += 1,
            None => return 0
        }
    }

    if count > 4{
        println!("Too many notes, maximum of 4 notes allowed");
        return 0
    }
    count
}
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
        ("E#4", 349.63),
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
        ("B#4", 523.35),
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
        ("E#5", 698.46),
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
            //println!("freq for {} is {}", key, freq);
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
    //println!("In create_note, letter is {}", letter);
    //TODO: check what happends if pitch created is double sharp or double flat
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
    //println!("note letter is {}", result);
    result
}
