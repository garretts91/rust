use std::io;

fn main() {

    loop{
    let mut input: String = String::new();
    let  _first_run = true;

    println!("Press enter again, and type 'yes' to exit other wise follow prompt below.");
    println!("\n");
    println!("Enter groups of up to four 0's or 1's separated by a comma: ");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input = input.replace(" ", "");
    println!("{}", input);

    // Validate that the input contains only 0s and 1s and is no more then 4 characters after spaces are removed.
    if input
        .split(',')
        .map(|group| group.trim())
        .all(|group| group.chars().all(|c| c == '0' || c == '1' || c ==',') && group.len() <= 4){

        let mut _output = String::new();

        for (idx, j) in input.trim().split(',').enumerate() {
            if idx > 0 {
                print!(" + ");
            }
            for i in 0..j.len() {
                print!("{}", if j.as_bytes()[i] == b'0' { "/" } else { "" });
                print!("{}", (i as u8 + b'A') as char);
                }
            }
        println!("\n");
        println!("Do you want to exit? (yes/no)");
        let mut exit_input = String::new();
        io::stdin().read_line(&mut exit_input).expect("Failed to read line");

        let exit_command = exit_input.trim().to_lowercase();
        if exit_command == "yes" {
            println!("Exiting the program.");
            break; // Exit the loop
        }
        } else {
        // Print an error message if the input contains characters other than 0 and 1
        println!("Error: Input must contain only 0s and 1s and no more then 4 between each comma. You didn't do this, so Try again.");
        println!("\n")
        }
    }
}