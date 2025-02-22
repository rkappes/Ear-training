extern crate rust_music_theory as rustmt;
use rustmt::note::{Note,Pitch, NoteLetter};
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

pub fn create_interval(q: Quality, semitone: u8, int: Number) -> Interval{
    let interval = Interval::new(semitone,q, int, None);
    interval
}

pub fn create_rand_interval() -> Result<Interval, IntervalError> {
    let semitone = rand_semitone();
    let interval_result = Interval::from_semitone(semitone);
    interval_result
}