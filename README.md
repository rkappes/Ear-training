# Ear-training

## Description
The purpose of this program is to be used as an ear training tool. It is assumed that anyone who uses this is familiar with intervals and chords in music. 

This program will use/accept these intervals: 
1. Unison
2. Minor 2nd (m2)
3. Major 2nd (M2)
4. Minor 3rd (m3)
5. Major 3rd (M3)
6. Diminished 4th (D4)
7. Perfect 4th (P4)
8. Augmented 4th (A4)
9. Diminished 5th (D5)
10. Perfect 5th (P5)
11. Augmented 5th (A5)
12. Minor 6th (m6)
13. Major 6th (M6)
14. Minor 7th (m7)
15. Major 7th (M7)
16. Octave
    
And these different chord types (for triads and seven chords):
1. Major
2. Minor
3. Diminished
4. Augmented
5. Major seven
6. Minor Seven
7. Diminished Seven
8. Half-diminished seven
9. Dominant Seven

And lastly the possible inversions:
1. Root
2. First
3. Second
4. Third (seven chords only)

## Usage
Users have the option to:
1. Hear a random interval - Guess the interval
2. Hear a random Interval from a given note - Guess 2nd note
3. Hear a given interval from random note
4. Randomly invert a given chord - Guess the inversion
5. Hear a random chord - Guess the chord type

The program will play and output the notes used. 

## Compile and Run
After cloning the repo, use ` cargo build ` and ` cargo run `

### There are 2 different ways to run this program

1. Use `cargo run` and follow in-program prompts
2. Use with command-line arguments
   - `cargo run -- random interval` will play a random interval
   - `cargo run -- random chord` will play a random chord
   - `cargo run -- given YOUR_INPUT` where YOUR_INPUT could be:
     - a single note to hear a random interval from this given note `cargo run -- given c#` 
     - an interval to hear this interval played `cargo run -- given P5` or `cargo run -- given octave`
     - notes constitutin a chord, to hear a random inversion of this chord `cargo run -- given "c e g"` or `cargo run -- given "b d# f a"`

## Tests
Tests are included. To run these `cargo test`

## License
MIT + Apache 2.0 
