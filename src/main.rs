use std::io::{BufRead, BufReader};
use std::{error::Error, fs::File};

fn main() {
    let read_result = read_file_by_lines("calibration.txt");
    match read_result {
        Ok(_) => println!("finished reading file!"),
        Err(e) => println!("error reading file: {:?}", e),
    }
}

fn read_file_by_lines(filepath: &str) -> Result<(), Box<dyn Error>> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);

    // for accmululating and adding up all the calibration values
    let mut total = 0;

    // loop through each line in the calibration document
    for each_line in reader.lines() {
        match each_line {
            Ok(line) => {
                let mut first_digit = 'x';
                let mut last_digit = '0';
                for each_c in line.chars() {
                    // first we check if this character is numeric
                    if each_c.is_numeric() {
                        // if the 1st digit haven't been cached yet
                        if first_digit == 'x' {
                            first_digit = each_c;
                        }

                        // we also assign this character to the last digit so when the line ends, this will be the last digit for sure
                        last_digit = each_c;
                    }
                }

                // we then combine the first and last numeric characters together
                // and get the i32 from it to add to our total
                let this_line_total = combine_characters(first_digit, last_digit);
                total += this_line_total;
            }
            Err(e) => println!("error reading next line: {:?}", e),
        }
    }

    println!("final total is :{}", total);

    Ok(())
}

fn combine_characters(first: char, second: char) -> i32 {
    let concat = format!("{}{}", first, second);
    concat.parse::<i32>().unwrap()
}
