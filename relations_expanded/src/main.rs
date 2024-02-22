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
    let set1 = vec!['a', 'b', 'c', 'd', 'e'];
    let pairs1 = vec![('a', 'b'), ('a', 'e'), ('b', 'c'), ('c', 'd'), ('d', 'f')];

    let set2 = vec![1, 3, 5, 7, 8, 12, 13];
    let pairs2 = vec![(1, 8), (1, 12), (3, 8), (3, 12), (5, 8), (5, 12), (7, 8), (7, 12)];

    let set3 = vec!["()", "!", "*", "+", "<<", "<", "&"];
    let pairs3 = vec![("+", "*"), ("*", "()"), ("*", "!"), ("+", "<<"), ("<<", "<"), ("<<", "&")];

    let set4 = vec!['m', 'd', 'b', 's', 'g', 'f'];
    let pairs4 = vec![('m', 'b'), ('m', 's'), ('d', 'b'), ('d', 's'), ('g', 'm'), ('f', 'm')];

    println!("Set 1:");
    println!("Elements:");
    for elem in &set1 {
        println!("{}", elem);
    }
    println!("Pairs:");
    for pair in &pairs1 {
        println!("{:?}", pair);
    }
    println!("Set 1 properties:");
    println!("Is reflexive: {}", is_reflexive(&pairs1, &set1));
    println!("Is symmetric: {}", is_symmetric(&pairs1));
    println!("Is transitive: {}", is_transitive(&pairs1));
    println!("Is antisymmetric: {}", is_antisymmetric(&pairs1));
    println!("Is equivalence: {}", is_equivalence(&pairs1, &set1));
    println!();

    println!("Set 2:");
    println!("Elements:");
    for elem in &set2 {
        println!("{}", elem);
    }
    println!("Pairs:");
    for pair in &pairs2 {
        println!("{:?}", pair);
    }

    println!("Set 2 properties:");
    println!("Is reflexive: {}", is_reflexive(&pairs2, &set2));
    println!("Is symmetric: {}", is_symmetric(&pairs2));
    println!("Is transitive: {}", is_transitive(&pairs2));
    println!("Is antisymmetric: {}", is_antisymmetric(&pairs2));
    println!("Is equivalence: {}", is_equivalence(&pairs2, &set2));
    println!();

    println!("Set 3:");
    println!("Elements:");
    for elem in &set3 {
        println!("{}", elem);
    }
    println!("Pairs:");
    for pair in &pairs3 {
        println!("{:?}", pair);
    }

    println!("Set 3 properties:");
    println!("Is reflexive: {}", is_reflexive(&pairs3, &set3));
    println!("Is symmetric: {}", is_symmetric(&pairs3));
    println!("Is transitive: {}", is_transitive(&pairs3));
    println!("Is antisymmetric: {}", is_antisymmetric(&pairs3));
    println!("Is equivalence: {}", is_equivalence(&pairs3, &set3));
    println!();

    println!("Set 4:");
    println!("Elements:");
    for elem in &set4 {
        println!("{}", elem);
    }
    println!("Pairs:");
    for pair in &pairs4 {
        println!("{:?}", pair);
    }

    println!("Set 4 properties:");
    println!("Is reflexive: {}", is_reflexive(&pairs4, &set4));
    println!("Is symmetric: {}", is_symmetric(&pairs4));
    println!("Is transitive: {}", is_transitive(&pairs4));
    println!("Is antisymmetric: {}", is_antisymmetric(&pairs4));
    println!("Is equivalence: {}", is_equivalence(&pairs4, &set4));

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
