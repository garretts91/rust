use std::io;

fn is_reflexive<T: PartialEq>(pairs: &[(T, T)], set: &[T]) -> bool {
    for elem in set {
        if !pairs.iter().any(|(x, y)| x == elem && y == elem) {
            return false;
        }
    }
    true
}

fn is_symmetric<T: PartialEq>(pairs: &[(T, T)]) -> bool {
    pairs.iter().all(|(x, y)| pairs.iter().any(|(a, b)| x == b && y == a))
}

fn is_transitive<T: PartialEq>(pairs: &[(T, T)]) -> bool {
    pairs.iter().all(|(x, y)| 
        pairs.iter().all(|(a, b)| 
            y != a || pairs.iter().any(|(c, d)| x == c && b == d)
        )
    )
}

fn is_antisymmetric<T: PartialEq>(pairs: &[(T, T)]) -> bool {
    pairs.iter().all(|(x, y)| x == y || !pairs.iter().any(|(a, b)| y == a && x == b))
}

fn is_equivalence<T: PartialEq + Clone>(pairs: &[(T, T)], set: &[T]) -> bool {
    is_reflexive(pairs, set) && is_symmetric(pairs) && is_transitive(pairs)
}

fn main() {
    println!("Enter pairs as tuples (x, y), one per line. Enter an empty line to finish:");
    let mut pairs = Vec::new();
    let mut input = String::new();

    while let Ok(_) = io::stdin().read_line(&mut input) {
        let trimmed_input = input.trim();
        if trimmed_input.is_empty() {
            break;
        }
        let pair: Result<Vec<i32>, _> = trimmed_input.split(',')
                                                     .map(|s| s.trim().parse::<i32>())
                                                     .collect();
        match pair {
            Ok(p) if p.len() == 2 => pairs.push((p[0], p[1])),
            _ => println!("Invalid input, please enter pairs as two comma-separated numbers."),
        }
        input.clear();
    }

    // Generate the set of elements from the pairs to ensure reflexivity can be checked.
    let set: Vec<i32> = pairs.iter().flat_map(|&(x, y)| vec![x, y]).collect::<Vec<_>>().into_iter().collect();

    println!("Is reflexive: {}", is_reflexive(&pairs, &set));
    println!("Is symmetric: {}", is_symmetric(&pairs));
    println!("Is transitive: {}", is_transitive(&pairs));
    println!("Is antisymmetric: {}", is_antisymmetric(&pairs));
    println!("Is equivalence: {}", is_equivalence(&pairs, &set));
}
