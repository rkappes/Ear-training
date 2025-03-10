extern crate rust_music_theory as rustmt;
use ear_training::intervals; 
use ear_training::notes;
use ear_training::play;
use ear_training::chord;
use rustmt::note::Notes;
use std::io;
// extern crate rodio;
// use std::error::Error;

fn main() {
    // let notes: Vec<f32> = vec![261.63, 329.63, 392.00];
    // play::play_notes(notes);
 
    let btree = notes::create_note_mappings();
    println!("{:?}", btree);

    // let freq = btree.get("A#");
    // match freq {
    //     Some(freq) => println!("freq for A# is {}", freq),
    //     None => println!("Key not found"),
    // }

    loop{
        let mut root = String::new();
        //let mut interval = String::new();
        let mut chord = String::new();
        let mut choice = String::new();
        let mut res = String::new();
        

        println!("Welcome to the Ear-training tool.");
        println!("Select an option below by entering the corresponding number");
        println!("1. Random interval - Guess the interval");
        println!("2. Random Interval from given note - Guess 2nd note");
        println!("3. Given interval from random note - ");
        println!("4. Randomly invert a given chord - Guess the inversion");
        println!("5. Random chord - guess the chord type");

        io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

        println!("choice is {}", choice);
        if choice.trim() == "1"{
            //Guess a random interval
            let root = notes::rand_note();
            let (note2, interval) = intervals::create_note_from_rand_interval(root.clone());

            let root_string = notes::get_note_letter(root.clone());
            let note2_string = notes::get_note_letter(note2);

            let root_hz = notes::get_hz(&root_string, &btree);
            //play::play_note(root_hz);

            let note2_hz = notes::get_hz(&note2_string, &btree);
            //play::play_note(note2_hz);

            if root_hz > 0.0 && note2_hz > 0.0 {
                let notes: Vec<f32> = vec![root_hz, note2_hz];
                play::play_notes(notes);
            }
            else{
                println!{"Sorry, failed to get hz for one or more of the notes"};
            }
            let answer =  format!("{}{}",interval.quality, interval.number);
            println!("Interval was {}", answer);
        }
        else if choice.trim() == "2" {
            //Guess 2nd note from random interval applied
            println!("Enter the note you want as the root. Ex. A, D#, Bb..etc");
            io::stdin()
            .read_line(&mut root)
            .expect("Failed to read line");

            let root: &str = &root.trim();
            if let Some(note1) = notes::create_note(root){
                let (note2, interval) = intervals::create_note_from_rand_interval(note1.clone());
                let note2_string = notes::get_note_letter(note2);
    
                let root_string = notes::get_note_letter(note1.clone());
                let root_hz = notes::get_hz(&root_string, &btree);
                // play::play_note(root_hz);
    
                let note2_hz = notes::get_hz(&note2_string.trim(), &btree);
                // play::play_note(note2_hz);
    
                if root_hz > 0.0 && note2_hz > 0.0{
                    let notes: Vec<f32> = vec![root_hz, note2_hz];
                    play::play_notes(notes);
                }
                else{
                    println!{"Sorry, failed to get hz for one or more of the notes"};
                }
                let answer =  format!("{}{}",interval.quality, interval.number);
                println!("Interval was {}, 2nd note was {}", answer, note2_string);
                
            } else{
                println!("Failed to create note");
            }


        }
        // else if choice.trim() == "3"{
        //     println!("Enter the interval you want to use. Ex. M3, P5, m6..etc");
        //     io::stdin()
        //     .read_line(&mut interval)
        //     .expect("Failed to read line");

        //     if let Some(root) = notes::rand_note(){
        //         let root_string = notes::get_note_letter(root.clone());

        //         let interval_type = intervals::create_interval_string(&interval.trim());
        //         let note2 = intervals::create_note_from_given_interval(root, interval_type, &mut String::from("Up"));
    
        //         let note2_string = notes::get_note_letter(note2);
        //         let root_hz = notes::get_hz(&root_string, &btree);
        //         // play::play_note(root_hz);
    
        //         let note2_hz = notes::get_hz(&note2_string, &btree);
        //         // play::play_note(note2_hz);
        //     }else{
        //         println!("Failed to generate random note");
        //     }   
            
        // }
        else if choice.trim() == "4"{
            //Guess the inversion
            println!("Enter the notes you want to use in the chord. Ex. 'C, E, G'");
            io::stdin()
            .read_line(&mut chord)
            .expect("Failed to read line");

            //TODO: create_chord will panic if it can't create chord
            // check that user enteres max of 4 notes
            let chord_type = chord::create_chord(&chord);
            let chord_inverted = chord::rand_inversion(chord_type);
            println!("chord after inversion is {:?}", chord_inverted);
            let notes_in_chord = chord_inverted.notes();

            let mut notes: Vec<f32> = Vec::new();

            for note in notes_in_chord.into_iter() {
                let letter = notes::get_note_letter(note);
                println!("note letter is {}", letter);

                let note_hz = notes::get_hz(&letter, &btree);

                if note_hz > 0.0 {
                    notes.push(note_hz);
                    // play::play_note(note_hz);
                }
            }
            play::play_notes(notes);

            println!("inversion is {}", chord_inverted.inversion);
        }
        else if choice.trim() == "5"{
            //Guess the chord type
            let chord = chord::rand_chord();
            let notes_in_chord: Vec<rustmt::note::Note> = chord.notes();
            let mut notes: Vec<f32> = Vec::new();

            for note in notes_in_chord.into_iter() {
                let letter = notes::get_note_letter(note);
                println!("note letter is {}", letter);
                let note_hz = notes::get_hz(&letter, &btree);
                //TODO: check that hzs are > 0

                if note_hz > 0.0 {
                    notes.push(note_hz);
                // play::play_note(note_hz);
                }
            }
            play::play_notes(notes);

            let answer =  format!("{}{}",chord.quality, chord.number);
            println!("Chord was {}", answer);

        }
        else{
            println!("Choice not valid")
        }

        println!("Again? y/n");
        io::stdin()
        .read_line(&mut res)
        .expect("Failed to read line");

        println!("res is {}", res);
        if res.trim() == "n"{
            break;
        }
    }
}
