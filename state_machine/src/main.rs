use std::io;
// TODO Implement 
// Define the TCP finite state machine
struct TCPFsm {
    fsm: std::collections::HashMap<&'static str, std::collections::HashMap<&'static str, &'static str>>,
}

impl TCPFsm {
    fn new() -> Self {
        let mut fsm = std::collections::HashMap::new();
        fsm.insert("CLOSED", [
            ("APP_PASSIVE_OPEN", "LISTEN"),
            ("APP_ACTIVE_OPEN", "SYN_SENT"),
        ].iter().cloned().collect());
        fsm.insert("LISTEN", [
            ("RCV_SYN", "SYN_RCVD"),
            ("APP_SEND", "SYN_SENT"),
            ("APP_CLOSE", "CLOSED"),
        ].iter().cloned().collect());
        fsm.insert("SYN_RCVD", [
            ("APP_CLOSE", "FIN_WAIT_1"),
            ("RCV_ACK", "ESTABLISHED"),
        ].iter().cloned().collect());
        fsm.insert("SYN_SENT", [
            ("RCV_SYN", "SYN_RCVD"),
            ("RCV_SYN_ACK", "ESTABLISHED"),
            ("APP_CLOSE", "CLOSED"),
        ].iter().cloned().collect());
        fsm.insert("ESTABLISHED", [
            ("APP_CLOSE", "FIN_WAIT_1"),
            ("RCV_FIN", "CLOSE_WAIT"),
        ].iter().cloned().collect());
        fsm.insert("FIN_WAIT_1", [
            ("RCV_FIN", "CLOSING"),
            ("RCV_FIN_ACK", "TIME_WAIT"),
            ("RCV_ACK", "FIN_WAIT_2"),
        ].iter().cloned().collect());
        fsm.insert("CLOSING", [
            ("RCV_ACK", "TIME_WAIT"),
        ].iter().cloned().collect());
        fsm.insert("FIN_WAIT_2", [
            ("RCV_FIN", "TIME_WAIT"),
        ].iter().cloned().collect());
        fsm.insert("TIME_WAIT", [
            ("APP_TIMEOUT", "CLOSED"),
        ].iter().cloned().collect());
        fsm.insert("CLOSE_WAIT", [
            ("APP_CLOSE", "LAST_ACK"),
        ].iter().cloned().collect());
        fsm.insert("LAST_ACK", [
            ("RCV_ACK", "CLOSED"),
        ].iter().cloned().collect());
        
        TCPFsm { fsm }
    }

    fn traverse_states(&self, events: Vec<&str>) -> Result<&'static str, &'static str> {
        let mut state = "CLOSED";
        for event in events {
            if let Some(transitions) = self.fsm.get(state) {
                if let Some(next_state) = transitions.get(event) {
                    state = *next_state;
                } else {
                    return Err("Invalid event for current state");
                }
            } else {
                return Err("Invalid state");
            }
        }
        Ok(state)
    }
}

fn main() {
    let fsm = TCPFsm::new();
    
    println!("Enter test case events separated by spaces:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let events: Vec<&str> = input.trim().split_whitespace().collect();
    
    match fsm.traverse_states(events) {
        Ok(state) => println!("Final state: {}", state),
        Err(err) => println!("Error: {}", err),
    }
}
