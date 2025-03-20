extern crate rust_music_theory as rustmt;
use lazy_static::lazy_static;
use regex::Regex;
use rustmt::note::{Note, NoteLetter, Pitch};
use std::collections::BTreeMap;

// This code is taken from rust-music-theaory crate from Pitch.rs.
// I wanted to use it to validate if the user enters a valid note(s)
// however as this code was not made public from the pitch module and
// I wanted to use it in a slightly different way, I duplicated it here
lazy_static! {
    static ref REGEX_PITCH: Regex = Regex::new("^[ABCDEFGabcdefg][b♭♯#s𝄪x]*$").unwrap();
}

/// Given a string (which will be user input)
/// validate if the string contains valid notes
/// ### Parameters
/// - an input str
/// ### Returns
/// - the number of valid notes
pub fn validate_input(input: &str) -> u8 {
    //TODO: trime leading and trailing whitespace and replace ',' with ''
    let input = input.replace(',', "");
    let notes = input.split_whitespace();
    let mut count = 0;
    for note in notes {
        match REGEX_PITCH.find(note) {
            Some(_m) => count += 1,
            None => return 0,
        }
    }

    if count > 4 {
        println!("Too many notes, maximum of 4 notes allowed");
        return 0;
    }
    count
}
/// Creates a BTree that will be used to find the corresponding Hz for a given pitch
/// Notes included are ony for octaves 4 and 5
/// Enharmonic notes are included
pub fn create_note_mappings() -> BTreeMap<String, f64> {
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
        ("B5", 987.77),
    ];

    for (note, freq) in &notes {
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
pub fn get_hz(key: &str, btree: &BTreeMap<String, f64>) -> f32 {
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
pub fn create_rand_pitch() -> Pitch {
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
    Pitch::from_str(letter.trim()).map(|pitch| Note::new(pitch, 4))
}

/// Creates a random note in octave 4
/// ### Returns
/// - A Note
pub fn rand_note() -> Note {
    let pitch = create_rand_pitch();
    Note::new(pitch, 4)
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
    let char_letter: char = match letter {
        NoteLetter::A => 'A',
        NoteLetter::B => 'B',
        NoteLetter::C => 'C',
        NoteLetter::D => 'D',
        NoteLetter::E => 'E',
        NoteLetter::F => 'F',
        NoteLetter::G => 'G',
    };

    let char_accidental: char = match accidental {
        1 => '#',
        2 => 'x',
        -1 => 'b',
        _ => '\0',
    };

    let mut result = String::new();
    result.push(char_letter);
    if char_accidental != '\0' {
        result.push(char_accidental);
    }
    result.push_str(&octave);
    //println!("note letter is {}", result);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_input_invalid_input() {
        assert_eq!(0, validate_input("p3"));
        assert_eq!(0, validate_input("A1"));
        assert_eq!(0, validate_input("5"));
        assert_eq!(0, validate_input("c ex s#"));
        assert_eq!(0, validate_input("h4"));
        assert_eq!(0, validate_input("m2 p4"));
        assert_eq!(0, validate_input("d7b"));
    }

    #[test]
    fn test_validate_input_valid_input() {
        assert_eq!(1, validate_input("c"));
        assert_eq!(1, validate_input("bb"));
        assert_eq!(1, validate_input("f#"));
        assert_eq!(1, validate_input("Ax"));
        assert_eq!(1, validate_input("Gbb"));
        assert_eq!(1, validate_input("Db"));
        assert_eq!(1, validate_input("E"));
    }

    #[test]
    fn test_get_hz() {
        let mappings = create_note_mappings();

        assert_eq!(261.63, get_hz("C4", &mappings));
        assert_eq!(392.00, get_hz("Fx4", &mappings));
        assert_eq!(392.00, get_hz("Fx4", &mappings));
        assert_eq!(698.46, get_hz("E#5", &mappings));
        assert_eq!(415.30, get_hz("Ab4", &mappings));

        assert_eq!(0.0, get_hz("H4", &mappings));
        assert_eq!(0.0, get_hz("Bbb4", &mappings));
        assert_eq!(0.0, get_hz("Cxx4", &mappings));
    }

    #[test]
    fn test_create_rand_pitch() {
        for _ in 0..1000 {
            let pitch = create_rand_pitch();
            let pitch_value = pitch.into_u8();
            assert!(pitch_value < 12, "Pitch out of range: {}", pitch_value);
        }
    }

    #[test]
    fn test_create_note() {
        if let Some(note) = create_note("D") {
            let note_pitch = note.pitch.into_u8();
            assert_eq!(2, note_pitch);
        }

        let bad_note = create_note("x4");
        assert!(bad_note.is_none());
    }

    #[test]
    fn test_rand_note() {
        let note = rand_note();
        let pitch_value = note.pitch.into_u8();
        assert!(pitch_value < 12, "Pitch out of range: {}", pitch_value);
        assert_eq!(note.octave, 4, "Expected octave to be 4");
    }

    #[test]
    fn test_get_note_letter() {
        let pitch = Pitch::new(NoteLetter::G, 1);
        let note = Note::new(pitch, 4);
        let note_letter = get_note_letter(note);

        assert_eq!("G#4", note_letter);
    }
}
