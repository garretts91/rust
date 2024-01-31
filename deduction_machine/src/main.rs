use std::collections::{HashSet, HashMap};

#[derive(Debug, Eq, PartialEq, Hash, Clone)] // Derive Clone for Proposition
struct Proposition {
    symbol: String,
    negation: bool,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)] // Derive Clone for Rule
struct Rule {
    antecedent: Proposition,
    consequent: Proposition,
}

// Implement Clone manually for Proposition
impl Clone for Proposition {
    fn clone(&self) -> Self {
        Proposition {
            symbol: self.symbol.clone(),
            negation: self.negation,
        }
    }
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
fn validate_deduction(premises: &Vec<Proposition>, deduction: &Proposition, rules: &Vec<Rule>) -> Option<Vec<(Rule, Proposition)>> {
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

// Function to print the proof
fn print_proof(proofs: &Vec<(Rule, Proposition)>) {
    for proof in proofs.iter().rev() {
        println!("{} {:?} (by rule {:?})", proof.1.symbol, proof.1.negation, proof.0);
    }
}

fn main() {
    // Example premises and deduction
    let premises = vec![
        Proposition { symbol: "m".to_string(), negation: false },
        Proposition { symbol: "f".to_string(), negation: false },
        Proposition { symbol: "s".to_string(), negation: false },
        Proposition { symbol: "b".to_string(), negation: false },
        Proposition { symbol: "t".to_string(), negation: false },
        Proposition { symbol: "j".to_string(), negation: false },
    ];

    let deduction = Proposition { symbol: "j".to_string(), negation: false };

    // Example rules
    let rules = vec![
        Rule { antecedent: Proposition { symbol: "m".to_string(), negation: false }, consequent: Proposition { symbol: "j".to_string(), negation: false } },
        Rule { antecedent: Proposition { symbol: "f".to_string(), negation: false }, consequent: Proposition { symbol: "m".to_string(), negation: false } },
        Rule { antecedent: Proposition { symbol: "b".to_string(), negation: false }, consequent: Proposition { symbol: "t".to_string(), negation: false } },
        Rule { antecedent: Proposition { symbol: "f".to_string(), negation: false }, consequent: Proposition { symbol: "t".to_string(), negation: true } },
    ];

    // Validate deduction and print proof
    if let Some(proofs) = validate_deduction(&premises, &deduction, &rules) {
        println!("Deduction is valid. Proof:");
        print_proof(&proofs);
    } else {
        println!("Deduction is invalid or has insufficient information.");
    }
}
