use std::io;

fn main() {
    loop {
        println!("Please enter up to 4 bits per expression in a truth table (e.g., 0010), each separated by commas (or type 'exit' to quit):");

        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("Failed to read line");

        let user_input = user_input.trim();

        if user_input.to_lowercase() == "exit" {
            break;
        }

        let rows: Vec<&str> = user_input.split(',').map(|s| s.trim()).collect();

        let mut final_expression = String::new();

        for (index, row) in rows.iter().enumerate() {
            let char_vec: Vec<String> = row.chars().map(|c| c.to_string()).collect();
            let inputs: Vec<&str> = char_vec.iter().map(|s| s.as_str()).collect();

            // Check if the input is valid
            if inputs.len() <= 4 && inputs.iter().all(|&bit| bit == "0" || bit == "1") {
                let sum_of_products: String = inputs
                    .iter()
                    .enumerate()
                    .map(|(i, &bit)| if bit == "1" { format!("/{}", (b'A' + i as u8) as char) } else { format!("{}", (b'A' + i as u8) as char) })
                    .collect::<Vec<String>>()
                    .join("");

                if !sum_of_products.is_empty() {
                    if index > 0 {
                        final_expression.push_str(" + ");
                    }

                    final_expression.push_str(&sum_of_products);
                }
            } else {
                println!("Please enter valid inputs for each row (up to 4 bits, 0 or 1).");
                break;
            }
        }

        println!("Sum of Products: {}", final_expression);
    }
}