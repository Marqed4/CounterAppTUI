use std::vec;
#[cfg(test)]
use crate::tests;
/*  GOAL: Create a module 
    that will return a valid ascii combination 
    that represents a given number. */

// Main only allows the user to count up to u8 sized integer.
pub fn get_ascii_number(idx: u8) -> Vec<&'static str> {
    let zero: Vec<&str> = vec![
        "000000",
        "00  00",
        "00  00",
        "00  00",
        "000000",
    ];

    let one: Vec<&str> = vec![
        "1111",
        "1 11",
        "  11",
        "  11",
        "111111",
    ];

    let two: Vec<&str> = vec![
        "222222",
        "     2",
        " 2222 ",
        "2     ",
        "222222",
    ];

    let three: Vec<&str> = vec![
        "33333 ",
        "    33",
        "333333",
        "    33",
        "33333 ",
    ];

    let four: Vec<&str> = vec![
        "44  44",
        "44  44",
        " 44444",
        "    44",
        "    44",
    ];

    let five:Vec<&str> = vec![
        "555555",
        "55    ",
        "555555",
        "    55",
        "555555",
    ];

    let six: Vec<&str> = vec![
        "666666",
        "66    ",
        "66666 ",
        "66  66",
        " 6666 ",
    ];

    let seven: Vec<&str> = vec![
        "777777",
        "    77",
        "    77",
        "    77",
        "    77",
    ];

    let eight: Vec<&str> = vec![
        " 8888 ",
        "88  88",
        " 8888 ",
        "88  88",
        " 8888 ",
    ];

    let nine: Vec<&str> = vec![
        " 9999 ",
        "99  99",
        " 9999 ",
        "    99",
        "99999 ",
    ];

    let ascii_numbers = vec![zero, one, two, three, four, five, six, seven, eight, nine];;

    return ascii_numbers[idx as usize].to_vec();
}

// ASCII concat on a level basis.
// Thoughts: Build each digit as vector line and zip them together.
pub fn new_u8_to_ascii_string(mut value: u8) -> String {
    let mut ascii_string = String::new();
    let mut current_ascii_numbers = vec![];

    while value != 0 {
        current_ascii_numbers.insert(0, get_ascii_number(value % 10));
        value = value / 10;
    }

    for idx in 0..5 {
        for i in 0..current_ascii_numbers.len() {
            ascii_string.push_str(current_ascii_numbers[i][idx]);
            // Tab size eq four spaces.
            ascii_string.push_str("    ");
            /* Depending on the distance of the last char 
            in a string calculate spaces until next sub vector. */
            for _ in 1..(6 - current_ascii_numbers[i][idx].len()) {
                // Push strings to continue gap formating
                ascii_string.push_str(" ");
            }
        }
        ascii_string.push('\n');
    }

    return ascii_string
}