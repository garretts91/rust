use std::collections::{HashSet, HashMap};
use std::io::{self, Write};

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Proposition {
    symbol: String,
    negation: bool,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Rule {
    antecedent: Proposition,
    consequent: Proposition,
}

struct DeductionSystem {
    premises: HashSet<Proposition>,
    rules: Vec<Rule>,
    applied_rules: HashMap<Rule, HashSet<Proposition>>,
}

impl DeductionSystem {
    fn new(premises: HashSet<Proposition>, rules: Vec<Rule>) -> Self {
        DeductionSystem {
            premises,
            rules,
            applied_rules: HashMap::new(),
        }
    }

    // Simplified function to apply rules and attempt to deduce the target
    fn validate_deduction(&mut self, deduction: &Proposition) -> bool {
        let mut new_premises = true;
        while new_premises {
            new_premises = false;
            for rule in &self.rules {
                if let Some(new_prop) = self.apply_rule(rule) {
                    if !self.premises.contains(&new_prop) {
                        self.premises.insert(new_prop.clone());
                        new_premises = true;
                        if &new_prop == deduction {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }

    fn apply_rule(&mut self, rule: &Rule) -> Option<Proposition> {
        if self.premises.contains(&rule.antecedent) && !self.applied_rules.entry(rule.clone()).or_insert_with(HashSet::new).contains(&rule.antecedent) {
            self.applied_rules.get_mut(rule).unwrap().insert(rule.antecedent.clone());
            return Some(rule.consequent.clone());
        }
        None
    }
}

fn main() {
    let premises = vec![
        Proposition { symbol: "p".to_string(), negation: false },
        Expr::Implies(
            Box::new(Expr::And(
                Box::new(Expr::Var("m".to_string())),
                Box::new(Expr::Not(Box::new(Expr::Var("b".to_string())))),
            )),
            Box::new(Expr::Var("j".to_string())),
        ),
        Expr::Implies(
            Box::new(Expr::Or(
                Box::new(Expr::Var("f".to_string())),
                Box::new(Expr::Var("s".to_string())),
            )),
            Box::new(Expr::Var("m".to_string())),
        ),
        Expr::Implies(
            Box::new(Expr::Var("b".to_string())),
            Box::new(Expr::Var("t".to_string())),
        ),
        Expr::Implies(
            Box::new(Expr::Var("f".to_string())),
            Box::new(Expr::Not(Box::new(Expr::Var("t".to_string())))),
        ),
//     ];
    ].into_iter().collect();

    let rules = vec![
        Rule {
            antecedent: Proposition { symbol: "p".to_string(), negation: false },
            consequent: Proposition { symbol: "q".to_string(), negation: false },
        },
        // Define more rules as needed
    ];

    let mut deduction_system = DeductionSystem::new(premises, rules);
    let deduction = Proposition { symbol: "q".to_string(), negation: false };

    if deduction_system.validate_deduction(&deduction) {
        println!("Deduction is valid!");
    } else {
        println!("Deduction is invalid or cannot be proven with given rules and premises.");
    }
}


// use std::collections::HashMap;

// enum Expr {
//     Var(String),
//     Not(Box<Expr>),
//     And(Box<Expr>, Box<Expr>),
//     Or(Box<Expr>, Box<Expr>),
//     Implies(Box<Expr>, Box<Expr>),
// }

// impl Expr {
//     fn evaluate(&self, valuation: &HashMap<String, bool>) -> bool {
//         match self {
//             Expr::Var(var) => *valuation.get(var).unwrap_or(&false),
//             Expr::Not(expr) => !expr.evaluate(valuation),
//             Expr::And(a, b) => a.evaluate(valuation) && b.evaluate(valuation),
//             Expr::Or(a, b) => a.evaluate(valuation) || b.evaluate(valuation),
//             Expr::Implies(a, b) => !a.evaluate(valuation) || b.evaluate(valuation),
//         }
//     }
// }

// // Function to generate all possible combinations of truth values for the variables
// fn generate_valuations(vars: Vec<String>, index: usize, current: &mut HashMap<String, bool>, results: &mut Vec<HashMap<String, bool>>) {
//     if index == vars.len() {
//         results.push(current.clone());
//         return;
//     }

//     let var = vars[index].clone();
//     current.insert(var.clone(), true);
//     generate_valuations(vars.clone(), index + 1, current, results);
//     current.insert(var.clone(), false);
//     generate_valuations(vars.clone(), index + 1, current, results);
// }

// fn main() {
//     // Define the logical expressions based on the provided argument
//     let premises = vec![
//         Expr::Implies(
//             Box::new(Expr::And(
//                 Box::new(Expr::Var("m".to_string())),
//                 Box::new(Expr::Not(Box::new(Expr::Var("b".to_string())))),
//             )),
//             Box::new(Expr::Var("j".to_string())),
//         ),
//         Expr::Implies(
//             Box::new(Expr::Or(
//                 Box::new(Expr::Var("f".to_string())),
//                 Box::new(Expr::Var("s".to_string())),
//             )),
//             Box::new(Expr::Var("m".to_string())),
//         ),
//         Expr::Implies(
//             Box::new(Expr::Var("b".to_string())),
//             Box::new(Expr::Var("t".to_string())),
//         ),
//         Expr::Implies(
//             Box::new(Expr::Var("f".to_string())),
//             Box::new(Expr::Not(Box::new(Expr::Var("t".to_string())))),
//         ),
//     ];

//     let conclusion = Expr::Var("j".to_string()); // Example: Define your conclusion here

//     // Identify all unique variables involved in premises and conclusion
//     let vars = vec!["f".to_string(), "b".to_string(), "m".to_string(), "j".to_string(), "s".to_string(), "t".to_string()];

//     // Generate all possible valuations
//     let mut results = Vec::new();
//     generate_valuations(vars, 0, &mut HashMap::new(), &mut results);

//     // Evaluate the premises and conclusion for each valuation
//     let mut valid = true;
//     for valuation in results.iter() {
//         let premises_true = premises.iter().all(|premise| premise.evaluate(&valuation));
//         let conclusion_true = conclusion.evaluate(&valuation);

//         if premises_true && !conclusion_true {
//             valid = false;
//             break;
//         }
//     }

//     if valid {
//         println!("The deduction is valid under all valuations.");
//     } else {
//         println!("The deduction is invalid under some valuations.");
//     }
// }
