use regex::Regex;
use std::fs;

fn main() {
    // Search text files for addresses
    search_addresses_in_text_files();

    // Search log files for events of interest
    search_log_files_for_events_of_interest();

    // Format phone numbers
    format_phone_numbers_from_file();
}

fn search_addresses_in_text_files() {
    let file_content = fs::read_to_string("X:\\Users\\grrtt\\source\\repos\\rust\\reg_ex\\sample_text_file.txt").expect("Failed to read file");

    // Regular expression for detecting addresses (simplified for demonstration)
    let address_regex = Regex::new(r"[^\n]+").unwrap();

    println!("Addresses found:");
    for address in address_regex.find_iter(&file_content) {
        println!("{}", address.as_str());
    }
}

fn search_log_files_for_events_of_interest() {
    let log_content = fs::read_to_string("X:\\Users\\grrtt\\source\\repos\\rust\\reg_ex\\sample_log_file.txt").expect("Failed to read log file");

    // Regular expressions for events of interest
    let event_regex = Regex::new(r"(ERROR|INFO|DEBUG): (\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}) - (Event \d{3}): (.+)").unwrap();

    println!("Events of interest found:");
    for event in event_regex.captures_iter(&log_content) {
        let event_type = &event[1];
        let timestamp = &event[2];
        let event_code = &event[3];
        let event_info = &event[4];
        println!("{}: {} - {}: {}", event_type, timestamp, event_code, event_info);
    }
}

fn format_phone_numbers_from_file() {
    let file_content = fs::read_to_string("X:\\Users\\grrtt\\source\\repos\\rust\\reg_ex\\sample_phone_numbers_file.txt").expect("Failed to read phone number file");

    // Regular expression for finding phone numbers and their associated country names
    let phone_regex = Regex::new(r"(\+\d{1,3}\s?\(?\d{1,4}\)?[-.\s]?\d{1,4}[-.\s]?\d{1,4}[-.\s]?\d{1,4})\s\((.*?)\)").unwrap();

    println!("Phone numbers found and formatted:");
    for phone_match in phone_regex.captures_iter(&file_content) {
        let phone_number = &phone_match[1];
        let country = &phone_match[2];
        let formatted_number = phone_number.replace(|c: char| !c.is_digit(10), "");
        println!("{} ({})", formatted_number, country);
    }
}
