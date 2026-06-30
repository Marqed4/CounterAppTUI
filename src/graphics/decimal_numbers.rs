use std::vec;
/*  GOAL: Create a module 
    that will return a valid ascii combination 
    that represents a given number. */

// Main only allows the user to count up to u8 sized integer.
pub fn get_ascii_number(idx: u8) -> Vec<&'static str> {
    let zero: Vec<&str> = vec![
        " 0000 ",
        "00  00",
        "00  00",
        "00  00",
        " 0000 ",
    ];

    let one: Vec<&str> = vec![
        "  11  ",
        " 111  ",
        "  11  ",
        "  11  ",
        " 1111 ",
    ];

    let two: Vec<&str> = vec![
        " 2222 ",
        "22  22",
        "   22 ",
        " 22   ",
        "222222",
    ];

    let three: Vec<&str> = vec![
        " 3333 ",
        "33  33",
        "  333 ",
        "33  33",
        " 3333 ",
    ];

    let four: Vec<&str> = vec![
        "44  44",
        "44  44",
        "444444",
        "    44",
        "    44",
    ];

    let five: Vec<&str> = vec![
        "555555",
        "55    ",
        "55555 ",
        "    55",
        "55555 ",
    ];

    let six: Vec<&str> = vec![
        " 6666 ",
        "66    ",
        "66666 ",
        "66  66",
        " 6666 ",
    ];

    let seven: Vec<&str> = vec![
        "777777",
        "   77 ",
        "  77  ",
        " 77   ",
        " 77   ",
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
        " 99999",
        "    99",
        " 9999 ",
    ];

    let ascii_numbers = vec![zero, one, two, three, four, five, six, seven, eight, nine];

    return ascii_numbers[idx as usize].to_vec();
}

// ASCII concat on a level basis.
// Thoughts: Build each digit as vector line and zip them together.
pub fn new_u8_to_ascii_string(mut value: u8) -> String {

    let mut current_ascii_numbers = vec![];

    fn create_ascii(current_ascii_numbers: Vec<Vec<&str>>) -> String {
        
        let mut ascii_string = String::new();

        for idx in 0..5 {
            for i in 0..current_ascii_numbers.len() {
                ascii_string.push_str(current_ascii_numbers[i][idx]);
                // Tab size eq four spaces.
                ascii_string.push_str("    ");
                /* Depending on the distance of the last char 
                in a string calculate spaces until next sub vector. */
                // Don't apply spacing to the character because it offsets the image.
                if i != current_ascii_numbers.len() - 1 {
                    for _ in 1..(6 - current_ascii_numbers[i][idx].len()) {
                        // Push strings to continue gap formating
                        ascii_string.push_str(" ");
                    }
                }
            }
            ascii_string.push('\n');
        }

        return ascii_string
    }

    // Unsigned integer limitations:

    // u8 for values equal to 0
    if value == 0 {
        current_ascii_numbers.insert(0, get_ascii_number(0));
    }

    // u8 for values greater than 0
    while value != 0 {
        current_ascii_numbers.insert(0, get_ascii_number(value % 10));
        value = value / 10;
    }

    return create_ascii(current_ascii_numbers);
}