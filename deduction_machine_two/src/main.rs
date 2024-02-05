use std::collections::HashSet;
use std::io;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Proposition {
    symbol: String,
    negation: bool,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Rule {
    name: String,
    antecedent: Proposition,
    consequent: Proposition,
}

// Function to check if a proposition matches another proposition
fn proposition_matches(p1: &Proposition, p2: &Proposition) -> bool {
    p1.symbol == p2.symbol && p1.negation != p2.negation
}

// Function to apply a rule and generate a new proposition
fn apply_rule(rule: &Rule, premises: &HashSet<Proposition>) -> Option<Proposition> {
    if premises.contains(&rule.antecedent) {
        Some(rule.consequent.clone())
    } else {
        None
    }
}

// Function to perform deduction and determine validity
fn validate_deduction(
    premises: &Vec<Proposition>,
    deduction: &Proposition,
    rules: &Vec<Rule>,
) -> Option<Vec<(Rule, Proposition)>> {
    let mut current_premises: HashSet<Proposition> = HashSet::new();
    let mut proofs: Vec<(Rule, Proposition)> = Vec::new();

    // Start with the last premise
    current_premises.insert(premises.last().unwrap().clone());

    while !current_premises.is_empty() {
        let current_proposition = current_premises.iter().next().unwrap().clone();
        current_premises.remove(&current_proposition);

        // Apply rules to generate new propositions
        for rule in rules {
            if let Some(new_proposition) = apply_rule(rule, &current_premises) {
                if !premises.contains(&new_proposition) {
                    current_premises.insert(new_proposition.clone());
                    proofs.push((rule.clone(), new_proposition.clone()));

                    // Check if the deduction is valid
                    if proposition_matches(&new_proposition, deduction) {
                        return Some(proofs);
                    }
                }
            }
        }
    }

    None
}

// Function to print the proof with ASCII substitutes
fn print_proof(proofs: &Vec<(Rule, Proposition)>) {
    for proof in proofs.iter().rev() {
        let substitute_symbol = match &proof.1.symbol[..] {
            "∧" => "*",
            "∨" => "+",
            "→" => ">",
            "¬" => "~",
            "∀" => "A",
            "∃" => "E",
            "∴" => "R",
            _ => &proof.1.symbol,
        };
        println!(
            "{} {:?} (by rule {})",
            substitute_symbol, proof.1.negation, proof.0.name
        );
    }
}

fn main() {
    // Introduction and instructions
    println!("Welcome to the Deduction Validator!");
    println!("Please enter your premises one at a time, followed by 'done'.");
    println!("For example, enter 'p' for 'p' or '¬p' for 'not p'.");
    println!("Use symbols: ∧ for AND, ∨ for OR, → for IMPLIES, ¬ for NOT, ∀ for FOR ALL, ∃ for EXISTS, ∴ for THEREFORE.");
    println!("ASCII substitutes: * for ∧, + for ∨, > for →, ~ for ¬, A for ∀, E for ∃, R for ∴.");
    println!();

    // Read input premises and deduction from the terminal
    let mut premises: Vec<Proposition> = Vec::new();

    loop {
        println!("Enter premise (or 'done' to finish): ");
        let input = read_line();
        if input.trim().to_lowercase() == "done" {
            break;
        }

        let proposition = parse_input(input);
        premises.push(proposition);
    }

    println!();
    println!("Great! Now enter your deduction.");
    println!("For example, enter 'q' for 'q' or '¬q' for 'not q'.");
    println!();

    println!("Enter deduction: ");
    let deduction_input = read_line();
    let deduction = parse_input(deduction_input);

    // Debug print to check the premises and deduction
    println!("Premises: {:?}", premises);
    println!("Deduction: {:?}", deduction);

    // Define logical rules
    let rules = vec![
        // Modus Ponens
        Rule {
            name: "Modus Ponens".to_string(),
            antecedent: Proposition {
                symbol: "(p ∧ (p → q))".to_string(),
                negation: false,
            },
            consequent: Proposition {
                symbol: "q".to_string(),
                negation: false,
            },
        },
        // Modus Tollens
        Rule {
            name: "Modus Tollens".to_string(),
            antecedent: Proposition {
                symbol: "((¬q) ∧ (p → q))".to_string(),
                negation: false,
            },
            consequent: Proposition {
                symbol: "(¬p)".to_string(),
                negation: true,
            },
        },
        // Commutative Law for AND
        Rule {
            name: "Commutative Law for AND".to_string(),
            antecedent: Proposition {
                symbol: "(p ∧ q)".to_string(),
                negation: false,
            },
            consequent: Proposition {
                symbol: "(q ∧ p)".to_string(),
                negation: false,
            },
        },
        // Associative Law for OR
        Rule {
            name: "Associative Law for OR".to_string(),
            antecedent: Proposition {
                symbol: "(p ∨ (q ∨ r))".to_string(),
                negation: false,
            },
            consequent: Proposition {
                symbol: "((p ∨ q) ∨ r)".to_string(),
                negation: false,
            },
        },
        // Distributive Law for AND over OR (left distribution)
        Rule {
            name: "Distributive Law for AND over OR (Left Distribution)".to_string(),
            antecedent: Proposition {
                symbol: "(p ∧ (q ∨ r))".to_string(),
                negation: false,
            },
            consequent: Proposition {
                symbol: "((p ∧ q) ∨ (p ∧ r))".to_string(),
                negation: false,
            },
        },
        // Distributive Law for AND over OR (right distribution)
        Rule {
            name: "Distributive Law for AND over OR (Right Distribution)".to_string(),
            antecedent: Proposition {
                symbol: "((q ∨ r) ∧ p)".to_string(),
                negation: false,
            },
            consequent: Proposition {
                symbol: "((q ∧ p) ∨ (r ∧ p))".to_string(),
                negation: false,
            },
        },
        // Identity Law for AND (left identity)
        Rule {
            name: "Identity Law for AND (Left Identity)".to_string(),
            antecedent: Proposition {
                symbol: "(true ∧ p)".to_string(),
                negation: false,
            },
            consequent: Proposition {
                symbol: "p".to_string(),
                negation: false,
            },
        },
        // Identity Law for AND (right identity)
        Rule {
            name: "Identity Law for AND (Right Identity)".to_string(),
            antecedent: Proposition {
                symbol: "(p ∧ true)".to_string(),
                negation: false,
            },
            consequent: Proposition {
                symbol: "p".to_string(),
                negation: false,
            },
        },
        // Negation Law for AND
        Rule {
            name: "Negation Law for AND".to_string(),
            antecedent: Proposition {
                symbol: "(p ∧ ¬p)".to_string(),
                negation: false,
            },
            consequent: Proposition {
                symbol: "false".to_string(),
                negation: false,
            },
        },
        // Idempotent Law for AND
        Rule {
            name: "Idempotent Law for AND".to_string(),
            antecedent: Proposition {
                symbol: "(p ∧ p)".to_string(),
                negation: false,
            },
            consequent: Proposition {
                symbol: "p".to_string(),
                negation: false,
            },
        },
        // Null Law for AND (left null)
        Rule {
            name: "Null Law for AND (Left Null)".to_string(),
            antecedent: Proposition {
                symbol: "(false ∧ p)".to_string(),
                negation: false,
            },
            consequent: Proposition {
                symbol: "false".to_string(),
                negation: false,
            },
        },
        // Null Law for AND (right null)
        Rule {
            name: "Null Law for AND (Right Null)".to_string(),
            antecedent: Proposition {
                symbol: "(p ∧ false)".to_string(),
                negation: false,
            },
            consequent: Proposition {
                symbol: "false".to_string(),
                negation: false,
            },
        },
        // Absorption Law (left absorption)
        Rule {
            name: "Absorption Law (Left Absorption)".to_string(),
            antecedent: Proposition {
                symbol: "(p ∨ (p ∧ q))".to_string(),
                negation: false,
            },
            consequent: Proposition {
                symbol: "p".to_string(),
                negation: false,
            },
        },
        // Absorption Law (right absorption)
        Rule {
            name: "Absorption Law (Right Absorption)".to_string(),
            antecedent: Proposition {
                symbol: "(p ∧ (p ∨ q))".to_string(),
                negation: false,
            },
            consequent: Proposition {
                symbol: "p".to_string(),
                negation: false,
            },
        },
        // DeMorgan's Law (for AND)
        Rule {
            name: "DeMorgan's Law (for AND)".to_string(),
            antecedent: Proposition {
                symbol: "(¬(p ∧ q))".to_string(),
                negation: false,
            },
            consequent: Proposition {
                symbol: "((¬p) ∨ (¬q))".to_string(),
                negation: false,
            },
        },
        // DeMorgan's Law (for OR)
        Rule {
            name: "DeMorgan's Law (for OR)".to_string(),
            antecedent: Proposition {
                symbol: "(¬(p ∨ q))".to_string(),
                negation: false,
            },
            consequent: Proposition {
                symbol: "((¬p) ∧ (¬q))".to_string(),
                negation: false,
            },
        },
        // Involution Law
        Rule {
            name: "Involution Law".to_string(),
            antecedent: Proposition {
                symbol: "(¬(¬p))".to_string(),
                negation: false,
            },
            consequent: Proposition {
                symbol: "p".to_string(),
                negation: false,
            },
        },
    ];

    println!();
    // Validate deduction and print proof
    if let Some(proofs) = validate_deduction(&premises, &deduction, &rules) {
        println!("Deduction is valid. Proof:");
        print_proof(&proofs);
    } else {
        println!("Deduction is invalid or has insufficient information.");
    }
}

// Helper function to read a line from the terminal
fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

// Helper function to parse input into Proposition
fn parse_input(input: String) -> Proposition {
    let mut chars = input.chars().filter(|&c| c != ' ' && c != '(' && c != ')');
    let symbol = chars.next().expect("Empty input").to_string();
    let negation = if chars.next() == Some('~') { true } else { false }; // Use '~' for NOT
    Proposition { symbol, negation }
}