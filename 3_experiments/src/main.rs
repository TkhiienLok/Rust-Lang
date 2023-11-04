
mod temperature {
    pub fn fahrenheit_to_celsius(f: i8) -> f32 {
        (f - 32) as f32 * 5.0 / 9.0
    }
    
}
fn main() {
    // 1.
    // use std::io;
    // loop {
    //     let mut fahrengheit_temp: String = String::new();

    //     println!("Enter fahrengheits: ");

    //     io::stdin()
    //         .read_line(&mut fahrengheit_temp)
    //         .expect("Failed to read line");

    //     match fahrengheit_temp.trim().parse() {
    //         Ok(num) => {
    //             println!("{} F = {:.0} C ", num, temperature::fahrenheit_to_celsius(num));
    //             break;
    //         }
    //         Err(_) => continue,
    //     };
    // }

    // 2.
    sing_song()
}

const NUMBERS_ORDINAL: [&str; 12] = [
    "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth",
    "eleventh", "twelveth",
];

const SONG_LINES: [&str; 12] = [
    "A partridge in a pear tree",
    "Two turtle doves and",
    "Three french hens",
    "Four calling birds",
    "Five golden rings",
    "Six geese a-laying",
    "Seven swans a-swimming",
    "Eight maids a-milking",
    "Nine ladies dancing",
    "Ten lords a-leaping",
    "Eleven pipers piping",
    "Twelve drummers drumming",
];

fn sing_song() {
    println!("On the {0} day of Christmas, my true love sent to me", NUMBERS_ORDINAL[0]);
    println!("{0}", SONG_LINES[0]);
    println!("");
    for i in 1..SONG_LINES.len() {
        println!("On the {0} day of Christmas, my true love sent to me", NUMBERS_ORDINAL[i]);
    
        for j in (0..=i).rev() {
            println!("{0}", SONG_LINES[j]);
        }
        println!("");
    }
}
