extern crate rust_music_theory as rustmt;
use ear_training::chords;
use ear_training::intervals;
use ear_training::notes;
use ear_training::play;
use rustmt::note::Notes;
use std::{env, io, process};

/// Utility function to get user input
/// and store answer in passed in variable
/// ### Parameters
/// - a mutable string
fn get_input(guess: &mut String) {
    io::stdin().read_line(guess).expect("Failed to read line");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut root = String::new();
    let mut interval = String::new();
    let mut guess = String::new();
    let mut chord = String::new();
    let mut choice = String::new();
    let mut res = String::new();

    if args.len() == 3 {
        let choice_type = &args[1];
        let option = &args[2];

        if choice_type.to_lowercase() == "random" && option.to_lowercase() == "interval" {
            choice = String::from("1");
        } else if choice_type.to_lowercase() == "random" && option.to_lowercase() == "chord" {
            choice = String::from("5");
        }
    }

    else if args.len() == 4 {
        let choice_type = &args[1];
        let option = &args[2];
        let input = &args[3];

        if choice_type.to_lowercase() == "given" && option.to_lowercase() == "interval"{
            if input.contains("unison")
                || input.contains("octave")
                || intervals::check_if_interval(input)
            {
                interval = String::from(input);
                choice = String::from("3");
            }
            else{
                process::exit(1);
            }
        } else if choice_type.to_lowercase() == "given" && option.to_lowercase() == "note"{
            let note_count = notes::validate_input(input);

            if note_count == 1 {
                choice = String::from("2");
                root = String::from(input);
            } else if note_count == 3 || note_count == 4 {
                choice = String::from("4");
                chord = String::from(input);
            } else {
                println!("Invalid note detected, or too many or too little notes given. Number of notes can be 1 (single note), 3 (triad) or 4 (seven chord)");
                process::exit(1);
            }
        } else {
            println!("Command arguments not valid, exiting");
            process::exit(1);
        }

        // When running with command line args, do not want to run loop
        res = String::from("n");
    } else if args.len() != 1 {
        println!("Number of args passed not valid. Expected 0, 2, or 3 arguments");
        process::exit(1);
    }

    let btree = notes::create_note_mappings();
    //println!("{:?}", btree);

    loop {
        if args.len() == 1 {
            println!("Welcome to the Ear-training tool.");
            println!("Select an option below by entering the corresponding number");
            println!("1. Random interval - Guess the interval");
            println!("2. Random Interval from a given note - Guess 2nd note");
            println!("3. Hear given interval from random note");
            println!("4. Randomly invert a given chord - Guess the inversion");
            println!("5. Random chord - Guess the chord type");

            get_input(&mut choice);
            println!("choice is {}", choice);
            res.clear();
        }

        if choice.trim() == "1" {
            //Guess a random interval
            let root = notes::rand_note();
            let (note2, interval) = intervals::create_note_from_rand_interval(root.clone());

            let root_string = notes::get_note_letter(root.clone());
            let note2_string = notes::get_note_letter(note2);

            let root_hz = notes::get_hz(&root_string, &btree);
            let note2_hz = notes::get_hz(&note2_string, &btree);

            if root_hz > 0.0 && note2_hz > 0.0 {
                let notes: Vec<f32> = vec![root_hz, note2_hz];
                play::play_notes(notes);

                println!("What interval did you hear? Hit Return/Enter key to see the answer");
                get_input(&mut guess);

                let answer = format!("{}{}", interval.quality, interval.number);
                println!("Interval was {}", answer);
            } else {
                println! {"Sorry, failed to get hz for one or more of the notes"};
            }
        } else if choice.trim() == "2" {
            //Guess 2nd note from random interval applied
            if args.len() == 1 {
                println!("Enter the note you want as the root. Ex. A, D#, Bb..etc");
                get_input(&mut root);
            }
            if notes::validate_input(&root) == 0 {
                println!("invalid note detected");
            } else {
                let root: &str = root.trim();
                if let Some(note1) = notes::create_note(root) {
                    let (note2, interval) =
                        intervals::create_note_from_rand_interval(note1.clone());

                    let note2_string = notes::get_note_letter(note2);
                    let root_string = notes::get_note_letter(note1.clone());
                    let root_hz = notes::get_hz(&root_string, &btree);
                    let note2_hz = notes::get_hz(note2_string.trim(), &btree);

                    if root_hz > 0.0 && note2_hz > 0.0 {
                        let notes: Vec<f32> = vec![root_hz, note2_hz];
                        play::play_notes(notes);

                        println!("What was the 2nd note? Hit Return/Enter key to see the answer!");
                        get_input(&mut guess);

                        let answer = format!("{}{}", interval.quality, interval.number);
                        println!("Interval was {}, 2nd note was {}", answer, note2_string);
                    } else {
                        println! {"Sorry, failed to get hz for one or more of the notes"};
                    }
                } else {
                    println!("Failed to create note");
                }
            }
        } else if choice.trim() == "3" {
            if args.len() == 1 {
                println!("Enter the interval you want to use. Ex. M3, P5, m6...etc. For a unison or octave, use 'unison', 'octave' ");
                get_input(&mut interval);
            }

            if interval.contains("unison")
                || interval.contains("octave")
                || intervals::check_if_interval(&interval)
            {
                let root = notes::rand_note();
                let root_string = notes::get_note_letter(root.clone());

                let interval_type = intervals::create_interval_string(interval.trim());
                let note2 = intervals::create_note_from_given_interval(
                    root,
                    interval_type,
                    &mut String::from("Up"),
                );

                let note2_string = notes::get_note_letter(note2);
                let root_hz = notes::get_hz(&root_string, &btree);
                let note2_hz = notes::get_hz(&note2_string, &btree);

                if root_hz > 0.0 && note2_hz > 0.0 {
                    let notes: Vec<f32> = vec![root_hz, note2_hz];
                    play::play_notes(notes);
                    println!("Notes are {} and {}.", root_string, note2_string);
                } else {
                    println! {"Sorry, failed to get hz for one or more of the notes"};
                }
            }
        } else if choice.trim() == "4" {
            //Guess the inversion
            if args.len() == 1 {
                println!("Enter the notes you want to use in the chord. Ex. 'C E G'");
                get_input(&mut chord);
            }
            if notes::validate_input(&chord) == 0 {
                // check that user enteres max of 4 notes
                println!("invalid note detected");
            } else {
                let chord_type = chords::create_chord(&chord);
                let chord_inverted = chords::rand_inversion(chord_type);
                //println!("chord after inversion is {:?}", chord_inverted);
                let notes_in_chord = chord_inverted.notes();

                let mut notes: Vec<f32> = Vec::new();

                for note in notes_in_chord.into_iter() {
                    let letter = notes::get_note_letter(note);
                    //println!("note letter is {}", letter);
                    let note_hz = notes::get_hz(&letter, &btree);

                    if note_hz > 0.0 {
                        notes.push(note_hz);
                    }
                }
                play::play_notes(notes);

                println!(
                    "Which chord inversion did you hear? Hit Return/Enter key to see the answer"
                );

                get_input(&mut guess);

                match chord_inverted.inversion {
                    0 => println!("Root position"),
                    1 => println!("Frist inversion"),
                    2 => println!("Second inversion"),
                    3 => println!("Third inversion"),
                    _ => println!("inversion is {}", chord_inverted.inversion),
                }
            }
        } else if choice.trim() == "5" {
            //Guess the chord type
            let chord = chords::rand_chord();
            let notes_in_chord: Vec<rustmt::note::Note> = chord.notes();
            let mut notes: Vec<f32> = Vec::new();

            for note in notes_in_chord.into_iter() {
                let letter = notes::get_note_letter(note);
                //println!("note letter is {}", letter);
                let note_hz = notes::get_hz(&letter, &btree);

                if note_hz > 0.0 {
                    notes.push(note_hz);
                }
            }
            play::play_notes(notes);

            println!("What chord type did you hear? Hit Return/Enter key to see the answer");

            get_input(&mut guess);
            let answer = format!("{} {}", chord.quality, chord.number);
            println!("Chord was {}", answer);
        } else {
            println!("Choice not valid")
        }

        if args.len() == 1 {
            println!("Choose again? y/n");

            get_input(&mut res);
            println!("res is {}", res);
        }
        if res.trim() == "n" {
            break;
        }

        root.clear();
        interval.clear();
        guess.clear();
        chord.clear();
        choice.clear();
    }
}
