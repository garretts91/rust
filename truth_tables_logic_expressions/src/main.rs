use std::io;

// Function to generate the Sum of Products logic expression from a truth table
fn sum_of_products(truth_table: &Vec<Vec<u8>>, variables: &Vec<char>) -> String {
    truth_table
        .iter()
        .map(|row| {
            row.iter().enumerate().fold(String::new(), |mut term, (i, &value)| {
                if value == 0 {
                    term.push('/');
                }
                term.push(variables[i]);
                term
            })
        })
        .collect::<Vec<String>>()
        .join(" + ")
}

// Function to remove trailing " + " from the expression
fn remove_trailing_plus(expression: &str) -> String {
    let mut modified_expression = expression.to_string();
    if modified_expression.ends_with(" + ") {
        modified_expression.pop();
        modified_expression.pop();
    }
    modified_expression
}

// Function to read the number of variables from the user
fn read_num_variables() -> usize {
    println!("Enter the number of variables (up to 4): ");
    let mut num_variables = String::new();
    io::stdin().read_line(&mut num_variables).expect("Failed to read line");
    num_variables.trim().parse().expect("Invalid number")
}

// Function to read truth table rows from the user
fn read_truth_table_rows() -> Vec<Vec<u8>> {
    println!("Enter the truth table rows separated by commas: ");
    let mut input_rows = String::new();
    io::stdin().read_line(&mut input_rows).expect("Failed to read line");

    input_rows
        .split(',')
        .map(|row| row.trim().chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
        .collect()
}

// Function to generate and print the Sum of Products logic expression
fn generate_and_print_sum_of_products(truth_table: &Vec<Vec<u8>>, variables: &Vec<char>) {
    let sop_expression = sum_of_products(truth_table, variables);
    let modified_expression = remove_trailing_plus(&sop_expression);
    println!("Logic Expression (Sum of Products): {}", modified_expression);
}

fn main() {
    // Read the number of variables
    let num_variables = read_num_variables();

    // Read truth table rows
    let truth_table = read_truth_table_rows();

    // Generate and print the Sum of Products logic expression
    let variable_labels: Vec<char> = (b'A'..(b'A' + num_variables as u8)).map(char::from).collect();
    generate_and_print_sum_of_products(&truth_table, &variable_labels);
}
