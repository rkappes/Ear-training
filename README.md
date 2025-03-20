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
   - `cargo run -- given interval YOUR_INPUT` to hear an interval, where YOUR_INPUT is:
     - an interval `cargo run -- given interval P5` or `cargo run -- given interval octave`
         - To hear a unison or octave, please use 'unison' or 'octave'.
         - Augmented and Diminished should only be used with perfect intervals. Ex. A4, d5
         - Other intervals can use typical notation (M3, m6, P5)   
   - `cargo run -- given note YOUR_INPUT` to hear a random interval from this given note. Where YOUR_INPUT could be:
     -  a single note  `cargo run -- given note c#` 
     - notes constituting a chord, to hear a random inversion of this chord `cargo run -- given note "c e g"` or `cargo run -- given note "b d# f a"`
         - Please note that the chord notes should be contained within a string " "

## Tests
Tests are included. To run these `cargo test`

## Comments
Overall the process of using the rust-music-theory crate to create notes, intervals, and chords went smoother than anticipated. At the start of development, I did have an issue where I wasn't able to access some of the  modules (interval Number and Quality) needed to create intervals. This was solved by pulling the crate directly from the github repo versus crate.io. I beleive that the latest changes to the git repo (made 7 months ago) have not made it to the crate.io version. 
There were also some functions used from the rust-music-theory crate that would panic if invalid input was used. So finding a way to manage this so _my_ program would not panic was something.

Even using rodio crate to play-back the pitches was less challengeing to figure out than anticipated. I did have to map each note to their corresponding frequency. I was not sure how many octaves and the best way to store this information would be. I used a BTree as I wanted to be able to search the note name and retreive the frequency, but this tree does need to be constructed each time the program runs. 

The most challenging parts were accounting for enharmonic note spellings and equivalent intervals (Ex. augmented fourth is the same as a diminished 5th) and checking user input (if a valid note or interval was entered).  

## License
MIT + Apache 2.0 
