// use std::collections::HashSet;
// use std::io;

// #[derive(Debug, Eq, PartialEq, Hash, Clone)]
// struct Proposition {
//     symbol: String,
//     negation: bool,
// }

// #[derive(Debug, Eq, PartialEq, Hash, Clone)]
// struct Rule {
//     antecedent: Proposition,
//     consequent: Proposition,
// }

// // Function to check if a proposition matches another proposition
// fn proposition_matches(p1: &Proposition, p2: &Proposition) -> bool {
//     p1.symbol == p2.symbol && p1.negation != p2.negation
// }

// // Function to apply a rule and generate a new proposition
// fn apply_rule(rule: &Rule, premises: &HashSet<Proposition>) -> Option<Proposition> {
//     let antecedent = &rule.antecedent;
//     let negated_antecedent = Proposition {
//         symbol: antecedent.symbol.clone(),
//         negation: !antecedent.negation,
//     };

//     let result = match &antecedent.symbol[..] {
//         "^" if !antecedent.negation => apply_commutative_law_and(premises),
//         "v" if !antecedent.negation => apply_commutative_law_or(premises),
//         "^" if antecedent.negation => apply_idempotent_law_and(premises),
//         "v" if antecedent.negation => apply_idempotent_law_or(premises),
//         "^" if !antecedent.negation => apply_associative_law_and(premises),
//         "v" if !antecedent.negation => apply_associative_law_or(premises),
//         "^" if !antecedent.negation => apply_distributive_law(premises),
//         "v" if !antecedent.negation => apply_distributive_law(premises),
//         "^" if !antecedent.negation => apply_identity_law_and(premises),
//         "v" if !antecedent.negation => apply_identity_law_or(premises),
//         "^" if antecedent.negation => apply_negation_law_and(premises),
//         "v" if antecedent.negation => apply_negation_law_or(premises),
//         "^" if !antecedent.negation => apply_null_law_and(premises),
//         "v" if !antecedent.negation => apply_null_law_or(premises),
//         "^" if !antecedent.negation => apply_absorption_law_and(premises),
//         "v" if !antecedent.negation => apply_absorption_law_or(premises),
//         "^" if antecedent.negation => apply_demorgans_law_and(premises),
//         "v" if antecedent.negation => apply_demorgans_law_or(premises),
//         "!" if !antecedent.negation => apply_involution_law_not(premises),
//         "->" if !antecedent.negation => apply_modus_ponens(premises),
//         "->" if antecedent.negation => apply_modus_tollens(premises),
//         "v" if !antecedent.negation => apply_disjunctive_addition(premises),
//         "^" if !antecedent.negation => apply_conjunctive_simplification(premises),
//         "v" if antecedent.negation => apply_disjunctive_simplification(premises),
//         "->" if !antecedent.negation => apply_chain_rule(premises),
//         "->" if !antecedent.negation => apply_conditional_equivalence(premises),
//         "<->" if !antecedent.negation => apply_biconditional_equivalences(premises),
//         "->" if !antecedent.negation => apply_contrapositive(premises),
//         _ => None,
//     };

//     if let Some(res) = result {
//         Some(res)
//     } else if premises.contains(antecedent) {
//         Some(rule.consequent.clone())
//     } else if premises.contains(&negated_antecedent) {
//         Some(Proposition {
//             symbol: rule.consequent.symbol.clone(),
//             negation: !rule.consequent.negation,
//         })
//     } else {
//         None
//     }
// }


// // Function to check if a proposition is within a complex proposition
// fn is_deduction_within_complex(complex_prop: &Proposition, deduction: &Proposition) -> bool {
//     if complex_prop.symbol.starts_with('(') {
//         // If the complex proposition starts with '(', it might contain the deduction
//         let inner_proposition = parse_input(complex_prop.symbol.clone());
//         return is_deduction_within_complex(&inner_proposition, deduction);
//     }

//     // Check if the deduction matches the complex proposition
//     let deduction_without_negation = Proposition {
//         symbol: deduction.symbol.clone(),
//         negation: false,
//     };

//     complex_prop.symbol == deduction_without_negation.symbol
//         && complex_prop.negation == deduction_without_negation.negation
// }

// // Function to apply the Commutative Law for ∧
// fn apply_commutative_law_and(premises: &HashSet<Proposition>) -> Option<Proposition> {
//     for prop1 in premises {
//         for prop2 in premises {
//             if prop1.symbol == prop2.symbol && prop1.negation == prop2.negation {
//                 return Some(prop1.clone());
//             }
//         }
//     }
//     None
// }

// // Function to apply the Commutative Law for ∨
// fn apply_commutative_law_or(premises: &HashSet<Proposition>) -> Option<Proposition> {
//     for prop1 in premises {
//         for prop2 in premises {
//             if prop1.symbol == prop2.symbol && prop1.negation == prop2.negation {
//                 return Some(prop1.clone());
//             }
//         }
//     }
//     None
// }

// // Function to apply the Associative Law for ∧
// fn apply_associative_law_and(premises: &HashSet<Proposition>) -> Option<Proposition> {
//     for prop1 in premises {
//         for prop2 in premises {
//             for prop3 in premises {
//                 if prop1.symbol == prop2.symbol && prop2.symbol == prop3.symbol && prop1.negation == prop2.negation {
//                     return Some(prop1.clone());
//                 }
//             }
//         }
//     }
//     None
// }

// // Function to apply the Associative Law for ∨
// fn apply_associative_law_or(premises: &HashSet<Proposition>) -> Option<Proposition> {
//     for prop1 in premises {
//         for prop2 in premises {
//             for prop3 in premises {
//                 if prop1.symbol == prop2.symbol && prop2.symbol == prop3.symbol && prop1.negation == prop2.negation {
//                     return Some(prop1.clone());
//                 }
//             }
//         }
//     }
//     None
// }

// // Function to apply the Distributive Law for ∧ and ∨
// fn apply_distributive_law(premises: &HashSet<Proposition>) -> Option<Proposition> {
//     // Assuming p∧(q∨r)⇔(p∧q)∨(p∧r), find matching patterns
//     for prop1 in premises {
//         for prop2 in premises {
//             for prop3 in premises {
//                 if prop1.symbol == prop2.symbol {
//                     // Implement the rest of the logic based on the specific properties
//                     // of the Distributive Law for ∨
//                     // ...
//                     return Some(prop1.clone());
//                 }
//             }
//         }
//     }
//     None
// }

// // Function to apply the Identity Law for ∧ and ∨
// fn apply_identity_law(premises: &HashSet<Proposition>) -> Option<Proposition> {
//     // Assuming p∨0⇔p, find matching patterns
//     for prop1 in premises {
//         for prop2 in premises {
//             if prop1.symbol == prop2.symbol {
//                 return Some(prop1.clone());
//             }
//         }
//     }
//     None
// }

// // Function to apply the Identity Law for ∧ and ∨
// fn apply_negation_law(premises: &HashSet<Proposition>) -> Option<Proposition> {
//     // Assuming p∧¬p⇔0, find matching patterns
//     for prop1 in premises {
//         for prop2 in premises {
//             if prop1.symbol == prop2.symbol && prop1.negation != prop2.negation {
//                 return Some(Proposition {
//                     symbol: "0".to_string(),
//                     negation: false,
//                 });
//             }
//         }
//     }
//     None
// }

// // Function to apply the Idempotent Law for ∧ and ∨
// fn apply_idempotent_law(premises: &HashSet<Proposition>) -> Option<Proposition> {
//     // Assuming p∨p⇔p, find matching patterns
//     for prop1 in premises {
//         for prop2 in premises {
//             if prop1.symbol == prop2.symbol {
//                 return Some(prop1.clone());
//             }
//         }
//     }
//     None
// }

// // Function to apply the Null Law for ∧ and ∨
// fn apply_null_law(premises: &HashSet<Proposition>) -> Option<Proposition> {
//     // Assuming p∧0⇔0, find matching patterns
//     for prop1 in premises {
//         for prop2 in premises {
//             if prop1.symbol == prop2.symbol && prop2.symbol == "0" {
//                 return Some(Proposition {
//                     symbol: "0".to_string(),
//                     negation: false,
//                 });
//             }
//         }
//     }
//     None
// }

// // Function to apply the Absorption Law for ∧ and ∨
// fn apply_absorption_law(premises: &HashSet<Proposition>) -> Option<Proposition> {
//     // Assuming p∧(p∨q)⇔p, find matching patterns
//     for prop1 in premises {
//         for prop2 in premises {
//             for prop3 in premises {
//                 if prop1.symbol == prop2.symbol && prop2.symbol == prop3.symbol {
//                     return Some(prop1.clone());
//                 }
//             }
//         }
//     }
//     None
// }

// // Function to apply DeMorgan's Law for ∧ and ∨
// fn apply_demorgans_law(premises: &HashSet<Proposition>) -> Option<Proposition> {
//     // Assuming ¬(p∧q)⇔(¬p)∨(¬q), find matching patterns
//     for prop1 in premises {
//         for prop2 in premises {
//             if prop1.symbol == prop2.symbol {
//                 return Some(Proposition {
//                     symbol: format!("¬{}", prop1.symbol),
//                     negation: true,
//                 });
//             }
//         }
//     }
//     None
// }

// // Function to apply Involution Law for ¬
// fn apply_involution_law(premises: &HashSet<Proposition>) -> Option<Proposition> {
//     // Assuming ¬(¬p)⇔p, find matching patterns
//     for prop1 in premises {
//         for prop2 in premises {
//             if prop1.symbol == format!("¬{}", prop2.symbol) && prop1.negation != prop2.negation {
//                 return Some(Proposition {
//                     symbol: prop2.symbol.clone(),
//                     negation: prop2.negation,
//                 });
//             }
//         }
//     }
//     None
// }

// // Function to apply Detachment (Modus Ponens) Law
// fn apply_modus_ponens(premises: &HashSet<Proposition>) -> Option<Proposition> {
//     // Assuming (p→q)∧p⇒q, find matching patterns
//     for prop1 in premises {
//         for prop2 in premises {
//             if prop1.symbol == format!("{}→{}", prop2.symbol, prop2.symbol)
//                 && prop1.negation == prop2.negation
//             {
//                 return Some(prop2.clone());
//             }
//         }
//     }
//     None
// }

// // Function to apply Indirect Reasoning (Modus Tollens) Law
// fn apply_modus_tollens(premises: &HashSet<Proposition>) -> Option<Proposition> {
//     // Assuming (p→q)∧¬q⇒¬p, find matching patterns
//     for prop1 in premises {
//         for prop2 in premises {
//             for prop3 in premises {
//                 if prop1.symbol == format!("{}→{}", prop2.symbol, prop3.symbol)
//                     && prop2.symbol == format!("¬{}", prop3.symbol)
//                     && prop1.negation == prop3.negation
//                 {
//                     return Some(Proposition {
//                         symbol: format!("¬{}", prop1.symbol),
//                         negation: !prop1.negation,
//                     });
//                 }
//             }
//         }
//     }
//     None
// }

// // Function to apply Disjunctive Addition Law
// fn apply_disjunctive_addition(premises: &HashSet<Proposition>) -> Option<Proposition> {
//     // Assuming p⇒(p∨q), find matching patterns
//     for prop1 in premises {
//         for prop2 in premises {
//             if prop1.symbol == format!("{}→{}", prop1.symbol, prop2.symbol)
//                 && prop1.negation == prop2.negation
//             {
//                 return Some(Proposition {
//                     symbol: format!("{}∨{}", prop1.symbol, prop2.symbol),
//                     negation: prop1.negation,
//                 });
//             }
//         }
//     }
//     None
// }

// // Function to apply Conjunctive Simplification Law
// fn apply_conjunctive_simplification(premises: &HashSet<Proposition>) -> Option<Proposition> {
//     // Assuming (p∧q)⇒p and (p∧q)⇒q, find matching patterns
//     for prop1 in premises {
//         for prop2 in premises {
//             for prop3 in premises {
//                 if prop1.symbol == format!("{}∧{}", prop2.symbol, prop3.symbol)
//                     && prop2.symbol == prop1.symbol
//                     && prop3.symbol == prop1.symbol
//                     && prop1.negation == prop2.negation
//                     && prop1.negation == prop3.negation
//                 {
//                     return Some(prop1.clone());
//                 }
//             }
//         }
//     }
//     None
// }

// // Function to apply Disjunctive Simplification Law
// fn apply_disjunctive_simplification(premises: &HashSet<Proposition>) -> Option<Proposition> {
//     // Assuming (p∨q)∧¬p⇒q and (p∨q)∧¬q⇒p, find matching patterns
//     for prop1 in premises {
//         for prop2 in premises {
//             for prop3 in premises {
//                 if prop1.symbol == format!("{}∨{}", prop2.symbol, prop3.symbol)
//                     && prop2.symbol == format!("¬{}", prop3.symbol)
//                     && prop1.negation == prop3.negation
//                 {
//                     return Some(prop2.clone());
//                 }
//                 if prop1.symbol == format!("{}∨{}", prop2.symbol, prop3.symbol)
//                     && prop3.symbol == format!("¬{}", prop2.symbol)
//                     && prop1.negation == prop2.negation
//                 {
//                     return Some(prop3.clone());
//                 }
//             }
//         }
//     }
//     None
// }

// // Function to apply Chain Rule Law
// fn apply_chain_rule(premises: &HashSet<Proposition>) -> Option<Proposition> {
//     // Assuming (p→q)∧(q→r)⇒(p→r), find matching patterns
//     for prop1 in premises {
//         for prop2 in premises {
//             for prop3 in premises {
//                 if prop1.symbol == format!("{}→{}", prop2.symbol, prop3.symbol)
//                     && prop2.symbol == format!("{}→{}", prop3.symbol, prop1.symbol)
//                     && prop1.negation == prop3.negation
//                 {
//                     return Some(prop1.clone());
//                 }
//             }
//         }
//     }
//     None
// }

// // Function to apply Conditional Equivalence Law
// fn apply_conditional_equivalence(premises: &HashSet<Proposition>) -> Option<Proposition> {
//     // Assuming p→q⇔¬p∨q, find matching patterns
//     for prop1 in premises {
//         for prop2 in premises {
//             if prop1.symbol == format!("{}→{}", prop2.symbol, prop2.symbol)
//                 && prop1.negation != prop2.negation
//             {
//                 return Some(Proposition {
//                     symbol: format!("¬{}∨{}", prop1.symbol, prop2.symbol),
//                     negation: prop1.negation,
//                 });
//             }
//         }
//     }
//     None
// }

// // Function to apply Biconditional Equivalences Law
// fn apply_biconditional_equivalences(premises: &HashSet<Proposition>) -> Option<Proposition> {
//     // Assuming (p↔q)⇔(p→q)∧(q→p)⇔(p∧q)∨(¬p∧¬q), find matching patterns
//     for prop1 in premises {
//         for prop2 in premises {
//             for prop3 in premises {
//                 if prop1.symbol == format!("{}→{}", prop2.symbol, prop3.symbol)
//                     && prop2.symbol == format!("{}→{}", prop3.symbol, prop1.symbol)
//                     && prop1.negation == prop3.negation
//                 {
//                     return Some(Proposition {
//                         symbol: format!("({}∧{})∨(¬{}∧¬{})", prop1.symbol, prop3.symbol, prop1.symbol, prop3.symbol),
//                         negation: prop1.negation,
//                     });
//                 }
//             }
//         }
//     }
//     None
// }

// // Function to apply Contrapositive Law
// fn apply_contrapositive(premises: &HashSet<Proposition>) -> Option<Proposition> {
//     // Assuming (p→q)⇔(¬q→¬p), find matching patterns
//     for prop1 in premises {
//         for prop2 in premises {
//             if prop1.symbol == format!("{}→{}", prop1.symbol, prop2.symbol)
//                 && prop1.negation != prop2.negation
//             {
//                 return Some(Proposition {
//                     symbol: format!("¬{}→¬{}", prop2.symbol, prop1.symbol),
//                     negation: !prop1.negation,
//                 });
//             }
//         }
//     }
//     None
// }

// // Function to perform deduction and determine validity
// fn validate_deduction(
//     premises: &Vec<Proposition>,
//     deduction: &Proposition,
//     rules: &Vec<Rule>,
// ) -> Option<Vec<(Rule, Proposition)>> {
//     let mut current_premises: HashSet<Proposition> = HashSet::new();
//     let mut proofs: Vec<(Rule, Proposition)> = Vec::new();

//     // Start with the last premise
//     current_premises.insert(premises.last().unwrap().clone());

//     while !current_premises.is_empty() {
//         let current_proposition = current_premises.iter().next().unwrap().clone();
//         current_premises.remove(&current_proposition);

//         // Apply rules to generate new propositions
//         for rule in rules {
//             if let Some(new_proposition) = apply_rule(rule, &current_premises) {
//                 if !premises.contains(&new_proposition) {
//                     current_premises.insert(new_proposition.clone());
//                     proofs.push((rule.clone(), new_proposition.clone()));

//                     // Check if the deduction is valid
//                     if is_deduction_within_complex(&new_proposition, deduction) {
//                         return Some(proofs);
//                     }
//                 }
//             }
//         }
//     }

//     None
// }

// // Function to print the proof with ASCII substitutes
// fn print_proof(proofs: &Vec<(Rule, Proposition)>) {
//     for proof in proofs.iter().rev() {
//         let substitute_symbol = match &proof.1.symbol[..] {
//             "*" => "∧",
//             "v" => "∨",
//             ">" => "→",
//             "!" => "~",
//             "A" => "∀",
//             "E" => "∃",
//             "R" => "∴",
//             _ => &proof.1.symbol,
//         };
//         println!(
//             "{} {:?} (by rule {:?})",
//             substitute_symbol, proof.1.negation, proof.0
//         );
//     }
// }

fn main(){
    println!("Hello, World");
}

// fn main() {
//     // Introduction and instructions
//     println!("Welcome to the Deduction Validator!");
//     println!("Please enter your premises one at a time, followed by 'done'.");
//     println!("For example, enter 'p' for 'p' or '¬p' for 'not p'.");
//     println!("Use symbols: ∧ for AND, ∨ for OR, → for IMPLIES, ¬ for NOT, ∀ for FOR ALL, ∃ for EXISTS, ∴ for THEREFORE.");
//     println!("ASCII substitutes: * for ∧, v for ∨, > for →, ! for ¬, A for ∀, E for ∃, R for ∴.");
//     println!();

//     // Read input premises and deduction from the terminal
//     let mut premises: Vec<Proposition> = Vec::new();

//     loop {
//         println!("Enter premise (or 'done' to finish): ");
//         let input = read_line();
//         if input.trim().to_lowercase() == "done" {
//             break;
//         }

//         let proposition = parse_input(input);
//         premises.push(proposition);
//     }

//     println!();
//     println!("Great! Now enter your deduction.");
//     println!("For example, enter 'q' for 'q' or '¬q' for 'not q'.");
//     println!();

//     println!("Enter deduction: ");
//     let deduction_input = read_line();
//     let deduction = parse_input(deduction_input);

//     // Example rules, including Modus Ponens, Modus Tollens, and Law of Syllogism
//     let rules = vec![
//         Rule {
//             antecedent: Proposition {
//                 symbol: "p".to_string(),
//                 negation: false,
//             },
//             consequent: Proposition {
//                 symbol: "q".to_string(),
//                 negation: false,
//             },
//         },
//         Rule {
//             antecedent: Proposition {
//                 symbol: "not p".to_string(),
//                 negation: true,
//             },
//             consequent: Proposition {
//                 symbol: "q".to_string(),
//                 negation: false,
//             },
//         },
//         Rule {
//             antecedent: Proposition {
//                 symbol: "p".to_string(),
//                 negation: false,
//             },
//             consequent: Proposition {
//                 symbol: "r".to_string(),
//                 negation: false,
//             },
//         },
//         Rule {
//             antecedent: Proposition {
//                 symbol: "r".to_string(),
//                 negation: false,
//             },
//             consequent: Proposition {
//                 symbol: "s".to_string(),
//                 negation: false,
//             },
//         },
//         // Add more rules as needed
//     ];

//     println!();
//     // Validate deduction and print proof
//     if let Some(proofs) = validate_deduction(&premises, &deduction, &rules) {
//         println!("Deduction is valid. Proof:");
//         print_proof(&proofs);
//     } else {
//         println!("Deduction is invalid or has insufficient information.");
//     }
// }

// // Helper function to read a line from the terminal
// fn read_line() -> String {
//     let mut input = String::new();
//     io::stdin().read_line(&mut input).expect("Failed to read line");
//     input.trim().to_string()
// }

// // Helper function to parse input into Proposition
// fn parse_input(input: String) -> Proposition {
//     let mut chars = input.chars().filter(|&c| !c.is_whitespace()).peekable();
//     parse_proposition(&mut chars)
// }

// fn parse_proposition<I>(chars: &mut std::iter::Peekable<I>) -> Proposition
// where
//     I: Iterator<Item = char>,
// {
//     let symbol = chars.next().expect("Empty input").to_string();
//     let negation = if chars.peek() == Some(&'¬') { true } else { false };

//     // Check for additional symbols
//     if let Some(operator) = chars.next() {
//         if operator == '(' {
//             // Handle the case of complex propositions within parentheses
//             let antecedent = parse_proposition(chars);
//             chars.next(); // Consume the closing parenthesis
//             let consequent = parse_proposition(chars);

//             return Proposition {
//                 symbol: format!("({} {} {})", antecedent.symbol, operator, consequent.symbol),
//                 negation,
//             };
//         } else {
//             // Consume the operator and add it to the symbol
//             let operator_str = match operator {
//                 '*' => "∧".to_string(),
//                 'v' => "∨".to_string(),
//                 '>' => "→".to_string(),
//                 '!' => "~".to_string(),
//                 'A' => "∀".to_string(),
//                 'E' => "∃".to_string(),
//                 'R' => "∴".to_string(),
//                 _ => operator.to_string(),
//             };
            
//             return Proposition {
//                 symbol: format!("{}{}", symbol, operator_str),
//                 negation,
//             };
//         }
//     }

//     Proposition {
//         symbol,
//         negation,
//     }
// }
