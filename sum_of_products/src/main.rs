use std::io;

// Function to convert u8 value to String
fn u8_to_string(value: &u8) -> String {
    value.to_string()
}

// Function to convert a row of u8 values to Vec<String>
fn row_to_string_vector(row: &Vec<u8>) -> Vec<String> {
    row.iter().map(u8_to_string).collect()
}

// Function to create the header string
fn create_header_string(variables: &Vec<char>) -> String {
    variables.iter().collect::<String>() + " | F"
}

// Function to create the dividing line string
fn create_dividing_line(variables_len: usize) -> String {
    "-".repeat(variables_len * 2 + 4)
}

// Function to print a single row of the truth table
fn print_truth_table_row(values: &Vec<String>, function_value: &u8) {
    println!("{} | {}", values.join(" "), function_value);
}

// Function to print the truth table
fn print_truth_table(variables: &Vec<char>, truth_table: &Vec<Vec<u8>>) {
    // Create and print header
    let header = create_header_string(variables);
    println!("{}", header);

    // Create and print dividing line
    let dividing_line = create_dividing_line(variables.len());
    println!("{}", dividing_line);

    // Iterate over both rows and values simultaneously
    for (row, values) in truth_table.iter().zip(truth_table.iter().map(row_to_string_vector)) {
        // Print a single row of the truth table
        print_truth_table_row(&values, row.last().unwrap());
    }
}

// Function to create a term for a single row
fn create_term_for_row(row: &Vec<u8>, variables: &Vec<char>) -> String {
    row.iter().enumerate().fold(String::new(), |mut term, (i, &value)| {
        if value == 0 {
            term.push('/');
        }
        term.push(variables[i]);
        term
    })
}

// Function to create the Sum of Products expression from the truth table
fn create_sop_expression(truth_table: &Vec<Vec<u8>>, variables: &Vec<char>) -> String {
    truth_table
        .iter()
        .map(|row| create_term_for_row(row, variables))
        .collect::<Vec<String>>()
        .join(" + ")
}

// Function to remove trailing " + " from the expression
fn remove_trailing_plus(expression: &mut String) {
    expression.pop();
    expression.pop();
}

// Function to generate Sum of Products logic expression from a truth table
fn sum_of_products(truth_table: &Vec<Vec<u8>>, variables: &Vec<char>) -> String {
    let mut sop_expression = create_sop_expression(truth_table, variables);
    remove_trailing_plus(&mut sop_expression);
    sop_expression
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

// Function to generate and print the truth table
fn generate_and_print_truth_table(variables: &Vec<char>, truth_table: &Vec<Vec<u8>>) {
    print_truth_table(variables, truth_table);
}

// Function to generate and print the Sum of Products logic expression
fn generate_and_print_sop_expression(truth_table: &Vec<Vec<u8>>, variables: &Vec<char>) {
    let sop_expression = sum_of_products(truth_table, variables);
    println!("Logic Expression (Sum of Products): {}", sop_expression);
}

fn main() {
    // Read the number of variables
    let num_variables = read_num_variables();

    // Read truth table rows
    let truth_table = read_truth_table_rows();

    // Generate and print the truth table
    let variable_labels: Vec<char> = (b'A'..(b'A' + num_variables as u8)).map(char::from).collect();
    generate_and_print_truth_table(&variable_labels, &truth_table);

    // Generate and print the Sum of Products logic expression
    generate_and_print_sop_expression(&truth_table, &variable_labels);
}
