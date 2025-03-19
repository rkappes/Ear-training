extern crate rust_music_theory as rustmt;
use rustmt::chord::{Chord, Number, Quality};
use rustmt::note::Pitch;
use crate::notes::create_rand_pitch;

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
    
    let pitch = create_rand_pitch();

    //println!("In rand_chord, pitch is {}, number is {} and quality is {}", pitch, number, quality);

    Chord::new(pitch, quality, number)
} 

/// Applies a random inversion to a given chord
/// Inversion are: 0 (root), 1 (first inversion), 2 (second inversion), 
/// 3 (third inversion - seven chords only)
/// ### Returns
/// - a chord type
pub fn rand_inversion(chord: Chord) -> Chord {
    let root = chord.root;
    let quality = chord.quality;
    let number = chord.number;
    let inversion = rand::random_range(1..4);
    //println!("In rand_inversion, inversion is {}", inversion);
    Chord::with_inversion(root, quality, number,inversion)
}

/// Code duplicated from the Chord::from_interval function in rust-music-theory crate
/// https://github.com/ozankasikci/rust-music-theory/blob/master/src/chord/chord.rs
/// However that function panics if chord is not valid
/// This function uses the Default chord instead to avoid panicing
/// ### Parameters
/// - root pitch
/// - interval: the intervals in semitones
/// ### Returns
/// - chord
fn create_from_intervals(root: Pitch, interval: &[u8]) -> Chord {
    use Number::*;
    use Quality::*;
    let (quality, number) = match interval {
        &[4, 3] => (Major, Triad),
        &[3, 4] => (Minor, Triad),
        &[2, 5] => (Suspended2, Triad),
        &[5, 2] => (Suspended4, Triad),
        &[4, 4] => (Augmented, Triad),
        &[3, 3] => (Diminished, Triad),
        &[4, 3, 4] => (Major, Seventh),
        &[3, 4, 3] => (Minor, Seventh),
        &[4, 4, 2] => (Augmented, Seventh),
        &[4, 4, 3] => (Augmented, MajorSeventh),
        &[3, 3, 3] => (Diminished, Seventh),
        &[3, 3, 4] => (HalfDiminished, Seventh),
        &[3, 4, 4] => (Minor, MajorSeventh),
        &[4, 3, 3] => (Dominant, Seventh),
        &[4, 3, 3, 4] => (Dominant, Ninth),
        &[4, 3, 4, 3] => (Major, Ninth),
        &[4, 3, 3, 4, 4] => (Dominant, Eleventh),
        &[4, 3, 4, 3, 3] => (Major, Eleventh),
        &[3, 4, 3, 4, 3] => (Minor, Eleventh),
        &[4, 3, 3, 4, 3, 4] => (Dominant, Thirteenth),
        &[4, 3, 4, 3, 3, 4] => (Major, Thirteenth),
        &[3, 4, 3, 4, 3, 4] => (Minor, Thirteenth),
        _ => {println!("Couldn't create chord! Using CMaj Triad instead");
            return Chord::default();
        },
    };
    Chord::new(root, quality, number)
}

/// Creates a chord from a string - Ex "C, E, G"
/// Code is duplicated from Chord::from_string in rust-music-theory crate
/// https://github.com/ozankasikci/rust-music-theory/blob/master/src/chord/chord.rs
/// However that function calls the Chord::from_interval function which panics if chord is not valid
/// This calls the create_from_intervals function to create a chord without panicing
/// ### Parameters
/// - string representation of the chord notes. Ex "C E G"
/// ### Returns
/// - a Chord
pub fn create_chord(string: & str) -> Chord{
    // Chord::from_string(string)
    let notes: Vec<Pitch> = string.to_string()
        .replace(",", "")
        .split_whitespace()
        .into_iter()
        .map(|x| Pitch::from_str(x).expect(&format!("Invalid note {:?}.", x)))
        .collect();

    let intervals: Vec<u8> = notes.iter()
        .map(|&x| Pitch::into_u8(x) % 12)
        .zip(notes[1..].iter().map(|&x| Pitch::into_u8(x)))
        .map(|(x, y)| if x < y {y - x} else {y + 12 - x})
        .collect();

    create_from_intervals(notes[0], &intervals)
}

#[cfg(test)]
mod tests {

    use super::*;
    use rust_music_theory::note::NoteLetter;

    fn mock_rand_number() -> Number{
        Number::Triad
    }

    fn mock_rand_quality_triad() -> Quality{
        Quality::Major
    }

    fn mock_rand_quality_seventh() -> Quality{
        Quality::Minor
    }

    fn mock_create_rand_pitch() -> Pitch{
        Pitch::new(NoteLetter::A, 0)
    }
    
    fn mock_rand_chord() ->Chord{
        let number = mock_rand_number();
        let quality = if number == Number::Triad{
            mock_rand_quality_triad()
        }
        else{
            mock_rand_quality_seventh()
        };
        
        let pitch = mock_create_rand_pitch();
    
        //println!("In rand_chord, pitch is {}, number is {} and quality is {}", pitch, number, quality);
    
        Chord::new(pitch, quality, number)
    }

    #[test]
    fn test_rand_chord(){
        let chord = mock_rand_chord();
        let number = chord.number;
        let quality = chord.quality;
        assert_eq!(number, Number::Triad);
        assert_eq!(quality, Quality::Major);
    }

}